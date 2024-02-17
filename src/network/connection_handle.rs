use crate::hierarchy::{ClusterConfig, ClusterId, RegionConfig, RegionId, TeamConfig};
use crate::network::{Packet, Session, SessionHandler};
use crate::{GameError, GameErrorKind, TeamId};
use async_channel::{RecvError, SendError, Sender};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) sender: Sender<SenderData>,
    pub(crate) sessions: Arc<Mutex<SessionHandler>>,
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
            sessions: Arc::new(Mutex::new(SessionHandler::default())),
        }
    }
}

impl ConnectionHandle {
    #[inline]
    pub async fn is_even(&self, number: i32) -> Result<bool, GameError> {
        self.is_even_split(number).await?.await
    }

    pub async fn is_even_split(
        &self,
        number: i32,
    ) -> Result<impl Future<Output = Result<bool, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x55);
        packet.write(|writer| writer.write_int32(number));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            Ok(response.header().param0() != 0)
        })
    }

    /// Sets the given values for the given [`crate::hierarchy::Cluster`].
    #[inline]
    pub async fn configure_cluster(
        &self,
        cluster: ClusterId,
        config: &ClusterConfig,
    ) -> Result<(), GameError> {
        self.configure_cluster_split(cluster, config).await?.await
    }

    /// Sets the given values for the given [`crate::hierarchy::Cluster`].
    pub async fn configure_cluster_split(
        &self,
        cluster: ClusterId,
        config: &ClusterConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x42);
        packet.header_mut().set_param0(cluster.0);
        packet.write(|writer| config.write_to(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Removes the given [`crate::hierarchy::Cluster`]
    #[inline]
    pub async fn remove_cluster(&self, cluster: ClusterId) -> Result<(), GameError> {
        self.remove_cluster_split(cluster).await?.await
    }

    /// Removes the given [`crate::hierarchy::Cluster`]
    pub async fn remove_cluster_split(
        &self,
        cluster: ClusterId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x43);
        packet.header_mut().set_param0(cluster.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Creates a [`crate::hierarchy::Region`] with the given values for the given
    /// [`crate::hierarchy::Cluster`].
    #[inline]
    pub async fn create_region(
        &self,
        cluster: ClusterId,
        config: &RegionConfig,
    ) -> Result<(), GameError> {
        self.create_region_split(cluster, config).await?.await
    }

    /// Creates a [`crate::hierarchy::Region`] with the given values for the given
    /// [`crate::hierarchy::Cluster`].
    pub async fn create_region_split(
        &self,
        cluster: ClusterId,
        config: &RegionConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x44);
        packet.header_mut().set_param0(cluster.0);
        packet.write(|writer| config.write_to(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sets the given values for the given [`crate::hierarchy::Region`].
    #[inline]
    pub async fn configure_region(
        &self,
        region: RegionId,
        config: &RegionConfig,
    ) -> Result<(), GameError> {
        self.configure_region_split(region, config).await?.await
    }

    /// Sets the given values for the given [`crate::hierarchy::Region`].
    pub async fn configure_region_split(
        &self,
        region: RegionId,
        config: &RegionConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x45);
        packet.header_mut().set_param0(region.0);
        packet.write(|writer| config.write_to(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            GameError::check(response, |_| Ok(()))
        })
    }
    /// Removes the given [`crate::hierarchy::Region`]
    #[inline]
    pub async fn remove_region(&self, region: RegionId) -> Result<(), GameError> {
        self.remove_region_split(region).await?.await
    }

    /// Removes the given [`crate::hierarchy::Region`]
    pub async fn remove_region_split(
        &self,
        region: RegionId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x46);
        packet.header_mut().set_param0(region.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let respones = session.receiver.recv().await?;
            GameError::check(respones, |_| Ok(()))
        })
    }

    /// Sets the given values for the given [`crate::Team`].
    #[inline]
    pub async fn configure_team(&self, team: TeamId, config: &TeamConfig) -> Result<(), GameError> {
        self.configure_team_split(team, config).await?.await
    }

    /// Sets the given values for the given [`crate::Team`].
    pub async fn configure_team_split(
        &self,
        team: TeamId,
        config: &TeamConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x48);
        packet.header_mut().set_param0(team.0);
        packet.write(|writer| config.write_to(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Removes the given [`crate::Team`].
    #[inline]
    pub async fn remove_team(&self, team: TeamId) -> Result<(), GameError> {
        self.remove_team_split(team).await?.await
    }

    /// Removes the given [`crate::Team`].
    pub async fn remove_team_split(
        &self,
        team: TeamId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x49);
        packet.header_mut().set_param0(team.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.receiver.recv().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    pub async fn send_packet_on_new_session(
        &self,
        mut packet: Packet,
    ) -> Result<Session, GameError> {
        let session = {
            self.sessions
                .lock()
                .await
                .get()
                .ok_or(GameErrorKind::ConnectionClosed)?
        };

        packet.header_mut().set_session(session.id());

        self.sender.send(SenderData::Packet(packet)).await?;

        Ok(session)
    }
}

pub enum SenderData {
    #[cfg(not(feature = "wasm"))]
    Raw(tokio_tungstenite::tungstenite::Message),
    Packet(Packet),
}

impl From<RecvError> for GameError {
    #[inline]
    fn from(_error: RecvError) -> Self {
        GameErrorKind::ConnectionClosed.into()
    }
}

impl<T> From<SendError<T>> for GameError {
    #[inline]
    fn from(_error: SendError<T>) -> Self {
        GameErrorKind::ConnectionClosed.into()
    }
}
