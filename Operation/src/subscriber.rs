use async_channel::{Receiver, Sender};
use color_eyre::eyre::{eyre, Context, Result};
use tracing::debug;

#[derive(Clone, Debug)]
pub struct Subscriber<T: Copy> {
    rx: Receiver<T>,
    value: T,
}

impl<T: Copy> Subscriber<T> {
    pub fn create(rx: Receiver<T>, value: T) -> Self {
        Self { rx, value }
    }

    pub async fn get(&mut self) -> Result<T> {
        while !self.rx.is_empty() {
            self.value = self.rx.recv().await?;
        }
        Ok(self.value)
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
            Err(_) => Err(eyre!("failed to send message to the subscriber")),
        }
    }
}
