use color_eyre::eyre::{eyre, Context};
use color_eyre::Result;
use futures::sink::SinkExt;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
pub enum PictureFormat {
    Binary,
    FileLocation,
    TakePicture,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardData {
    pub type_of: PictureFormat,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Request {
    CardData(CardData),
    EndConnection,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub error: i32,
    pub value: i32,
}

#[derive(Debug)]
pub struct Network {
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
        let buf = serde_pickle::to_vec(&payload, Default::default())?;
        self.framed_stream.send(buf.into()).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Response> {
        if let Some(buf) = self.framed_stream.next().await {
            match buf {
                Ok(v) => {
                    let response: Response = serde_pickle::from_slice(&v, Default::default())
                        .wrap_err_with(|| "failed to deserialise response")?;
                    return Ok(response);
                }
                Err(_) => return Err(eyre!("failed to generate a response from stream")),
            }
        }
        Err(eyre!("something went wrong when recieving a response"))
    }
}
