use super::message::{
    BroadcasterTX, OverseerChannel, ResponseChannelRX, ResponseChannelTX, Signal, SignalMessage,
};
use color_eyre::eyre::{eyre, Context, Result};
use std::time::Duration;
use tokio::signal;
use tokio::time::timeout;
use tracing::{info, debug, error};

#[derive(Debug)]
pub struct Overseer {
    rx: ResponseChannelRX,
    tx: BroadcasterTX,
    channel_in_tx: ResponseChannelTX, // Needs to be held otherwise channel becomes void.
    registered_circuits: usize, // Since a circuit can drop for whatever reason, this will not always be array length.
}

impl Overseer {
    pub fn create() -> Self {
        let (broad_tx, _) = tokio::sync::broadcast::channel::<SignalMessage>(100);
        let (mpsc_tx, mpsc_rx) = tokio::sync::mpsc::channel::<SignalMessage>(100);
        Self {
            tx: broad_tx,
            rx: mpsc_rx,
            channel_in_tx: mpsc_tx,
            registered_circuits: 0,
        }
    }

    pub fn get_comm_channels(&mut self) -> OverseerChannel {
        self.registered_circuits += 1;
        let rx = self.tx.subscribe();
        OverseerChannel::create(rx, self.channel_in_tx.clone())
    }

    async fn broadcast(&mut self, signal: Signal) -> Result<()> {
        match self
            .tx
            .send(SignalMessage::create(signal, std::time::Instant::now()))
            .wrap_err_with(|| "failed to send broadcast out from overseer")
        {
            Ok(resp) => {
                // If responses dont match registered channels we should assume the worst
                // and prepare for shutdown.
                if resp != self.registered_circuits {
                    return Err(eyre!(
                        "Sent {0} messages, expected to send {1} to circuits",
                        resp,
                        self.registered_circuits
                    ));
                }
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }

    async fn start_up(&mut self) -> Result<()> {
        self.broadcast(Signal::Start).await?;

        // Track our responses to determine what circuit failed to respond if one does.
        let mut resp_counter: usize = 0;

        // loop until we get responses for all circuits to know they are ready.
        loop {
            // Check we have the expected amount of responses and exit if we do.
            if resp_counter == self.registered_circuits {
                break;
            }

            // Get message from response channel, we give a grace period of 1 second before we assume nothing else is coming
            let message = timeout(Duration::from_secs(100), self.rx.recv())
                .await
                .wrap_err_with(|| {
                    "during overseer startup a response took longer then a second to recieve"
                })?;
            match message {
                Some(v) => {
                    // If the signal is an acknowledge we add it to the list
                    if v.signal == Signal::Ack {
                        resp_counter += 1;
                    } else {
                        // If not an ack signal then something has gone wrong and we should terminate.
                        return Err(eyre!(
                            "recieved a signal from circuit that was ack, {:?}",
                            v
                        ));
                    }
                }
                None => {
                    return Err(eyre!(
                        "unexpected channel error, returned none during startup"
                    ))
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&mut self, message: SignalMessage) -> Result<()> {
        match message.signal {
            Signal::Kill => {
                self.broadcast(Signal::Kill).await?;
                Err(eyre!(
                    "recieved a kill signal from circuit, terminating program. {:?}",
                    message,
                ))
            }
            Signal::Stop => {
                self.broadcast(Signal::Stop).await?;
                Err(eyre!(
                    "recieved a stop signal from circuit, terminating program. {:?}",
                    message,
                ))
            }
            Signal::Start => Err(eyre!(
                "a start signal was returned by circuit, this is not defined behaviour. {:?}",
                message
            )),
            Signal::Resume => self.broadcast(Signal::Resume).await,
            Signal::Pause => self.broadcast(Signal::Pause).await,
            Signal::HeartBeat => Err(eyre!(
                "a heartbeat signal was returned by circuit, this is not defined behaviour. {:?}",
                message
            )),
            Signal::Ack => Ok(()),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Starting up oveerseer, issuing start up command");
        self.start_up().await?;

        info!("Oveerseer startup completing beginning main loop");
        loop {
            // Listen for circuit notifications or signals from the OS
            tokio::select! {
                v = self.rx.recv() => {
                    match v {
                        Some(msg) => self.handle_message(msg).await?,
                        None => break,
                    }
                }
                _ = signal::ctrl_c() => {
                    debug!("sigstop was issued, relaying to subscribers");
                    self.broadcast(Signal::Stop).await?;
                    break;
                }
            }
        }

        self.rx.close();
        info!("All registered circuits have been closed, overseer shutting down");
        Ok(())
    }
}
