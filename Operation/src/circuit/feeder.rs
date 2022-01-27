use crate::circuit;
use crate::sensor;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Result};
use sensor::motor_controller::{Motor, Movement};
use sensor::{motor_controller::MotorController, photo_resistor::PhotoResistor, servo::Servo};
use tracing::debug;
use tracing::{info, instrument};

#[derive(Clone, Debug)]
pub struct Feeder {
    id: u32,
    state: CircuitState,
    motor_cont: MotorController,
    photo_resistor: PhotoResistor,
    servo: Servo,
    trigger: u16,
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
    async fn update(&mut self) -> Result<()> {
        match self.state {
            CircuitState::Ready => self.process_ready().await,
            CircuitState::Running => self.process_running().await,
            CircuitState::Waiting => self.process_waiting(),
            CircuitState::Stopped => Ok(()),
        }
    }
    async fn stop(&mut self) -> Result<()> {
        self.state = CircuitState::Stopped;
        Ok(())
    }
}

impl Feeder {
    pub fn create(
        id: u32,
        state: CircuitState,
        motor_cont: MotorController,
        photo_resistor: PhotoResistor,
        servo: Servo,
        trigger: u16,
    ) -> Self {
        Self {
            id,
            state,
            motor_cont,
            photo_resistor,
            servo,
            trigger,
        }
    }

    #[instrument(skip_all)]
    async fn process_ready(&mut self) -> Result<()> {
        self.servo.set(0).await?;
        self.motor_cont.set(Motor::A, Movement::Forward).await?;
        self.state = CircuitState::Running;
        Ok(())
    }

    #[instrument(skip_all)]
    async fn process_running(&mut self) -> Result<()> {
        let value = self.photo_resistor.get()?;
        debug!("Feeder running Value: {}, Trigger: {}", value, self.trigger);
        if value >= self.trigger || value == 0 {
            return Ok(());
        }
        info!(
            "trigger `{}` value `{}` hit, moving to waiting",
            self.trigger, value
        );
        self.motor_cont.set(Motor::A, Movement::Stop).await?;

        self.state = CircuitState::Waiting;
        Ok(())
    }

    #[instrument(skip_all)]
    fn process_waiting(&mut self) -> Result<()> {
        Ok(())
    }
}
