use crate::circuit;
use crate::sensor;
use crate::sensor::motor_controller;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use sensor::motor_controller::{Motor, MotorControllerMessage, Movement};
use sensor::{photo_resistor::PhotoResistorSubscriber, servo::ServoPublisher};

#[derive(Clone, Debug)]
pub struct Capture {
    id: u32,
    state: CircuitState,
    servo: ServoPublisher,
    photo_resistor: PhotoResistorSubscriber,
    trigger: i32,
}

#[async_trait]
impl Circuit for Capture {
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
        }
    }

    fn process_ready(&mut self) {
        self.state = CircuitState::Running;
    }

    async fn process_running(&mut self) {
        let value = self.photo_resistor.get().await;
        let value = match value {
            Ok(v) => v,
            Err(_) => return,
        };
        if value < self.trigger {
            self.servo.set(90).await;
            tokio::time::sleep(std::time::Duration::from_millis(3000));
            self.servo.set(0).await;
            tokio::time::sleep(std::time::Duration::from_millis(1500));
            self.state = CircuitState::Waiting;
        }
    }

    fn process_waiting(&mut self) {}
}
