use crate::blk::BlockManager;
use crate::packet::{Command, ServerRequest};
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::time::UNIX_EPOCH;
use tokio::net::TcpStream;
use tokio::sync::oneshot::Receiver;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::units::uni::UniverseEvent;

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
    async fn send_json_text(&mut self, data: &impl serde::Serialize) -> Result<(), SendError> {
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
    ) -> Result<Receiver<ServerMessage>, SendError> {
        let (block_id, receiver) = self.block_manager.next_block();
        self.send(&ServerRequest {
            id: block_id,
            command: command.into(),
            parameters: Default::default(),
        })
            .await?;
        Ok(receiver)
    }

    pub async fn send_ping(&mut self) {
        let _ = self
            .stream
            .send(Message::Ping(current_time_millis().to_be_bytes().to_vec()))
            .await;
    }

    pub async fn update(&mut self) -> Result<UpdateEvent, ReceiveError> {
        loop {
            match self.stream.next().await {
                None => return Err(ReceiveError::ConnectionClosed),
                Some(Err(e)) => return Err(ReceiveError::ConnectionError(e)),
                Some(Ok(Message::Text(text))) => {
                    eprintln!("RECEIVED {text}");
                    let response = serde_json::from_str::<ServerMessage>(&text)?;
                    eprintln!("RESPONSE {response:?}");

                    if let Err(r) = self.block_manager.answer(response) {
                        eprintln!("GONE {r:?}");
                    }
                }
                Some(Ok(Message::Close(_))) => {
                    return Ok(UpdateEvent::ConnectionGracefullyClosed);
                }
                Some(Ok(Message::Pong(data))) => {
                    let mut millis = 0_u64.to_be_bytes();
                    let millis_len = millis.len();
                    if millis_len <= data.len() {
                        millis.copy_from_slice(&data[..millis_len]);
                        return Ok(UpdateEvent::PingMeasurement {
                            millis: (current_time_millis()
                                .saturating_sub(u64::from_be_bytes(millis))
                                as u32),
                        });
                    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerMessage {
    Error {
        id: String,
        result: String,
    },
    #[serde(rename = "success")]
    Success {
        id: String,
        result: i64,
    },
    #[serde(rename = "events")]
    Events(ServerEvents),
}

impl ServerMessage {
    pub fn command_id(&self) -> Option<&str> {
        match self {
            Self::Error { id, .. } => Some(id),
            Self::Success { id, .. } => Some(id),
            Self::Events { .. } => None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEvents {
    pub tick: u64,
    pub payload: Vec<UniverseEvent>,
}

#[derive(Debug, Clone)]
pub enum UpdateEvent {
    ConnectionGracefullyClosed,
    PingMeasurement { millis: u32 },
}

fn current_time_millis() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as u64
}
