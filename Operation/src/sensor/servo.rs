use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use firmata::asynchronous::board::Board;
use getset::Getters;

#[derive(Clone, Debug, Getters)]
pub struct Servo {
    #[get = "pub"]
    id: u32,
    pin: firmata::PinId,
    board: Board,
}

impl Servo {
    pub fn create(id: u32, pin: u8, board: Board) -> Self {
        let pin = firmata::PinId::Pin(pin);

        Self { id, pin, board }
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
