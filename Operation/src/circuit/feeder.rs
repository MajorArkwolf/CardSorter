use std::sync::Arc;

use crate::circuit;
use crate::sensor;
use async_trait::async_trait;
use circuit::Circuit;
use color_eyre::eyre::{eyre, Result, WrapErr};
use sensor::motor_controller::{Motor, Movement};
use sensor::{motor_controller::MotorController, photo_resistor::PhotoResistor};
use tokio::sync::watch;
use tokio::sync::Notify;
use tracing::debug;
use tracing::{info, instrument};

use circuit::State;

#[derive(Debug, Copy, Clone, PartialEq)]
enum InternalState {
    Waiting,
    MotorOn,
    WaitingForTrigger,
    Stopped,
}

pub struct Feeder {
    id: u32,
    external_state: State,
    start_trigger: Arc<Notify>,
    end_trigger: Arc<Notify>,
    motor_cont: MotorController,
    internal_state: InternalState,
}

impl Feeder {
    pub fn create(
        id: u32,
        external_state: State,
        start_trigger: Arc<Notify>,
        end_trigger: Arc<Notify>,
        motor_cont: MotorController,
    ) -> Self {
        Self {
            id,
            external_state,
            start_trigger,
            end_trigger,
            motor_cont,
            internal_state: InternalState::Waiting,
        }
    }
}

#[async_trait]
impl Circuit for Feeder {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_state(&self) -> State {
        self.external_state
    }

    fn set_state(&mut self, state: State) {
        self.external_state = state;
    }

    async fn stop(&mut self) -> Result<()> {
        self.motor_cont.set(Motor::A, Movement::Stop).await?;
        self.internal_state = InternalState::Stopped;
        Ok(())
    }

    async fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        match self.internal_state {
            InternalState::Waiting => {
                if self.external_state == State::Running {
                    self.start_trigger.notified().await;
                    self.internal_state = InternalState::MotorOn;
                    debug!("Feeder circuit has been notified");
                } else if self.external_state == State::Ending {
                    /*
                    We change the external state to waiting once we are back
                    at the beginning since a ending is recoverable and
                    acts as a notification back out to the task above.
                    */
                    self.external_state = State::Waiting;
                }
            }
            InternalState::MotorOn => {
                debug!("Feeder circuit turning motor on");
                self.motor_cont.set(Motor::A, Movement::Forward).await?;
                self.internal_state = InternalState::WaitingForTrigger;
                // Artifical sleep for testing.
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
            InternalState::WaitingForTrigger => {
                // add stop trigger
                debug!("Feeder circuit turning motor off");
                self.motor_cont.set(Motor::A, Movement::Stop).await?;
                self.internal_state = InternalState::Waiting;
                self.end_trigger.notify_one();
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
            InternalState::Stopped => {
                debug!("Feeder circuit stopping");
                return Err(eyre!(
                    "progress circuit was meant to be stopped and should not have reached here."
                ));
            }
        }
        Ok(())
    }
}
