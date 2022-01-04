use crate::circuit;
use crate::sensor;
use crate::sensor::motor_controller;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use sensor::motor_controller::{Motor, MotorControllerMessage, Movement};
use sensor::{motor_controller::MotorControllerPublisher, photo_resistor::PhotoResistorSubscriber};

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
    async fn get_id(&self) -> u32 {
        self.id
    }
    async fn get_state(&self) -> CircuitState {
        self.state
    }
    async fn change_state(&mut self, next_state: CircuitState) -> Result<()> {
        if self.state == CircuitState::Stopped {
            return Err(eyre!("tried to set from stopped"));
        }
        self.state = next_state;
        Ok(())
    }
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

    async fn process_ready(&mut self) {
        self.motor_cont
            .set(MotorControllerMessage::create(Motor::A, Movement::Forward))
            .await;

        self.state = CircuitState::Running;
    }

    async fn process_running(&mut self) {
        let value = self.photo_resistor.get().await;
        let value = match value {
            Ok(v) => v,
            Err(_) => return,
        };
        if value > self.trigger {
            self.motor_cont
                .set(MotorControllerMessage::create(Motor::A, Movement::Stop))
                .await;
            self.state == CircuitState::Waiting;
        }
    }

    fn process_waiting(&mut self) {}
}
