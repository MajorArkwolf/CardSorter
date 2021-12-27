pub mod servo;

trait DigitalIo {
    fn get() -> Result<bool>;
    fn set(value: bool) -> Result<()>;
}

trait AnalogIo {
    fn get() -> Result<u8>;
    fn set(value: u8) -> Result<()>;
}