use crate::board::{generate_board_io, BoardTemplate, FirmataBoardTask};
use crate::sensor::led_strip::LedStrip;
use crate::sensor::motor_controller::MotorController;
use crate::sensor::photo_resistor::PhotoResistor;
use crate::sensor::servo::Servo;
use crate::sensor::{SensorContainer, SensorTemplate, Type};
use color_eyre::eyre::{eyre, Context, Result};
use std::io::Read;
use std::{
    fs::{self, File},
    vec,
};

use tracing::info;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
pub struct System {
    pub board_tasks: Vec<FirmataBoardTask>,
    pub sensors: SensorContainer,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemTemplate {
    boards: Vec<BoardTemplate>,
    sensors: Vec<SensorTemplate>,
}

async fn generate_board(template: BoardTemplate) -> Result<FirmataBoardTask> {
    let board = generate_board_io(template.address).await?;
    Ok(FirmataBoardTask::create(template.id, board))
}

pub async fn generate_system() -> Result<System> {
    info!("Beginning system generation...");
    // Load our structure in from json to begin construction
    let mut file = File::open("./system.json")
        .wrap_err_with(|| "failed to find system.json file in the root directory")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let template: SystemTemplate = serde_json::from_str(&contents)?;

    let mut system = System::default();

    // Generate the boards we will be interfacing with
    info!("Generating boards...");
    for temp_board in template.boards {
        if system.board_tasks.iter().any(|x| *x.id() == temp_board.id) {
            return Err(eyre!("board id already exists, {:?}", temp_board));
        }
        system.board_tasks.push(generate_board(temp_board).await?);
    }
    info!("Generating boards complete.");

    // Generate sensors that will be required for this system
    info!("Generating sensors.");

    for sensor in template.sensors {
        if system.sensors.find_sensor_by_id(sensor.id) != Type::None {
            return Err(eyre!("sensor id already exists, {:?}", sensor));
        }

        let board = match system
            .board_tasks
            .iter_mut()
            .find(|x| *x.id() == sensor.board_id)
        {
            Some(v) => v.board().clone(),
            None => {
                return Err(eyre!("fail to find a board with board id, {:?}", sensor,));
            }
        };
        match sensor.sensor_type {
            Type::MotorController => {
                let mut en_pins: [u8; 2] = [0, 0];
                let mut pins: [u8; 4] = [0, 0, 0, 0];
                pins.copy_from_slice(&sensor.pins[0..4]);
                en_pins.copy_from_slice(&sensor.pins[4..6]);
                system
                    .sensors
                    .motor_controllers
                    .push(MotorController::create(sensor.id, en_pins, pins, board).await?)
            }
            Type::PhotoResistor => system
                .sensors
                .photo_resistor
                .push(PhotoResistor::create(sensor.id, sensor.pins[0], board).await?),
            Type::Servo => system
                .sensors
                .servos
                .push(Servo::create(sensor.id, sensor.pins[0], board).await?),
            Type::LedStrip => system
                .sensors
                .led_strips
                .push(LedStrip::create(sensor.id, board)),
            Type::None => return Err(eyre!("None is not a valid sensor")),
        }
    }
    info!("Generating sensors complete.");
    info!(
        "Found {} boards that were setup succesfully.",
        system.board_tasks.len()
    );
    Ok(system)
}

pub fn generate_mock_json() -> Result<()> {
    let mut system = SystemTemplate {
        boards: Vec::new(),
        sensors: Vec::new(),
    };

    // Sample board information
    let board_1 = BoardTemplate {
        id: 1,
        identifier: "FirmataBoard.ino".to_string(),
        address: "192.168.128.10:3030".to_string(),
    };

    let board_2 = BoardTemplate {
        id: 1,
        identifier: "FirmataBoard2.ino".to_string(),
        address: "192.168.128.11:3030".to_string(),
    };

    system.boards.push(board_1);
    system.boards.push(board_2);

    let sensor_1 = SensorTemplate {
        id: 1,
        board_id: 1,
        sensor_type: Type::Servo,
        pins: vec![3],
    };

    let sensor_2 = SensorTemplate {
        id: 2,
        board_id: 1,
        sensor_type: Type::PhotoResistor,
        pins: vec![0],
    };

    let sensor_3 = SensorTemplate {
        id: 3,
        board_id: 1,
        sensor_type: Type::MotorController,
        pins: vec![6, 7, 0, 0],
    };

    let sensor_4 = SensorTemplate {
        id: 4,
        board_id: 2,
        sensor_type: Type::LedStrip,
        pins: vec![],
    };
    system.sensors.push(sensor_1);
    system.sensors.push(sensor_2);
    system.sensors.push(sensor_3);
    system.sensors.push(sensor_4);

    let data = serde_json::to_string(&system).unwrap();
    fs::write("system.json", &data).wrap_err_with(|| "failed to write to file")?;
    Ok(())
}
