use crate::hierarchy::{ClusterConfig, ClusterId, RegionConfig};
use crate::network::{Packet, Session, SessionHandler};
use crate::{GameError, GameErrorKind};
use async_channel::{RecvError, SendError, Sender};
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

    /// Sets the given values for the given [`Cluster`].
    #[inline]
    pub async fn configure_cluster(
        &self,
        cluster: ClusterId,
        config: &ClusterConfig,
    ) -> Result<(), GameError> {
        self.configure_cluster_split(cluster, config).await?.await
    }

    /// Sets the given values for the given [`Cluster`].
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

    /// Creates a [`Region`] with the given values for the given [`Cluster`].
    #[inline]
    pub async fn create_region(
        &self,
        cluster: ClusterId,
        config: &RegionConfig,
    ) -> Result<(), GameError> {
        self.create_region_split(cluster, config).await?.await
    }

    /// Creates a [`Region`] with the given values for the given [`Cluster`].
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

    #[inline]
    pub async fn remove_cluster(&self, cluster: ClusterId) -> Result<(), GameError> {
        self.remove_cluster_split(cluster).await?.await
    }

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
