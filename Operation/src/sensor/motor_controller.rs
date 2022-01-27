use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::{asynchronous::board::Board, PinId, PinMode};
use getset::Getters;

const MOTOR_A_PINS_INDEX: [usize; 2] = [0, 1];
const MOTOR_B_PINS_INDEX: [usize; 2] = [2, 3];
const MOTOR_FORWARD: [bool; 2] = [true, false];
const MOTOR_REVERSE: [bool; 2] = [false, true];
const MOTOR_STOP: [bool; 2] = [false, false];

async fn check_and_set_pwm(pin: u8, board: &mut Board) -> Result<()> {
    if pin > 0 {
        let pin_id = PinId::Digital(pin);
        let pins = board.pins();
        if pins[pin as usize]
            .modes
            .iter()
            .any(|f| f.mode == PinMode::Pwm)
        {
            board
                .set_pin_mode(pin_id, PinMode::Pwm)
                .await
                .wrap_err_with(|| eyre!("failed to register photo resistor"))?;
            board.analog_write(pin_id, 200).await?;
        } else {
            board
                .set_pin_mode(pin_id, PinMode::Output)
                .await
                .wrap_err_with(|| eyre!("failed to register photo resistor"))?;
            board.digital_write(pin_id, true).await?;
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Copy)]
pub enum Motor {
    A,
    B,
}
#[derive(Clone, Debug, Copy)]
pub enum Movement {
    Stop,
    Forward,
    Reverse,
}

impl Default for Movement {
    fn default() -> Self {
        Movement::Stop
    }
}

#[derive(Getters, Clone, Debug)]
pub struct MotorController {
    #[get = "pub"]
    id: u32,
    pins: [u8; 4],
    board: Board,
}

impl MotorController {
    pub async fn create(
        id: u32,
        en_pins: [u8; 2],
        pins: [u8; 4],
        mut board: Board,
    ) -> Result<Self> {
        for pin in en_pins {
            check_and_set_pwm(pin, &mut board).await?;
        }

        for pin in pins {
            if pin > 0 {
                let pin = PinId::Digital(pin);
                board
                    .set_pin_mode(pin, PinMode::Output)
                    .await
                    .wrap_err_with(|| eyre!("failed to register photo resistor"))?;
            }
        }
        Ok(Self { id, pins, board })
    }

    pub async fn set(&mut self, motor: Motor, movement: Movement) -> Result<()> {
        let pins = match motor {
            Motor::A => [
                self.pins[MOTOR_A_PINS_INDEX[0]],
                self.pins[MOTOR_A_PINS_INDEX[1]],
            ],
            Motor::B => [
                self.pins[MOTOR_B_PINS_INDEX[0]],
                self.pins[MOTOR_B_PINS_INDEX[1]],
            ],
        };
        if pins[0] == 0 && pins[1] == 0 {
            return Err(eyre!("Pin {} and {} are not valid pins", pins[0], pins[1]));
        }
        let digital_assignment = match movement {
            Movement::Stop => MOTOR_STOP,
            Movement::Forward => MOTOR_FORWARD,
            Movement::Reverse => MOTOR_REVERSE,
        };
        for (i, pin) in pins.iter().enumerate() {
            self.board
                .digital_write(firmata::PinId::Digital(*pin), digital_assignment[i])
                .await?;
        }
        Ok(())
    }
}
