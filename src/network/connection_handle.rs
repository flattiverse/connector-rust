use crate::galaxy_hierarchy::{PlayerId, TeamId};
use crate::network::{Packet, Session, SessionHandler};
use crate::{GameError, GameErrorKind};
use std::fmt::{Debug, Formatter};
use std::future::Future;
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
    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Player`].
    #[inline]
    pub async fn chat_player(
        &self,
        player: PlayerId,
        message: impl AsRef<str>,
    ) -> Result<(), GameError> {
        self.chat_player_split(player, message).await?.await
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Player`].
    pub async fn chat_player_split(
        &self,
        player: PlayerId,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xC6);
        packet.write(|writer| {
            writer.write_byte(player.0);
            writer.write_string_with_len_prefix(message.as_ref())
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Team`].
    #[inline]
    pub async fn chat_team(&self, team: TeamId, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_team_split(team, message).await?.await
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Team`].
    pub async fn chat_team_split(
        &self,
        team: TeamId,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xC5);
        packet.write(|writer| {
            writer.write_byte(team.0);
            writer.write_string_with_len_prefix(message.as_ref())
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }
    /// Sends a chat message to all players in the connected [`crate::galaxy_hierarchy::Galaxy`].
    #[inline]
    pub async fn chat_galaxy(&self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_galaxy_split(message).await?.await
    }

    /// Sends a chat message with to all players in the connected [`crate::galaxy_hierarchy::Galaxy`].
    pub async fn chat_galaxy_split(
        &self,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xC4);
        packet.write(|writer| writer.write_string_with_len_prefix(message.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    // TODO functionality goes here
    pub(crate) fn respond_to_ping(&self, challenge: u16) -> Result<(), GameError> {
        let mut packet = Packet::default();
        packet.write(|writer| writer.write_uint16(challenge));
        self.send_packet_directly(packet)
    }

    #[inline]
    fn send_packet_directly(&self, packet: Packet) -> Result<(), GameError> {
        self.sender
            .try_send(SenderData::Packet(packet))
            .map_err(|_| GameErrorKind::ConnectionTerminated.into())
    }

    #[inline]
    async fn send_packet_on_new_session(&self, mut packet: Packet) -> Result<Session, GameError> {
        let session = self
            .sessions
            .get()
            .ok_or(GameErrorKind::SessionsExhausted)?;

        packet.header_mut().set_session(session.id().0);

        self.sender.send(SenderData::Packet(packet)).await?;

        Ok(session)
    }
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
