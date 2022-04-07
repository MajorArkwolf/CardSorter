use crate::{
    backbone::{message::OverseerChannel},
    circuit::{feeder::Feeder, capture::Capture},
    factory::System,
    sensor::{motor_controller::MotorController},
};
use std::sync::Arc;
use tracing::{info, debug};
use crate::circuit::{Circuit, circuitwatcher::CircuitWatcher, State};
use color_eyre::eyre::{eyre, Result};
use tokio::{
    sync::{watch, Notify},
    task::JoinHandle,
};

pub fn start_task(
    circuit: Box<dyn Circuit + Send>,
    external_state: watch::Receiver<State>,
) -> JoinHandle<Result<()>> {
    tokio::task::spawn(async move {
        let mut circuit = circuit;

        loop {
            let current_state = *external_state.borrow();
            circuit.set_state(current_state);
            let result: Result<()> = match current_state {
                State::Waiting => Ok(()),
                State::Stop => {
                    circuit.stop().await?;
                    break;
                }
                State::Running => circuit.run().await,
                State::Ending => {
                    circuit.run().await?;
                    if circuit.get_state() == State::Waiting {
                        debug!("Circuit {} has succesfully ended.", circuit.get_id());
                        break;
                    }
                    Ok(())
                }
            };

            match result {
                Ok(_) => {}
                Err(e) => {
                    return Err(eyre!(
                        "Circuit {0} has failed with the following error: {1}",
                        circuit.get_id(),
                        e
                    ));
                }
            }
        }

        Ok(())
    })
}

pub async fn generate_feeder(
    id: u32,
    system: &mut System,
    start_trigger: Arc<Notify>,
    end_trigger: Arc<Notify>,
) -> Result<Box<dyn Circuit + Send>> {
    let motor_cont = MotorController::create(
        1,
        [7, 18],
        [6, 5, 16, 17],
        system.board_tasks[0].board().clone(),
    )
    .await?;

    Ok(Box::new(Feeder::create(
        id,
        State::Waiting,
        start_trigger,
        end_trigger,
        motor_cont,
    )))
}

pub async fn generate_capture(
    id: u32,
    system: &mut System,
    start_trigger: Arc<Notify>,
    end_trigger: Arc<Notify>,
) -> Result<Box<dyn Circuit + Send>> {

    Ok(Box::new(Capture::create(
        id,
        State::Waiting,
        start_trigger,
        end_trigger,
    )))
}

pub async fn generat_distributor() -> Result<()> {
    Ok(())
}

pub async fn generate_circuits(
    overseer_channel: OverseerChannel,
    system: &mut System,
) -> Result<CircuitWatcher> {
    let (tx, rx) = watch::channel(State::Waiting);
    let mut watcher = CircuitWatcher::create(tx, overseer_channel);
    let end_notifacation = Arc::new(Notify::new());
    end_notifacation.notify_one();
    let feeder_end = Arc::new(Notify::new());
    let feeder = generate_feeder(1, system, end_notifacation.clone(), feeder_end.clone()).await?;
    watcher.add_join_handle(start_task(feeder, rx.clone()));
    let capture = generate_feeder(2, system, feeder_end, end_notifacation).await?;
    watcher.add_join_handle(start_task(capture, rx.clone()));
    // list of circuits to construct

    // go through each item and try and construct each circuit

    // clone overseer channels

    // move circuit into task and append to list of circuits
    info!("Circuit generation complete");
    Ok(watcher)
}
