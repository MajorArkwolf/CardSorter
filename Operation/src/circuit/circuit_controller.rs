use std::sync::Arc;

use super::{Circuit, CircuitState};
use color_eyre::eyre::{eyre, Context, ContextCompat, Error, Result};
use tokio::sync::Mutex;

pub struct CircuitController {
    circuit: Vec<Arc<Mutex<Box<dyn Circuit>>>>,
    active: usize,
    jobs: Vec<tokio::task::JoinHandle<Result<()>>>,
}

impl CircuitController {
    pub fn create(circuit: Vec<Arc<Mutex<Box<dyn Circuit>>>>) -> Self {
        Self {
            circuit,
            active: 0,
            jobs: vec![],
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        {
            let item = self.circuit[0].lock().await;
            item.change_state(CircuitState::Ready);
        }
        for c in self.circuit {
            let d = c.clone();
            let job = tokio::task::spawn(async move {
                let cir = d.lock().await;
                cir.update();
                Ok::<(), Error>(())
            });
            self.jobs.push(job);
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        let active_circuit = self.circuit[self.active].lock().await;
        let curr_state = active_circuit.get_state().await;
        if curr_state == CircuitState::Waiting {
            self.active += 1;
            if self.active >= self.circuit.len() {
                self.active = 0;
            }
            let mut active_circuit = self.circuit[self.active].lock().await;
            active_circuit.change_state(CircuitState::Ready);
        }

        Ok(())
    }
}
