use crate::network::{Packet, Session, SessionHandler};
use crate::{GameError, GameErrorKind};
use async_channel::Sender;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) sender: Sender<SenderData>,
    pub(crate) sessions: Arc<Mutex<SessionHandler>>,
}

impl From<Sender<SenderData>> for ConnectionHandle {
    fn from(sender: Sender<SenderData>) -> Self {
        Self {
            sender,
            sessions: Arc::new(Mutex::new(SessionHandler::default())),
        }
    }
}

impl ConnectionHandle {
    #[inline]
    pub async fn is_even(&mut self, number: i32) -> Result<bool, GameError> {
        self.is_even_split(number).await?.await
    }

    pub async fn is_even_split(
        &mut self,
        number: i32,
    ) -> Result<impl Future<Output = Result<bool, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x55);
        packet.write(|writer| {
            writer.write_int32(number);
        });

        let session = self
            .send_packet_on_new_session(packet)
            .await
            .map_err(|_| GameError::from(GameErrorKind::ConnectionClosed))?;

        Ok(async move {
            let response = session
                .receiver
                .recv()
                .await
                .map_err(|_| GameError::from(GameErrorKind::ConnectionClosed))?;
            Ok(response.header().param0() != 0)
        })
    }

    pub async fn send_packet_on_new_session(
        &mut self,
        mut packet: Packet,
    ) -> Result<Session, SendError> {
        let session = {
            self.sessions
                .lock()
                .await
                .get()
                .ok_or(SendError::SessionIdsExhausted)?
        };

        packet.header_mut().set_session(session.id());

        self.sender
            .send(SenderData::Packet(packet))
            .await
            .map_err(|_| SendError::ConnectionGone)?;

        Ok(session)
    }
}

pub enum SenderData {
    #[cfg(not(feature = "wasm"))]
    Raw(tokio_tungstenite::tungstenite::Message),
    Packet(Packet),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("The connection is no longer reachable, your request could not be transmitted")]
    ConnectionGone,
    #[error("Cannot issue more session ids: all possible session ids are in use")]
    SessionIdsExhausted,
}
