use crate::network::{Packet, Session, SessionHandler};
use async_channel::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;

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

        packet.update_header(|h| {
            h.set_session(session.id());
        });

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
