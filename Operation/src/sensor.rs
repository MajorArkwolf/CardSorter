
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SensorType {
    None = 0,
    PixelLight = 1,
    PhotoResistor = 2,
    ServoMotor = 3,
    DeattachedServoMotor = 4,
    Motor = 5,
}

impl Default for SensorType {
    fn default() -> Self {
        SensorType::None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    #[serde(default)]
    pub sensor_id: u8,
    #[serde(rename = "SensorType")]
    #[serde(default)]
    pub sensor_type: SensorType,
    #[serde(rename = "BoardID")]
    pub board_id: i64,
    #[serde(rename = "Params")]
    pub params: Params,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    #[serde(rename = "Pin")]
    pub pin: i64,
    #[serde(rename = "NumberOfLeds")]
    pub number_of_leds: Option<i64>,
}
