use color_eyre::eyre::Context;
use color_eyre::Result;
use futures::sink::SinkExt;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_util::codec::{BytesCodec, Framed, LengthDelimitedCodec};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum PictureFormat {
    Jpg,
    Png,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CardData {
    format: PictureFormat,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Request {
    CardData(CardData),
}

struct Network {
    framed_stream: Framed<TcpStream, LengthDelimitedCodec>,
}

impl Network {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(&addr)
            .await
            .wrap_err_with(|| "failed to connect to network device")?;
        let framed_stream = Framed::new(stream, LengthDelimitedCodec::new());
        Ok(Self { framed_stream })
    }

    pub async fn send(&mut self, payload: Request) -> Result<()> {
        let mut buf = Vec::new();
        payload.serialize(&mut Serializer::new(&mut buf))?;
        self.framed_stream.send(buf.into()).await?;
        Ok(())
    }
}
