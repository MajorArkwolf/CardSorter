use super::{Circuit, CircuitState};
use color_eyre::eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;
use tracing::{event, instrument, Level};

pub struct CircuitController {
    circuit: Vec<Arc<Mutex<Box<dyn Circuit + Send>>>>,
    active: usize,
    jobs: Vec<tokio::task::JoinHandle<()>>,
}

impl CircuitController {
    pub fn create(circuit: Vec<Arc<Mutex<Box<dyn Circuit + Send>>>>) -> Self {
        Self {
            circuit,
            active: 0,
            jobs: vec![],
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        {
            let mut item = self.circuit[0].lock().await;
            item.change_state(CircuitState::Ready).await?;
        }
        for c in self.circuit.iter() {
            let d = c.clone();
            let job = tokio::task::spawn(async move {
                loop {
                    let mut cir = d.lock().await;
                    cir.update().await;
                }
            });
            self.jobs.push(job);
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        let curr_state = {
            let circuit = self.circuit[self.active].lock().await;
            circuit.get_state().await
        };
        if curr_state == CircuitState::Waiting {
            self.active += 1;
            if self.active >= self.circuit.len() {
                self.active = 0;
            }
            let mut active_circuit = self.circuit[self.active].lock().await;
            active_circuit.change_state(CircuitState::Ready).await?;
        } else if curr_state == CircuitState::Stopped {
            error!("Circuit {} has reported a stop.", self.active);
        }
        Ok(())
    }
}
