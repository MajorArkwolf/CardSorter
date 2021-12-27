use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::{Result, WrapErr, eyre};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::loader;
use crate::sensor;
use crate::serial;

#[derive(Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "Boards")]
    pub board: Vec<u8>,
    #[serde(rename = "Sensors")]
    pub sensors: Vec<sensor::Sensor>
}

pub struct OverSeer {
    pub system_data: SystemData,
    pub serial_comm: serial::SerialComm,
}

impl OverSeer {
    pub fn new() -> Result<OverSeer> {
        let json_string = loader::load_file("A:\\Coding\\CardSorter\\Operation\\res\\System.json")?;
        let temp_data: SystemData = serde_json::from_str(&json_string).wrap_err_with(||"failed to parse json file")?;
        Ok(OverSeer{system_data: temp_data, serial_comm: serial::SerialComm::connect()?})
    }

    pub fn setup_system(&mut self) -> Result<()> {
        // Test to ensure our system has found connected to the correct device.
        self.serial_comm.send(ascii::AsciiStr::from_ascii("{\"Ping\": true}")?).wrap_err_with(||"failed to send ping to embedded system")?;
        let response: String = self.serial_comm.recieve().wrap_err_with(||"failed to recieve pong from embedded system")?.to_string();
        println!("We got: {}", response);
        let v: Value = serde_json::from_str(&response).wrap_err_with(||"failed to deserialise response")?;
        if v.get("Pong").is_none() {return Err(eyre!("system failed to respond with the correct response"));}
        
        // Once here we are satisfied to continue
        let system_data = json!(self.system_data);
        let pay_load = json!({ "Register": system_data.to_string()});
        println!("Beginning transmission: {}", pay_load);
        self.serial_comm.send(ascii::AsciiStr::from_ascii(pay_load.to_string().as_bytes())?).wrap_err_with(||"failed to parse payload json")?;
        let resp: String = self.serial_comm.recieve().wrap_err_with(||"failed to recieve response from IOOverseer system")?.to_string();
        println!("We got: {}", resp);
        
        Ok(())
    }

    pub fn run(&mut self, delta_time: std::time::Duration) -> Result<()> {

        Ok(())
    }
}
