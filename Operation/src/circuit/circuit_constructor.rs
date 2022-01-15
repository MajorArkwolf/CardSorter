use std::sync::Arc;
use std::vec;

use super::circuit_controller::CircuitController;
use super::Circuit;
use super::CircuitState;
use crate::board;
use crate::circuit;
use board::BoardContainer;
use circuit::{capture::Capture, feeder::Feeder};
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use firmata::Firmata;
use tokio::sync::Mutex;

struct CalibrationResult {
    sensor_id: u32,
    sensor_calibration_value: i32,
}

fn calibrate_photo_resistor(
    boards: &mut BoardContainer,
    sensor_id: u32,
) -> Result<CalibrationResult> {
    let photo_pin = boards
        .get_sensor(sensor_id)?
        .as_photo_resistor_mut()
        .unwrap()
        .get_pin_id();

    let mut unwrapped_board = {
        let board = boards.get_board_from_sensor_id(sensor_id)?;

        match board.get_board() {
            board::BoardTypes::SerialBoard(v) => v.board.blocking_lock(),
        }
    };

    //turn light off

    let mut off_average = 0;
    unwrapped_board.poll(5)?;
    for _i in 1..20 {
        unwrapped_board.poll(5)?;
        let pin = unwrapped_board.get_physical_pin(photo_pin)?;
        off_average += pin.value;
    }
    off_average /= 20;

    //turn light on
    unwrapped_board.poll(5)?;
    let mut on_average = 0;
    for _i in 1..20 {
        unwrapped_board.poll(5)?;
        let pin = unwrapped_board.get_physical_pin(photo_pin)?;
        off_average += pin.value;
    }
    on_average /= 20;

    if off_average > on_average {
        return Err(eyre!(
            "off average `{}` was larger then on average `{}`",
            off_average,
            on_average
        ));
    }

    let half_way_point: i32 = ((on_average - off_average) / 2) + off_average;

    if half_way_point > on_average {
        return Err(eyre!(
            "mid point `{}` was larger then on max average `{}`",
            half_way_point,
            on_average
        ));
    }

    Ok(CalibrationResult {
        sensor_id,
        sensor_calibration_value: half_way_point,
    })
}

fn calibrate_sensors(boards: &mut BoardContainer) -> Result<Vec<CalibrationResult>> {
    let calibration_results: Vec<CalibrationResult> = vec![calibrate_photo_resistor(boards, 2)?];
    Ok(calibration_results)
}

fn construct_feeder(
    boards: &mut BoardContainer,
    calibration_results: &[CalibrationResult],
) -> Result<Feeder> {
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
    let servo = boards.get_sensor(1)?.as_servo_mut().unwrap().publisher();

    let value = calibration_results
        .iter()
        .find(|x| x.sensor_id == 2)
        .ok_or_else(|| eyre!("failed to find a calibration result that matches the id"))?
        .sensor_calibration_value;

    Ok(Feeder::create(
        0,
        CircuitState::Waiting,
        motor_cont,
        photo,
        servo,
        value,
    ))
}

fn construct_capture(
    boards: &mut BoardContainer,
    calibration_results: &[CalibrationResult],
) -> Result<Capture> {
    let servo = boards.get_sensor(1)?.as_servo_mut().unwrap().publisher();
    let photo = boards
        .get_sensor(2)?
        .as_photo_resistor_mut()
        .unwrap()
        .subscribe();

    let value = calibration_results
        .iter()
        .find(|x| x.sensor_id == 2)
        .ok_or_else(|| eyre!("failed to find a calibration result that matches the id"))?
        .sensor_calibration_value;

    Ok(Capture::create(
        1,
        CircuitState::Waiting,
        servo,
        photo,
        value,
    ))
}

pub fn construct_circuit(boards: &mut BoardContainer) -> Result<CircuitController> {
    let calibration_results = calibrate_sensors(boards)?;
    let circuits: Vec<Arc<Mutex<Box<dyn Circuit + Send>>>> = vec![
        Arc::new(Mutex::new(Box::new(construct_feeder(
            boards,
            &calibration_results,
        )?))),
        Arc::new(Mutex::new(Box::new(construct_capture(
            boards,
            &calibration_results,
        )?))),
    ];
    Ok(CircuitController::create(circuits))
}
