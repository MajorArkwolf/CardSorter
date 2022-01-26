use std::io::Read;

use crate::circuit;
use crate::network::{CardData, Network, PictureFormat, Request};
use crate::sensor;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Context, Error, Result, WrapErr};
use color_eyre::Report;
use sensor::{photo_resistor::PhotoResistor, servo::Servo};
use tracing::{debug, error, info};
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
    servo: Servo,
    photo_resistor: PhotoResistor,
    trigger: u16,
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
    async fn update(&mut self) -> Result<()> {
        match self.state {
            CircuitState::Ready => self.process_ready(),
            CircuitState::Running => self.process_running().await,
            CircuitState::Waiting => self.process_waiting(),
            CircuitState::Stopped => Err(eyre!("capture system `{}` is stopped", self.id)),
        }
    }

    fn stop(&mut self) {
        self.state = CircuitState::Stopped;
    }
}

impl Capture {
    pub fn create(
        id: u32,
        state: CircuitState,
        servo: Servo,
        photo_resistor: PhotoResistor,
        trigger: u16,
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
    fn process_ready(&mut self) -> Result<()> {
        self.state = CircuitState::Running;
        self.internal_state = CaptureStates::TakePicture;
        info!("capture system started");
        Ok(())
    }

    #[instrument(skip_all)]
    async fn process_running(&mut self) -> Result<()> {
        // Take Picture
        match self.internal_state {
            CaptureStates::TakePicture => {
                self.internal_state = CaptureStates::RunOCR;
            }
            CaptureStates::RunOCR => {
                let contents: Vec<u8> = vec![];
                let mut network_ocr = Network::connect("127.0.0.1:10000").await?;
                let card = Request::CardData(CardData {
                    type_of: PictureFormat::TakePicture,
                    data: contents,
                });
                network_ocr.send(card).await?;
                let resp = network_ocr.recv().await?;
                if resp.error == 0 {
                    network_ocr.send(Request::EndConnection).await?;
                    self.internal_state = CaptureStates::ReleaseCard;
                } else {
                    let error_msg =
                        format!("OCR network returned {}, stopping circuit", resp.error);
                    error!("{}", error_msg);
                    self.state = CircuitState::Stopped;
                    return Err(eyre!(error_msg));
                }
                self.internal_state = CaptureStates::ReleaseCard;
            }
            CaptureStates::ReleaseCard => {
                info!("capture system releasing card");
                let result = self.servo.set(90).await;
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                self.internal_state = CaptureStates::Finished;
            }
            CaptureStates::Finished => {
                let value = self.photo_resistor.get()?;
                debug!("Photoresistor: {}", value);
                if value > self.trigger {
                    let result = self.servo.set(0).await;
                    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                    self.state = CircuitState::Waiting;
                    info!("capture system moving to waiting");
                }
            }
        }
        Ok(())
    }

    fn process_waiting(&mut self) -> Result<()> {
        Ok(())
    }
}
