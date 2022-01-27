use crate::circuit;
use crate::network::{CardData, Network, PictureFormat, Request};
use crate::sensor;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Result};
use sensor::{photo_resistor::PhotoResistor, servo::Servo};
use tokio::net::ToSocketAddrs;
use tracing::instrument;
use tracing::{debug, error, info};

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
    network: Network,
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

    async fn stop(&mut self) -> Result<()> {
        self.network.send(Request::EndConnection).await?;
        self.state = CircuitState::Stopped;
        Ok(())
    }
}

impl Capture {
    pub async fn create<A: ToSocketAddrs>(
        id: u32,
        state: CircuitState,
        servo: Servo,
        photo_resistor: PhotoResistor,
        trigger: u16,
        address: A,
    ) -> Result<Self> {
        let network = Network::connect(address).await?;
        Ok(Self {
            id,
            state,
            servo,
            photo_resistor,
            trigger,
            internal_state: CaptureStates::TakePicture,
            network,
        })
    }

    fn process_ready(&mut self) -> Result<()> {
        self.state = CircuitState::Running;
        self.internal_state = CaptureStates::TakePicture;
        info!("capture system started");
        Ok(())
    }

    async fn process_running(&mut self) -> Result<()> {
        // Take Picture
        match self.internal_state {
            CaptureStates::TakePicture => {
                self.internal_state = CaptureStates::RunOCR;
            }
            CaptureStates::RunOCR => {
                let contents: Vec<u8> = vec![];
                let card = Request::CardData(CardData {
                    type_of: PictureFormat::TakePicture,
                    data: contents,
                });
                self.network.send(card).await?;
                let resp = self.network.recv().await?;
                if resp.error == 0 {
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
                self.servo.set(90).await?;
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                self.internal_state = CaptureStates::Finished;
            }
            CaptureStates::Finished => {
                let value = self.photo_resistor.get()?;
                debug!("Photoresistor: {}", value);
                if value > self.trigger {
                    self.servo.set(0).await?;
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
