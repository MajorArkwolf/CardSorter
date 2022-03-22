use crate::circuit;
use crate::sensor;
use async_trait::async_trait;
use circuit::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Result};
use sensor::motor_controller::{Motor, Movement};
use sensor::{motor_controller::MotorController, photo_resistor::PhotoResistor, servo::Servo};
use tracing::debug;
use tracing::{info, instrument};

pub struct Knocker {
    id: u32,
    servos: Servo,
    trigger: u16,
}

impl CircuitState {
    async fn progress_circuit() {
        loop {
            // wait pre notification before starting.
            // read message from capture on where this will go
            // alter the servo based on pile output unless last pile
            // run motor until card leaves the circuit (how do I know when it leaves the circuit)
            // Notify next stage
            // sleep for servo to move to position
            // loop until we get the correct value
        }
    }

    async fn start(&mut self) -> Result<()> {
        // wait for notification.
        Ok(())
    }
}

#[async_trait]
impl Circuit for CircuitState {
    async fn get_id(&self) -> u32 {
        self.id
    }

    async fn stop(&mut self) -> Result<()> {

    }

    async fn setup(&mut self) -> Result<()> {

    }

    async fn run(&mut self) -> Result<()> {
        tokio::select! {
            _ = self.stop() => {}
            _ = self.start() => {
                self.progress_circuit().await?
            }
        }
        Ok(())
    }
}
