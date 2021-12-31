//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod board;
pub mod factory;
pub mod sensor;
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use firmata::Firmata;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("beginning startup of system");
    factory::generate_mock_json()?;
    //factory::generate_system()?;

    //let ports =
    //    tokio_serial::available_ports().wrap_err_with(|| "failed to find any comm ports")?;

    //let serial_port = tokio_serial::new("COM3", 57600).open()?;
    //
    ////let port = ports
    ////    .into_iter()
    ////    .map(|v| tokio_serial::new(&v.port_name, 9600).open())
    ////    .last()
    ////    .ok_or_else(|| eyre!("failed to find comm port"))??;
    //
    //let mut raw_board = firmata::Board::new(serial_port);
    //raw_board.populate_board_info()?;
    //raw_board.set_pin_mode(firmata::PinId::Digital(13), firmata::OutputMode::OUTPUT)?;
    //raw_board.digital_write(firmata::PinId::Digital(13), 1)?;
    //let mut board = arduino_board::ArduinoBoard::new(Arc::new(Mutex::new(raw_board)));
    //let mut motor =
    //    sensor::motor_controller::MotorController::create(board.clone(), [4, 5, 0, 0]).await?;
    //
    //let mut servo = sensor::servo::Servo::create(board.clone(), 12).await?;
    //let mut psensor = sensor::photo_resistor::PhotoResistor::create(board.clone(), 0).await?;
    //
    //let mut x = true;
    //loop {
    //    if x {
    //        motor
    //            .set_motor(
    //                sensor::motor_controller::Motor::A,
    //                sensor::motor_controller::Movement::Forward,
    //            )
    //            .await?;
    //        servo.set(0).await?;
    //        x = false;
    //    } else {
    //        motor
    //            .set_motor(
    //                sensor::motor_controller::Motor::A,
    //                sensor::motor_controller::Movement::Stop,
    //            )
    //            .await?;
    //        x = true;
    //        servo.set(90).await?;
    //    }
    //    board.update().await?;
    //    let p_value = psensor.get().await?;
    //    println!("PhotoSensor: {}", p_value);
    //    println!("tick");
    //    std::thread::sleep(std::time::Duration::from_millis(2000));
    //}

    info!("terminating");
    Ok(())
}
