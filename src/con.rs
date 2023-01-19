use crate::blk::BlockManager;
use crate::packet::Packet;
use futures_util::sink::SinkExt;
use futures_util::TryStreamExt;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct Connection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub(crate) block_manager: BlockManager,
}

impl Connection {
    pub const DEFAULT_HOST: &'static str = "flattiverse.com";

    pub async fn connect(api_key: &str) -> Result<Self, OpenError> {
        Self::connect_to(Self::DEFAULT_HOST, api_key).await
    }

    pub async fn connect_to(host: &str, api_key: &str) -> Result<Self, OpenError> {
        let (stream, response) = connect_async(format!("ws://{host}?auth={api_key}")).await?;
        println!("{response:?}");
        Ok(Connection {
            stream,
            block_manager: BlockManager::default(),
        })
    }

    #[inline]
    async fn send_json_text(&mut self, data: &impl Serialize) -> Result<(), SendError> {
        self.stream
            .send(Message::Text(serde_json::to_string(data)?))
            .await?;
        Ok(())
    }

    pub async fn send_packet(&mut self, packet: &Packet) -> Result<(), SendError> {
        self.send_json_text(&packet).await
    }

    pub async fn update(&mut self) {
        while let Ok(Some(Message::Text(text))) = self.stream.try_next().await {
            match serde_json::from_str(&text) {
                Err(e) => eprintln!("Failed to decode packet: {e:?}"),
                Ok(packet) => {
                    if let Err(packet) = self.block_manager.answer(packet) {
                        self.block_manager.unblock(&packet.id);
                    }
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
    #[error("Failed to encode request as JSON")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Failed to transmit request")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
}
