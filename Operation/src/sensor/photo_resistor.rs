use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use firmata::asynchronous::board::Board;

#[derive(Clone, Debug)]
pub struct PhotoResistor {
    id: u32,
    pin: firmata::PinId,
    board: Board,
}

impl PhotoResistor {
    pub fn create(id: u32, pin: u8, board: Board) -> Self {
        let pin = firmata::PinId::Analog(pin);
        Self { id, pin, board }
    }

    pub fn get_pin_id(&self) -> firmata::PinId {
        self.pin
    }

    pub fn get(&mut self) -> Result<u16> {
        self.board
            .get_pin_value(self.pin)
            .wrap_err_with(|| "failed to get photoresistor pin")
    }
}
