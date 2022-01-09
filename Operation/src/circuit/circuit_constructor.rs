use std::sync::Arc;
use std::vec;

use crate::board;
use crate::circuit;
use board::BoardContainer;
use circuit::{capture::Capture, feeder::Feeder};
use color_eyre::eyre::Result;
use tokio::sync::Mutex;

use super::circuit_controller::CircuitController;
use super::Circuit;
use super::CircuitState;

pub fn construct_feeder(boards: &mut BoardContainer) -> Result<Feeder> {
    let motor_cont = boards
        .get_sensor(3)?
        .as_motor_controller_mut()
        .unwrap()
        .publisher();
    let photo = boards
        .get_sensor(2)?
        .as_photo_resistor_mut()
        .unwrap()
        .subscribe();
    Ok(Feeder::create(
        0,
        CircuitState::Waiting,
        motor_cont,
        photo,
        500,
    ))
}

pub fn construct_capture(boards: &mut BoardContainer) -> Result<Capture> {
    let servo = boards.get_sensor(1)?.as_servo_mut().unwrap().publisher();
    let photo = boards
        .get_sensor(2)?
        .as_photo_resistor_mut()
        .unwrap()
        .subscribe();
    Ok(Capture::create(1, CircuitState::Waiting, servo, photo, 500))
}

pub fn construct_circuit(boards: &mut BoardContainer) -> Result<CircuitController> {
    let circuits: Vec<Arc<Mutex<Box<dyn Circuit + Send>>>> = vec![
        Arc::new(Mutex::new(Box::new(construct_feeder(boards)?))),
        Arc::new(Mutex::new(Box::new(construct_capture(boards)?))),
    ];
    Ok(CircuitController::create(circuits))
}