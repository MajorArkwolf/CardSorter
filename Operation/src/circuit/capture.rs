use std::io::Read;

use crate::circuit;
use crate::network::{CardData, Network, PictureFormat, Request};
use crate::sensor;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Context, Error, Result, WrapErr};
use color_eyre::Report;
use sensor::{photo_resistor::PhotoResistorSubscriber, servo::ServoPublisher};
use tracing::{error, info};
use tracing::{event, instrument, Level};

#[derive(Clone, Copy, Debug, PartialEq)]
enum CaptureStates {
    TakePicture,
    RunOCR,
    ReleaseCard,
    Finished,
}

#[derive(Debug)]
pub struct Capture {
    id: u32,
    state: CircuitState,
    servo: ServoPublisher,
    photo_resistor: PhotoResistorSubscriber,
    trigger: i32,
    internal_state: CaptureStates,
}

#[async_trait]
impl Circuit for Capture {
    async fn get_id(&self) -> u32 {
        self.id
    }

    #[instrument]
    async fn get_state(&self) -> CircuitState {
        self.state
    }

    #[instrument]
    async fn change_state(&mut self, next_state: CircuitState) -> Result<()> {
        if self.state == CircuitState::Stopped {
            if next_state != CircuitState::Stopped {
                return Err(eyre!("tried to set from stopped"));
            } else {
                return Ok(());
            }
        }
        self.state = next_state;
        Ok(())
    }

    #[instrument]
    async fn update(&mut self) {
        match self.state {
            CircuitState::Ready => self.process_ready(),
            CircuitState::Running => self.process_running().await,
            CircuitState::Waiting => self.process_waiting(),
            CircuitState::Stopped => return,
        }
    }

    async fn stop(&mut self) {}
}

impl Capture {
    pub fn create(
        id: u32,
        state: CircuitState,
        servo: ServoPublisher,
        photo_resistor: PhotoResistorSubscriber,
        trigger: i32,
    ) -> Self {
        Self {
            id,
            state,
            servo,
            photo_resistor,
            trigger,
            internal_state: CaptureStates::TakePicture,
        }
    }

    #[instrument(skip_all)]
    fn process_ready(&mut self) {
        self.state = CircuitState::Running;
        self.internal_state = CaptureStates::TakePicture;
        info!("capture system started");
    }

    #[instrument(skip_all)]
    async fn process_running(&mut self) {
        // Take Picture
        match self.internal_state {
            CaptureStates::TakePicture => {
                self.internal_state = CaptureStates::RunOCR;
            }
            CaptureStates::RunOCR => {
                let contents: Vec<u8> = vec![];
                let network_ocr = Network::connect("127.0.0.1:10000").await;
                match network_ocr {
                    Ok(mut network) => {
                        let card = Request::CardData(CardData {
                            type_of: PictureFormat::TakePicture,
                            data: contents,
                        });
                        network.send(card).await.unwrap();
                        let resp = network.recv().await.unwrap();
                        if resp.error == 0 {
                            network.send(Request::EndConnection).await.unwrap();
                            self.internal_state = CaptureStates::ReleaseCard;
                        } else {
                            error!("OCR network returned {}, stopping circuit", resp.error);
                            self.state = CircuitState::Stopped;
                            return;
                        }
                    }
                    Err(_) => {
                        self.state = CircuitState::Stopped;
                        self.internal_state = CaptureStates::Finished;
                        return;
                    }
                }
                self.internal_state = CaptureStates::ReleaseCard;
            }
            CaptureStates::ReleaseCard => {
                self.internal_state = CaptureStates::Finished;
                info!("capture system dispending card");
                let result = self.servo.set(90).await;
                self.handle_result(result);
                tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
            }
            CaptureStates::Finished => {
                let value = self.photo_resistor.get().await;
                let value = match value {
                    Ok(v) => v,
                    Err(_) => return,
                };
                if value > self.trigger {
                    let result = self.servo.set(0).await;
                    self.handle_result(result);
                    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                    self.state = CircuitState::Waiting;
                    info!("capture system moving to waiting");
                }
            }
        }
    }

    fn process_waiting(&mut self) {}

    #[instrument(skip_all)]
    fn handle_result(&mut self, result: Result<(), Report>) {
        match result {
            Ok(_) => {}
            Err(_) => {
                error!("error discovered, stopping self {:?}", self);
                self.state = CircuitState::Stopped
            }
        }
    }

    fn process_error(&mut self, error: Error) {
        error!("Error occured in capture: {}", error);
        self.state = CircuitState::Stopped;
    }
}
