use color_eyre::eyre::Result;
use color_eyre::eyre::{eyre, WrapErr};
use firmata::asynchronous::board::Board;
use firmata::PinMode;
use getset::Getters;

#[derive(Clone, Debug, Getters)]
pub struct Servo {
    #[get = "pub"]
    id: u32,
    pin: firmata::PinId,
    board: Board,
}

impl Servo {
    pub async fn create(id: u32, pin: u8, mut board: Board) -> Result<Self> {
        let pin = firmata::PinId::Pin(pin);
        board
            .set_pin_mode(pin, PinMode::Output)
            .await
            .wrap_err_with(|| eyre!("failed to register photo resistor"))?;
        Ok(Self { id, pin, board })
    }

    pub async fn get(&mut self) -> Result<u16> {
        self.board
            .get_pin_value(self.pin)
            .wrap_err_with(|| "failed to get servo pin")
    }

    pub async fn set(&mut self, value: u16) -> Result<()> {
        self.board.analog_write(self.pin, value).await?;
        Ok(())
    }
}
