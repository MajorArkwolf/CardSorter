use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
use tokio_serial::SerialPort;
pub type SerialHandle = Box<dyn SerialPort>;
pub type SerialMutex = Arc<Mutex<SerialHandle>>;

/// Purpose is to hold a synchronous serial port and handle the input and output in a manner that stops collisions
pub struct SerialJobQueue {
    port: SerialHandle,
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl SerialJobQueue {
    pub fn new(port: SerialHandle) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        Self { port, tx, rx }
    }

    pub async fn update() {}

    pub async fn send() {}

    pub async fn recieve() {}

    pub fn make_async_serial(&self) -> AsyncSerial {
        AsyncSerial::new()
    }
}
#[derive(Debug)]
pub struct AsyncSerial {}

impl AsyncSerial {
    fn new() -> Self {
        Self {}
    }
}

impl Read for AsyncSerial {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl Write for AsyncSerial {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
