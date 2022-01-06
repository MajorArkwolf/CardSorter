use async_channel::{Receiver, Sender};
use color_eyre::eyre::{eyre, Context, Result};
use tracing::error;

#[derive(Clone, Debug)]
pub struct Subscriber<T: Copy> {
    rx: Receiver<T>,
}

impl<T: Copy> Subscriber<T> {
    pub fn create(rx: Receiver<T>) -> Self {
        Self { rx }
    }

    pub async fn get(&mut self) -> Result<T> {
        loop {
            let y = self
                .rx
                .recv()
                .await
                .wrap_err_with(|| "failed to recv update");
            if self.rx.is_empty() {
                return y;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Publisher<T: Copy> {
    tx: Sender<T>,
}

impl<T: Copy> Publisher<T> {
    pub fn create(tx: Sender<T>) -> Self {
        Self { tx }
    }

    pub async fn set(&mut self, value: T) -> Result<()> {
        match self.tx.send(value).await {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("publisher failed to send message");
                Err(eyre!("failed to send message to the subscriber"))
            }
        }
    }
}
