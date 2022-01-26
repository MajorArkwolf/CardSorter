use std::sync::Arc;
use std::vec;

use super::circuit_controller::CircuitController;
use super::Circuit;
use super::CircuitState;
use crate::circuit;
use crate::factory::System;
use crate::sensor::led_strip::PixelColor;
use circuit::{capture::Capture, feeder::Feeder};
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use tokio::sync::Mutex;
use tracing::debug;

pub struct CalibrationResult {
    pub sensor_id: u32,
    pub sensor_calibration_value: u16,
}

async fn calibrate_photo_resistor(
    system: &mut System,
    sensor_id: u32,
    _light_sensor_id: u32,
) -> Result<CalibrationResult> {
    debug!("Beginning calibration of photo resistor");
    let photo_pin = &mut system.sensors.photo_resistor[0];
    let led_strip = &mut system.sensors.led_strips[0];

    let on = PixelColor::new(-1, 255, 255, 255);
    let off = PixelColor::new(-1, 0, 0, 0);

    debug!("Turning light off");
    //turn light off
    led_strip.set(off).await?;
    debug!("Off msg: {}", off.to_string());

    let mut off_average = 0;
    for _i in 1..20 {
        off_average += photo_pin.get()?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    off_average /= 20;

    debug!("Turning light on");
    //turn light on
    led_strip.set(on).await?;
    let mut on_average = 0;
    for _i in 1..20 {
        on_average += photo_pin.get()?;
    }
    on_average /= 20;

    debug!("Off average {}, On average {}", off_average, on_average);

    if off_average > on_average {
        return Err(eyre!(
            "off average `{}` was larger then on average `{}`",
            off_average,
            on_average
        ));
    }

    let half_way_point: u16 = ((on_average - off_average) / 2) + off_average;
    debug!("Half way point decided at: {}", half_way_point);

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

pub async fn calibrate_sensors(system: &mut System) -> Result<Vec<CalibrationResult>> {
    debug!("Beginning sensor calibration");
    let calibration_results: Vec<CalibrationResult> =
        vec![calibrate_photo_resistor(system, 2, 4).await?];
    Ok(calibration_results)
}

fn construct_feeder(
    system: &mut System,
    calibration_results: &[CalibrationResult],
) -> Result<Feeder> {
    let motor_cont = system
        .sensors
        .motor_controllers
        .iter()
        .find_map(|v| if *v.id() == 3 { Some(v) } else { None })
        .ok_or_else(|| eyre!("did not find motor controller with the given id"))?;
    let servo = system
        .sensors
        .servos
        .iter()
        .find_map(|v| if *v.id() == 1 { Some(v) } else { None })
        .ok_or_else(|| eyre!("did not find motor controller with the given id"))?;
    let photo = system
        .sensors
        .photo_resistor
        .iter()
        .find_map(|v| if *v.id() == 2 { Some(v) } else { None })
        .ok_or_else(|| eyre!("did not find photo resistor with the given id"))?;

    let value = calibration_results
        .iter()
        .find(|x| x.sensor_id == 2)
        .ok_or_else(|| eyre!("failed to find a calibration result that matches the id"))?
        .sensor_calibration_value;

    debug!("Calibration value used for feeder: {}", value);

    Ok(Feeder::create(
        0,
        CircuitState::Waiting,
        motor_cont.clone(),
        photo.clone(),
        servo.clone(),
        value,
    ))
}

fn construct_capture(
    system: &mut System,
    calibration_results: &[CalibrationResult],
) -> Result<Capture> {
    let servo = system
        .sensors
        .servos
        .iter()
        .find_map(|v| if *v.id() == 1 { Some(v) } else { None })
        .ok_or_else(|| eyre!("did not find motor controller with the given id"))?;
    let photo = system
        .sensors
        .photo_resistor
        .iter()
        .find_map(|v| if *v.id() == 2 { Some(v) } else { None })
        .ok_or_else(|| eyre!("did not find photo resistor with the given id"))?;

    let value = calibration_results
        .iter()
        .find(|x| x.sensor_id == 2)
        .ok_or_else(|| eyre!("failed to find a calibration result that matches the id"))?
        .sensor_calibration_value;

    Ok(Capture::create(
        1,
        CircuitState::Waiting,
        servo.clone(),
        photo.clone(),
        value,
    ))
}

pub fn construct_circuit(
    system: &mut System,
    calibration_results: &[CalibrationResult],
) -> Result<CircuitController> {
    let circuits: Vec<Arc<Mutex<Box<dyn Circuit + Send>>>> = vec![
        Arc::new(Mutex::new(Box::new(construct_feeder(
            system,
            calibration_results,
        )?))),
        Arc::new(Mutex::new(Box::new(construct_capture(
            system,
            calibration_results,
        )?))),
    ];
    Ok(CircuitController::create(circuits))
}
