use std::sync::Arc;

use crate::circuit;
use crate::sensor;
use async_trait::async_trait;
use circuit::Circuit;
use color_eyre::eyre::{eyre, Result};
use sensor::motor_controller::{Motor, Movement};
use sensor::{motor_controller::MotorController};
use tokio::sync::Notify;
use tracing::debug;
use circuit::State;

#[derive(Debug, Copy, Clone, PartialEq)]
enum InternalState {
    Waiting,
    Processing,
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
        debug!("Feeder {} has called stop.", self.get_id());
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
                self.start_trigger.notified().await;
                debug!("Feeder {} circuit has been notified", self.get_id());
                self.internal_state = InternalState::Processing;
            }
            InternalState::Processing => {                
                if self.external_state == State::Running {
                    debug!("Feeder {} circuit has been transitioned to motor on", self.get_id());
                    self.internal_state = InternalState::MotorOn;
                } else if self.external_state == State::Ending {
                    /*
                    We change the external state to waiting once we are back
                    at the beginning since a ending is recoverable and
                    acts as a notification back out to the task above.
                    */
                    self.end_trigger.notify_one();
                    debug!("Feeder {} circuit has been ended succesfully", self.get_id());
                    self.external_state = State::Waiting;
                }
            }
            InternalState::MotorOn => {
                debug!("Feeder {} circuit turning motor on", self.get_id());
                self.motor_cont.set(Motor::A, Movement::Forward).await?;
                self.internal_state = InternalState::WaitingForTrigger;
                // Artifical sleep for testing.
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
            InternalState::WaitingForTrigger => {
                // add stop trigger
                debug!("Feeder {} circuit turning motor off", self.get_id());
                self.motor_cont.set(Motor::A, Movement::Stop).await?;
                self.internal_state = InternalState::Waiting;
                self.end_trigger.notify_one();
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
            InternalState::Stopped => {
                debug!("Feeder {} circuit stopping", self.get_id());
                return Err(eyre!(
                    "progress circuit was meant to be stopped and should not have reached here."
                ));
            }
        }
        Ok(())
    }
}
