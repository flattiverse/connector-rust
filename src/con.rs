use crate::blk::BlockManager;
use crate::packet::{Command, ServerRequest};
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use serde::Serialize;
use serde_derive::Deserialize;
use tokio::net::TcpStream;
use tokio::sync::oneshot::Receiver;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct Connection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub(crate) block_manager: BlockManager,
}

impl Connection {
    pub const DEFAULT_HOST: &'static str = "www.flattiverse.com/api/universes/beginnersGround.ws";

    pub async fn connect(api_key: &str) -> Result<Self, OpenError> {
        Self::connect_to(Self::DEFAULT_HOST, api_key).await
    }

    pub async fn connect_to(host: &str, api_key: &str) -> Result<Self, OpenError> {
        let (stream, response) = connect_async(format!("wss://{host}?auth={api_key}")).await?;
        println!("{response:?}");
        Ok(Connection {
            stream,
            block_manager: BlockManager::default(),
        })
    }

    #[inline]
    async fn send_json_text(&mut self, data: &impl Serialize) -> Result<(), SendError> {
        self.stream
            .send(Message::Text({
                let text = serde_json::to_string(data)?;
                eprintln!("SENDING: {text}");
                text
            }))
            .await?;
        Ok(())
    }

    #[inline]
    pub async fn send(&mut self, packet: &ServerRequest) -> Result<(), SendError> {
        self.send_json_text(&packet).await
    }

    pub async fn send_block_command(
        &mut self,
        command: impl Into<Command>,
    ) -> Result<Receiver<CommandResponse>, SendError> {
        let (block_id, receiver) = self.block_manager.next_block();
        self.send(&ServerRequest {
            id: block_id,
            command: command.into(),
            parameters: Default::default(),
        })
            .await?;
        Ok(receiver)
    }

    pub async fn update(&mut self) -> Result<(), ReceiveError> {
        loop {
            match self.stream.next().await {
                None => return Err(ReceiveError::ConnectionClosed),
                Some(Err(e)) => return Err(ReceiveError::ConnectionError(e)),
                Some(Ok(Message::Text(text))) => {
                    eprintln!("RECEIVED {text}");
                    let response = serde_json::from_str::<CommandResponse>(&text)?;
                    eprintln!("RESPONSE {response:?}");

                    if let Err(r) = self
                        .block_manager
                        .answer(response)
                    {
                        eprintln!("GONE {r:?}");
                    }
                }
                Some(Ok(Message::Close(_))) => {
                    return Ok(());
                }
                Some(Ok(Message::Ping(ping))) => {
                    self.stream.send(Message::Pong(ping)).await?;
                }
                Some(Ok(msg)) => {
                    return Err(ReceiveError::UnexpectedData(format!("{msg:?}")));
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OpenError {
    #[error("Underlying connection error")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum SendError {
    #[error("Failed to encode request as JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Failed to transmit request: {0}")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ReceiveError {
    #[error("Fatal server error: `{message}` on id {id:?}")]
    Fatal { message: String, id: Option<String> },
    #[error("Connection is closed")]
    ConnectionClosed,
    #[error("Connection has encountered an error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Unexpected data received: {0}")]
    UnexpectedData(String),
    #[error("Failed to decode the JSON response: {0}")]
    DecodeError(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum CommandResponse {
    Error {
        id: String,
        result: String,
    },
    #[serde(rename = "success")]
    Success {
        id: String,
        result: i64,
    },
}

impl CommandResponse {
    pub fn id(&self) -> &str {
        match self {
            Self::Error { id, .. } => id,
            Self::Success { id, .. } => id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum ResponseKind {
    #[serde(rename = "success")]
    Success,
}
