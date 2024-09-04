use crate::galaxy_hierarchy::{ControllableId, PlayerId, TeamId};
use crate::network::{InvalidArgumentKind, Packet, Session, SessionHandler};
use crate::utils::check_name_or_err_32;
use crate::{GameError, GameErrorKind, Vector};
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

    /// Call this to close a [`crate::galaxy_hierarchy::Controllable`].
    #[inline]
    pub async fn dispose_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.dispose_controllable_split(controllable).await?.await
    }

    /// Call this to close a [`crate::galaxy_hierarchy::Controllable`].
    pub async fn dispose_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x8F);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to continue the game with the unit after you are dead or when you hve created the
    /// unit.
    #[inline]
    pub async fn continue_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.continue_controllable_split(controllable).await?.await
    }

    /// Call this to continue the game with the unit after you are dead or when you hve created the
    /// unit.
    pub async fn continue_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x84);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to suicide (=self destroy).
    #[inline]
    pub async fn suicide_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.suicide_controllable_split(controllable).await?.await
    }

    /// Call this to suicide (=self destroy).
    pub async fn suicide_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x85);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to move your ship. This vector will be the impulse your ship gets every tick until
    /// you specify a new vector. Length of 0 will turn off your engines.
    #[inline]
    pub async fn classic_controllable_move(
        &self,
        controllable: ControllableId,
        movement: Vector,
    ) -> Result<(), GameError> {
        self.classic_controllable_move_split(controllable, movement)
            .await?
            .await
    }

    /// Call this to move your ship. This vector will be the impulse your ship gets every tick until
    /// you specify a new vector. Length of 0 will turn off your engines.
    pub async fn classic_controllable_move_split(
        &self,
        controllable: ControllableId,
        movement: Vector,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if movement.is_damaged() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::ConstrainedInfinity,
                parameter: "movement".to_string(),
            }
            .into())
        } else if movement.length() > 0.101f32 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooLarge,
                parameter: "movement".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x87);
            packet.write(|writer| {
                writer.write_byte(controllable.0);
                movement.write(writer);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |_| Ok(()))
            })
        }
    }

    /// Create a classic style ship.
    #[inline]
    pub async fn create_classic_style_ship(
        &self,
        name: impl AsRef<str>,
    ) -> Result<ControllableId, GameError> {
        self.create_classic_style_ship_split(name).await?.await
    }

    /// Create a classic style ship.
    pub async fn create_classic_style_ship_split(
        &self,
        name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<ControllableId, GameError>>, GameError> {
        check_name_or_err_32(name.as_ref())?;

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x80);
        packet.write(|writer| writer.write_string_with_len_prefix(name.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                Ok(packet.read(|reader| ControllableId(reader.read_byte())))
            })
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
