use color_eyre::eyre::Context;
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex};
use tokio_serial::SerialPort;
pub type SerialHandle = Box<dyn SerialPort>;
pub type SerialMutex = Arc<Mutex<SerialHandle>>;
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::{END_SYSEX, START_SYSEX};

struct Message {
    message: Vec<u8>,
    tx: mpsc::Sender<Vec<u8>>,
}

/// Purpose is to hold a synchronous serial port and handle the input and output in a manner that stops collisions
pub struct SerialJobQueue {
    port: SerialHandle,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
}

impl SerialJobQueue {
    pub fn new(port: SerialHandle) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        Self { port, tx, rx }
    }

    pub async fn update(&mut self) -> Result<()> {
        loop {
            let message_in = self.rx.try_recv();
            let message = match message_in {
                Ok(v) => v,
                Err(e) => match e {
                    mpsc::error::TryRecvError::Empty => break,
                    mpsc::error::TryRecvError::Disconnected => {
                        return Err(eyre!("communication has been disconnected"))
                    }
                },
            };
            self.port.write_all(&message.message)?; // This should not panic here and instead notify the message of failure

            let mut message_out: Vec<u8> = Vec::new();
            let mut start_of_msg = false;
            loop {
                let mut byte: [u8; 1] = [0; 1];
                let n = self.port.read(&mut byte[..1])?;
                if n == 1 {
                    if !start_of_msg {
                        if byte[0] == START_SYSEX {
                            message_out.push(byte[0]);
                            start_of_msg = true;
                        } else {
                            continue;
                        }
                    } else if byte[0] == END_SYSEX {
                        message_out.push(byte[0]);
                        break;
                    } else {
                        message_out.push(byte[0]);
                    }
                }
            }
            message.tx.send(message_out).await?;
        }
        Ok(())
    }

    pub fn make_async_serial(&self) -> AsyncSerial {
        AsyncSerial::new(self.tx.clone())
    }
}

#[derive(Debug)]
pub struct AsyncSerial {
    out_to_comm: mpsc::Sender<Message>,
    tx: mpsc::Sender<Vec<u8>>,
    rx: mpsc::Receiver<Vec<u8>>,
    bytes_from_message: Vec<u8>,
}

impl AsyncSerial {
    fn new(out_to_comm: mpsc::Sender<Message>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        Self {
            out_to_comm,
            tx,
            rx,
            bytes_from_message: Vec::new(),
        }
    }

    async fn get_message_out(&mut self) {
        loop {
            let message_in = self.rx.recv().await;
            let output = match message_in {
                Some(x) => x,
                None => return,
            };
            self.bytes_from_message = output;
        }
    }
}

impl Read for AsyncSerial {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.bytes_from_message.is_empty() {
            self.get_message_out();
        }
        let max_size = std::cmp::min(buf.len(), self.bytes_from_message.len());
        if max_size == 0 {
            return Ok(0);
        }
        buf.copy_from_slice(&self.bytes_from_message[..max_size]);
        self.bytes_from_message.drain(..max_size);
        Ok(max_size)
    }
}

impl Write for AsyncSerial {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size: usize = buf.len();
        let message = Message {
            message: buf.to_vec(),
            tx: self.tx.clone(),
        };
        self.out_to_comm.send(message);
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
