use crate::network::{Packet, SessionHandler};
use crate::{GameError, GameErrorKind};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::error::RecvError;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) sender: Sender<SenderData>,
    pub(crate) sessions: Arc<SessionHandler>,
}

impl Debug for ConnectionHandle {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionHandle").finish_non_exhaustive()
    }
}

impl From<Sender<SenderData>> for ConnectionHandle {
    fn from(sender: Sender<SenderData>) -> Self {
        Self {
            sender,
            sessions: Arc::new(SessionHandler::default()),
        }
    }
}

impl ConnectionHandle {
    // TODO functionality goes here
    pub(crate) fn respond_to_ping(&self, challenge: u16) -> Result<(), GameError> {
        let mut packet = Packet::default();
        packet.write(|writer| writer.write_uint16(challenge));
        self.send_packet_directly(packet)
    }

    fn send_packet_directly(&self, packet: Packet) -> Result<(), GameError> {
        self.sender
            .try_send(SenderData::Packet(packet))
            .map_err(|_| GameErrorKind::ConnectionTerminated.into())
    }

    //    pub async fn send_packet_on_new_session(
    //        &self,
    //        mut packet: Packet,
    //    ) -> Result<Session, GameError> {
    //        let session = self
    //            .sessions
    //            .get()
    //            .ok_or(GameErrorKind::SessionIdsExhausted)?;
    //
    //        packet.header_mut().set_session(session.id().0);
    //
    //        self.sender.send(SenderData::Packet(packet)).await?;
    //
    //        Ok(session)
    //    }
}

pub enum SenderData {
    #[cfg(not(feature = "wasm"))]
    Raw(tokio_tungstenite::tungstenite::Message),
    Packet(Packet),
    Close,
}

impl From<RecvError> for GameError {
    #[inline]
    fn from(e: RecvError) -> Self {
        debug!("Connection Terminated: {e:?}");
        GameErrorKind::ConnectionTerminated.into()
    }
}

impl<T> From<SendError<T>> for GameError {
    #[inline]
    fn from(e: SendError<T>) -> Self {
        debug!("Connection Terminated: {e:?}");
        GameErrorKind::ConnectionTerminated.into()
    }
}

impl From<async_channel::RecvError> for GameError {
    #[inline]
    fn from(e: async_channel::RecvError) -> Self {
        debug!("Connection Terminated: {e:?}");
        GameErrorKind::ConnectionTerminated.into()
    }
}
