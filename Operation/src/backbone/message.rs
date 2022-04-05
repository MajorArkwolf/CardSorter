use color_eyre::eyre::{Result, Context, eyre};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Kill, // Kill commands should be treated as immediate stops, anything that is moving needs to stop if/when possible.
    Stop, // Stop is treated as a shutdown, the circuit should finish any immediate tasks and then exit their tasks.
    Start, // Signals the systems that its time to start operations
    Resume, // Resume allows a pause to go back to running.
    Pause, // A pause is similiar to kill with the possability of starting again, the circuit should not continue to do anything until a resume is sent.
    HeartBeat, // This is the equivilent of a ping, a ack should be sent back when a heartbeat signal is detected.
    Ack, // Acknowledge should be sent on every notification recieved to ensure operations are working.
}

pub type BroadcasterTX = tokio::sync::broadcast::Sender<SignalMessage>;
pub type BroadcasterRX = tokio::sync::broadcast::Receiver<SignalMessage>;
pub type ResponseChannelTX = tokio::sync::mpsc::Sender<SignalMessage>;
pub type ResponseChannelRX = tokio::sync::mpsc::Receiver<SignalMessage>;

#[derive(Debug)]
pub struct OverseerChannel {
    pub rx: BroadcasterRX,
    pub tx: ResponseChannelTX,
}

impl OverseerChannel {
    pub fn create(rx: BroadcasterRX, tx: ResponseChannelTX) -> Self {
        Self { rx, tx }
    }

    pub async fn acknowledge(&mut self) -> Result<()> {
        self.tx.send(SignalMessage::create_simple(Signal::Ack)).await
        .wrap_err_with(|| eyre!("failed to send acknowledgement back to the oversser"))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SignalMessage {
    pub signal: Signal,
    pub time_stamp: std::time::Instant,
}

impl SignalMessage {
    pub fn create(signal: Signal, time_stamp: std::time::Instant) -> Self {
        Self { signal, time_stamp }
    }

    pub fn create_simple(signal: Signal) -> Self {
        Self {signal, time_stamp: std::time::Instant::now()}
    }
}
