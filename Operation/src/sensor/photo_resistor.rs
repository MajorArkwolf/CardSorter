use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::asynchronous::board::Board;
use firmata::PinMode;
use getset::Getters;

#[derive(Clone, Debug, Getters)]
pub struct PhotoResistor {
    #[get = "pub"]
    id: u32,
    pin: firmata::PinId,
    board: Board,
}

impl PhotoResistor {
    pub async fn create(id: u32, pin: u8, mut board: Board) -> Result<Self> {
        let pin = firmata::PinId::Analog(pin);
        board
            .set_pin_mode(pin, PinMode::Analog)
            .await
            .wrap_err_with(|| eyre!("failed to register photo resistor"))?;
        Ok(Self { id, pin, board })
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
