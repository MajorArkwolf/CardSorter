use crate::{board, sensor};
use board::{
    network::{EthernetTemplate, SerialTemplate},
    serial_board::generate_serial_board,
};
use board::{BoardContainer, BoardTypes, BoardWrapper};
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use sensor::{motor_controller, photo_resistor, servo, Sensor};
use std::{
    fs::{self, File},
    vec,
};
use std::{
    io::{Read, Write},
    sync::Arc,
};

use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CommunicationType {
    Serial(SerialTemplate),
    Ethernet(EthernetTemplate),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BoardTemplate {
    id: u32,
    identifier: String,
    comm_type: CommunicationType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SensorTemplate {
    sensor: Sensor,
    board_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SystemTemplate {
    boards: Vec<BoardTemplate>,
    sensors: Vec<SensorTemplate>,
}

fn generate_board(template: BoardTemplate) -> Result<BoardTypes> {
    match template.comm_type {
        CommunicationType::Serial(v) => generate_serial_board(v, template.identifier),
        CommunicationType::Ethernet(_) => Err(eyre!("ethernet not implemented yet")),
    }
}

pub fn generate_system() -> Result<BoardContainer> {
    // Load our temp structure in to begin construction
    let mut file = File::open("./system.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let template: SystemTemplate = serde_json::from_str(&contents)?;
    info!("generating boards");
    let mut boards: Vec<BoardWrapper> = vec![];
    for template_board in template.boards {
        let id = template_board.id;
        let board = generate_board(template_board).wrap_err_with(|| "failed to generate board")?;
        boards.push(BoardWrapper::new(id, board));
    }

    for sensor in template.sensors {
        match boards.iter_mut().find(|x| *x.id() == sensor.board_id) {
            Some(v) => v.add_sensor(sensor.sensor),
            None => {
                return Err(eyre!(
                    "fail to find a board with board id {}, required for sensor {:?}",
                    sensor.board_id,
                    sensor.sensor,
                ));
            }
        };
    }
    info!("Found {} boards that were setup succesfully.", boards.len());
    Ok(BoardContainer::create(boards))
}

pub fn generate_mock_json() -> Result<()> {
    let mut system = SystemTemplate {
        boards: Vec::new(),
        sensors: Vec::new(),
    };
    let serial_template = SerialTemplate { baud_rate: 9600 };
    let serial_comm = CommunicationType::Serial(serial_template);
    let board = BoardTemplate {
        id: 0,
        identifier: "COM4".to_string(),
        comm_type: serial_comm,
    };
    system.boards.push(board);
    let sensor1 = Sensor::Servo(servo::Servo::create(1, 8));
    let sensor2 = Sensor::PhotoResistor(photo_resistor::PhotoResistor::create(1, 4));
    let sensor3 =
        Sensor::MotorController(motor_controller::MotorController::create(1, [4, 5, 0, 0]));
    system.sensors.push(SensorTemplate {
        sensor: sensor1,
        board_id: 1,
    });
    system.sensors.push(SensorTemplate {
        sensor: sensor2,
        board_id: 1,
    });
    system.sensors.push(SensorTemplate {
        sensor: sensor3,
        board_id: 2,
    });
    let data = serde_json::to_string(&system).unwrap();
    fs::write("system.json", &data).wrap_err_with(|| "failed to write to file")?;
    Ok(())
}
