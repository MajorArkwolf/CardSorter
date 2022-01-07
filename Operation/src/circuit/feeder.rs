use crate::circuit;
use crate::sensor;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Result};
use sensor::motor_controller::{Motor, MotorControllerMessage, Movement};
use sensor::{motor_controller::MotorControllerPublisher, photo_resistor::PhotoResistorSubscriber};
use tracing::debug;
use tracing::info;
use tracing::{event, instrument, Level};

#[derive(Clone, Debug)]
pub struct Feeder {
    id: u32,
    state: CircuitState,
    motor_cont: MotorControllerPublisher,
    photo_resistor: PhotoResistorSubscriber,
    trigger: i32,
}

#[async_trait]
impl Circuit for Feeder {
    #[instrument(skip_all)]
    async fn get_id(&self) -> u32 {
        self.id
    }

    #[instrument(skip_all)]
    async fn get_state(&self) -> CircuitState {
        self.state
    }

    #[instrument]
    async fn change_state(&mut self, next_state: CircuitState) -> Result<()> {
        if self.state == CircuitState::Stopped {
            return Err(eyre!("tried to set from stopped"));
        }
        self.state = next_state;
        Ok(())
    }

    #[instrument(skip_all)]
    async fn update(&mut self) {
        match self.state {
            CircuitState::Ready => self.process_ready().await,
            CircuitState::Running => self.process_running().await,
            CircuitState::Waiting => self.process_waiting(),
            CircuitState::Stopped => return,
        }
    }
    async fn stop(&mut self) {}
}

impl Feeder {
    pub fn create(
        id: u32,
        state: CircuitState,
        motor_cont: MotorControllerPublisher,
        photo_resistor: PhotoResistorSubscriber,
        trigger: i32,
    ) -> Self {
        Self {
            id,
            state,
            motor_cont,
            photo_resistor,
            trigger,
        }
    }

    #[instrument(skip_all)]
    async fn process_ready(&mut self) {
        let result = self
            .motor_cont
            .set(MotorControllerMessage::create(Motor::A, Movement::Forward))
            .await;
        match result {
            Ok(_) => self.state = CircuitState::Running,
            Err(_) => self.state = CircuitState::Stopped,
        }
    }

    #[instrument(skip_all)]
    async fn process_running(&mut self) {
        let value = self.photo_resistor.get().await;
        let value = match value {
            Ok(v) => v,
            Err(_) => return,
        };
        if value < self.trigger {
            info!(
                "trigger `{}` value `{}` hit, moving to waiting",
                self.trigger, value
            );
            let result = self
                .motor_cont
                .set(MotorControllerMessage::create(Motor::A, Movement::Stop))
                .await;

            match result {
                Ok(_) => self.state = CircuitState::Waiting,
                Err(_) => self.state = CircuitState::Stopped,
            }
        }
    }

    #[instrument(skip_all)]
    fn process_waiting(&mut self) {}
}
