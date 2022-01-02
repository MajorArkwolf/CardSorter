use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
pub mod feeder;
enum CircuitState {
    Ready,
    Running,
    Waiting,
    Stopped,
}

trait Circuit {
    fn get_id() -> u32;
    fn get_state() -> CircuitState;
    fn change_state(next_state: CircuitState) -> Result<()>;
    fn update();
    fn stop();
}
