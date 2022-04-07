use crate::circuit;
use async_trait::async_trait;
use circuit::Circuit;
use color_eyre::eyre::{eyre, Result};
use std::sync::Arc;
use tokio::sync::Notify;
use tracing::{debug};
use circuit::State;

#[derive(Debug, Copy, Clone, PartialEq)]
enum InternalState {
    Waiting,
    GetCard,
    ScanCard,
    MoveCardOn,
    Stopped,
}

#[derive(Debug)]
pub struct Capture {
    id: u32,
    external_state: State,
    start_trigger: Arc<Notify>,
    end_trigger: Arc<Notify>,
    internal_state: InternalState,
}

impl Capture {
    pub fn create(
        id: u32,
        external_state: State,
        start_trigger: Arc<Notify>,
        end_trigger: Arc<Notify>,
    ) -> Self {
        Self {
            id,
            external_state,
            start_trigger,
            end_trigger,
            internal_state: InternalState::Waiting,
        }
    }
}

#[async_trait]
impl Circuit for Capture {
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
        self.internal_state = InternalState::Stopped;
        Ok(())
    }

    async fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        debug!("capture {} has begun its run process.", self.get_id());
        match self.internal_state {
            InternalState::Waiting => {
                if self.external_state == State::Running {
                    self.start_trigger.notified().await;
                    debug!("capture {} has recieved its notification to begin.", self.get_id());
                    self.internal_state = InternalState::GetCard;
                } else if self.external_state == State::Ending {
                    debug!("capture {} has succesfully resolved an ending state transition.", self.get_id());
                    /*
                    We change the external state to waiting once we are back
                    at the beginning since a ending is recoverable and
                    acts as a notification back out to the task above.
                    */
                    self.external_state = State::Waiting;
                }
            }
            InternalState::GetCard => {
                // move card into position
                self.internal_state = InternalState::ScanCard;
            }
            InternalState::ScanCard => {
                // scan card and send to OCR
                self.internal_state = InternalState::MoveCardOn;
            }
            InternalState::MoveCardOn => {
                // Make sure distributor has card.
                self.internal_state = InternalState::Waiting;
                self.end_trigger.notify_one();
            }
            InternalState::Stopped => {
                return Err(eyre!(
                    "capture {} circuit was meant to be stopped and should not have reached here."
                , self.get_id()
                ))
            }
        }
        Ok(())
    }
}
