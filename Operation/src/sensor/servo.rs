use crate::sensor::AnalogIo;

pub struct Servo {
    pin: u8
}

impl Servo {
    pub fn new(pin: u8) -> Result<Self> {
        todo!("Implement setting this up on the arduino");
        Servo{pin}.into()
    }
}

impl AnalogIo for Servo {
    fn get() -> Result<u8> {

    }

    fn set(value: u8) -> Result<()> {
        
    }
}