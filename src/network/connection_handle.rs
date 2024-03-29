use crate::hierarchy::{
    ClusterConfig, ClusterId, GalaxyConfig, GalaxyId, RegionConfig, RegionId, ShipDesignConfig,
    ShipDesignId, ShipUpgradeConfig, ShipUpgradeId, TeamConfig,
};
use crate::network::{Packet, Session, SessionHandler};
use crate::unit::configurations::{
    BlackHoleConfiguration, BuoyConfiguration, Configuration, MeteoroidConfiguration,
    MoonConfiguration, PlanetConfiguration, SunConfiguration,
};
use crate::unit::UnitKind;
use crate::utils::check_message_or_err;
use crate::{ControllableId, GameError, GameErrorKind, PlayerId, TeamId};
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
            let response = session.response().await?;
            Ok(response.header().param0() != 0)
        })
    }
    /// Sets the given values for the given [`crate::hierarchy::Galaxy`].
    #[inline]
    pub async fn configure_galaxy(
        &self,
        galaxy: GalaxyId,
        config: &GalaxyConfig,
    ) -> Result<(), GameError> {
        self.configure_galaxy_split(galaxy, config).await?.await
    }

    /// Sets the given values for the given [`crate::hierarchy::Galaxy`].
    pub async fn configure_galaxy_split(
        &self,
        galaxy: GalaxyId,
        config: &GalaxyConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x40);
        packet.header_mut().set_param(galaxy.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Creates a new [`crate::hierarchy::Cluster`] for the given values.
    #[inline]
    pub async fn create_cluster(&self, config: &ClusterConfig) -> Result<ClusterId, GameError> {
        self.create_cluster_split(config).await?.await
    }

    /// Creates a new [`crate::hierarchy::Cluster`] for the given values.
    pub async fn create_cluster_split(
        &self,
        config: &ClusterConfig,
    ) -> Result<impl Future<Output = Result<ClusterId, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x41);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |p| Ok(ClusterId(p.header().id0())))
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
        packet.header_mut().set_id0(cluster.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
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
        packet.header_mut().set_id0(cluster.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
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
    ) -> Result<RegionId, GameError> {
        self.create_region_split(cluster, config).await?.await
    }

    /// Creates a [`crate::hierarchy::Region`] with the given values for the given
    /// [`crate::hierarchy::Cluster`].
    pub async fn create_region_split(
        &self,
        cluster: ClusterId,
        config: &RegionConfig,
    ) -> Result<impl Future<Output = Result<RegionId, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x44);
        packet.header_mut().set_id0(cluster.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |p| Ok(RegionId(p.header().id0())))
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
        packet.header_mut().set_id0(region.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
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
        packet.header_mut().set_id0(region.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let respones = session.response().await?;
            GameError::check(respones, |_| Ok(()))
        })
    }

    /// Creates a new [`crate::Team`] for the given values.
    #[inline]
    pub async fn create_team(&self, config: &TeamConfig) -> Result<TeamId, GameError> {
        self.create_team_split(config).await?.await
    }

    /// Creates a new [`crate::Team`] for the given values.
    pub async fn create_team_split(
        &self,
        config: &TeamConfig,
    ) -> Result<impl Future<Output = Result<TeamId, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x47);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |p| Ok(TeamId(p.header().id0())))
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
        packet.header_mut().set_id0(team.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
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
        packet.header_mut().set_id0(team.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Creates an [`crate::Upgrade`] with the given values for the given [`crate::unit::ShipDesign`].
    #[inline]
    pub async fn create_upgrade(
        &self,
        ship: ShipDesignId,
        config: &ShipUpgradeConfig,
    ) -> Result<ShipUpgradeId, GameError> {
        self.create_upgrade_split(ship, config).await?.await
    }

    /// Creates an [`crate::Upgrade`] with the given values for the given [`crate::unit::ShipDesign`].
    pub async fn create_upgrade_split(
        &self,
        ship: ShipDesignId,
        config: &ShipUpgradeConfig,
    ) -> Result<impl Future<Output = Result<ShipUpgradeId, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x4D);
        packet.header_mut().set_id0(ship.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |p| Ok(ShipUpgradeId(p.header().id0())))
        })
    }

    /// Sets the given values for the given [`crate::Upgrade`].
    #[inline]
    pub async fn configure_upgrade(
        &self,
        upgrade: ShipUpgradeId,
        design: ShipDesignId,
        config: &ShipUpgradeConfig,
    ) -> Result<(), GameError> {
        self.configure_upgrade_split(upgrade, design, config)
            .await?
            .await
    }

    /// Sets the given values for the given [`crate::Upgrade`].
    pub async fn configure_upgrade_split(
        &self,
        upgrade: ShipUpgradeId,
        design: ShipDesignId,
        config: &ShipUpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x4E);
        packet.header_mut().set_id0(upgrade.0);
        packet.header_mut().set_id1(design.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Removes the given [`crate::Upgrade`].
    #[inline]
    pub async fn remove_upgrade(
        &self,
        upgrade: ShipUpgradeId,
        design: ShipDesignId,
    ) -> Result<(), GameError> {
        self.remove_upgrade_split(upgrade, design).await?.await
    }

    /// Removes the given [`crate::Upgrade`].
    pub async fn remove_upgrade_split(
        &self,
        upgrade: ShipUpgradeId,
        design: ShipDesignId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x4F);
        packet.header_mut().set_id0(upgrade.0);
        packet.header_mut().set_id1(design.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Creates a new [`crate::unit::ShipDesign`] for the given values.
    #[inline]
    pub async fn create_ship_design(
        &self,
        config: &ShipDesignConfig,
    ) -> Result<ShipDesignId, GameError> {
        self.create_ship_design_split(config).await?.await
    }

    /// Creates a new [`crate::unit::ShipDesign`] for the given values.
    pub async fn create_ship_design_split(
        &self,
        config: &ShipDesignConfig,
    ) -> Result<impl Future<Output = Result<ShipDesignId, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x4A);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |p| Ok(ShipDesignId(p.header().id0())))
        })
    }

    /// Sets the given values for the given [`crate::unit::ShipDesign`].
    #[inline]
    pub async fn configure_ship_design(
        &self,
        ship: ShipDesignId,
        config: &ShipDesignConfig,
    ) -> Result<(), GameError> {
        self.configure_ship_design_split(ship, config).await?.await
    }

    /// Sets the given values for the given [`crate::unit::ShipDesign`].
    pub async fn configure_ship_design_split(
        &self,
        ship: ShipDesignId,
        config: &ShipDesignConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x4B);
        packet.header_mut().set_id0(ship.0);
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Removes the given [`crate::unit::ShipDesign`].
    #[inline]
    pub async fn remove_ship_design(&self, ship: ShipDesignId) -> Result<(), GameError> {
        self.remove_ship_design_split(ship).await?.await
    }

    /// Removes the given [`crate::unit::ShipDesign`].
    pub async fn remove_ship_design_split(
        &self,
        ship: ShipDesignId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x4C);
        packet.header_mut().set_id0(ship.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Creates a new [`crate::unit::Sun`].
    #[inline]
    pub async fn create_sun(
        &self,
        cluster: ClusterId,
        config: &SunConfiguration,
    ) -> Result<(), GameError> {
        self.create_sun_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::Sun`].
    pub async fn create_sun_split(
        &self,
        cluster: ClusterId,
        config: &SunConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.create_unit_split(cluster, config).await
    }

    /// Creates a new [`crate::unit::BlackHole`].
    #[inline]
    pub async fn create_black_hole(
        &self,
        cluster: ClusterId,
        config: &BlackHoleConfiguration,
    ) -> Result<(), GameError> {
        self.create_black_hole_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::BlackHole`].
    #[inline]
    pub async fn create_black_hole_split(
        &self,
        cluster: ClusterId,
        config: &BlackHoleConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.create_unit_split(cluster, config).await
    }

    /// Creates a new [`crate::unit::Planet`].
    #[inline]
    pub async fn create_planet(
        &self,
        cluster: ClusterId,
        config: &PlanetConfiguration,
    ) -> Result<(), GameError> {
        self.create_planet_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::Planet`].
    #[inline]
    pub async fn create_planet_split(
        &self,
        cluster: ClusterId,
        config: &PlanetConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.create_unit_split(cluster, config).await
    }

    /// Creates a new [`crate::unit::Moon`].
    #[inline]
    pub async fn create_moon(
        &self,
        cluster: ClusterId,
        config: &MoonConfiguration,
    ) -> Result<(), GameError> {
        self.create_moon_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::Moon`].
    #[inline]
    pub async fn create_moon_split(
        &self,
        cluster: ClusterId,
        config: &MoonConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.create_unit_split(cluster, config).await
    }

    /// Creates a new [`crate::unit::Meteoroid`].
    #[inline]
    pub async fn create_meteoroid(
        &self,
        cluster: ClusterId,
        config: &MeteoroidConfiguration,
    ) -> Result<(), GameError> {
        self.create_meteoroid_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::Meteoroid`].
    #[inline]
    pub async fn create_meteoroid_split(
        &self,
        cluster: ClusterId,
        config: &MeteoroidConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.create_unit_split(cluster, config).await
    }

    /// Creates a new [`crate::unit::Buoy`].
    #[inline]
    pub async fn create_buoy(
        &self,
        cluster: ClusterId,
        config: &BuoyConfiguration,
    ) -> Result<(), GameError> {
        self.create_buoy_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::Buoy`].
    #[inline]
    pub async fn create_buoy_split(
        &self,
        cluster: ClusterId,
        config: &BuoyConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.create_unit_split(cluster, config).await
    }

    /// Creates a new [`crate::unit::Unit`].
    #[inline]
    pub async fn create_unit(
        &self,
        cluster: ClusterId,
        config: &dyn Configuration,
    ) -> Result<(), GameError> {
        self.create_unit_split(cluster, config).await?.await
    }

    /// Creates a new [`crate::unit::Unit`].
    pub async fn create_unit_split(
        &self,
        cluster: ClusterId,
        config: &dyn Configuration,
    ) -> Result<impl Future<Output = Result<(), GameError>> + 'static, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x51);
        packet.header_mut().set_id0(cluster.0);
        packet.header_mut().set_param0(config.kind().into());
        packet.write(|writer| config.write(writer));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Removes the [`crate::unit::Unit`] with the given name.
    #[inline]
    pub async fn remove_unit(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
        kind: UnitKind,
    ) -> Result<(), GameError> {
        self.remove_unit_split(cluster, name, kind).await?.await
    }

    /// Removes the [`crate::unit::Unit`] with the given name.
    pub async fn remove_unit_split(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
        kind: UnitKind,
    ) -> Result<impl Future<Output = Result<(), GameError>> + 'static, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x53);
        packet.header_mut().set_id0(cluster.0);
        packet.header_mut().set_param0(kind.into());
        packet.write(|writer| writer.write_string(name.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Retrieves the [`crate::Configuration`] of [`crate::unit::Unit`] with the given name.
    #[inline]
    pub async fn retrieve_unit_configuration<T: Configuration + Default>(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
        kind: UnitKind,
    ) -> Result<T, GameError> {
        self.retrieve_unit_configuration_split::<T>(cluster, name, kind)
            .await?
            .await
    }

    /// Retrieves the [`crate::Configuration`] of [`crate::unit::Unit`] with the given name.
    pub async fn retrieve_unit_configuration_split<T: Configuration + Default>(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
        kind: UnitKind,
    ) -> Result<impl Future<Output = Result<T, GameError>> + 'static, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x50);
        packet.header_mut().set_id0(cluster.0);
        packet.header_mut().set_param0(kind.into());
        packet.write(|writer| writer.write_string(name.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                Ok(packet.read(|reader| T::default().with_read(reader)))
            })
        })
    }

    /// Applies the given [`crate::Configuration`].
    #[inline]
    pub async fn configure_unit<T: Configuration + Default>(
        &self,
        cluster: ClusterId,
        name: &str,
        configuration: &T,
    ) -> Result<(), GameError> {
        self.configure_unit_split::<T>(cluster, name, configuration)
            .await?
            .await
    }

    /// Removes the [`crate::unit::Unit`] with the given name.
    pub async fn configure_unit_split<T: Configuration + Default>(
        &self,
        cluster: ClusterId,
        name: &str,
        configuration: &T,
    ) -> Result<impl Future<Output = Result<(), GameError>> + 'static, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x52);
        packet.header_mut().set_id0(cluster.0);
        packet.header_mut().set_param0(configuration.kind().into());
        packet.write(|writer| {
            writer.write_string(name);
            configuration.write(writer);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Registers a new ship with the given name and [`crate::hierarchy::ShipDesign`]. The name must
    /// obey naming conventions and the chosen design must have set `free_spawn`. All
    /// [`crate::hierarchy::ShipDesign`]s which don't have `free_spawn` set (=`false`) must be built
    /// in game and can't be just registered.
    #[inline]
    pub async fn register_ship(
        &self,
        name: impl AsRef<str>,
        design: ShipDesignId,
    ) -> Result<ControllableId, GameError> {
        self.register_ship_split(name, design).await?.await
    }

    /// Registers a new ship with the given name and [`crate::hierarchy::ShipDesign`]. The name must
    /// obey naming conventions and the chosen design must have set `free_spawn`. All
    /// [`crate::hierarchy::ShipDesign`]s which don't have `free_spawn` set (=`false`) must be built
    /// in game and can't be just registered.
    pub async fn register_ship_split(
        &self,
        name: impl AsRef<str>,
        design: ShipDesignId,
    ) -> Result<impl Future<Output = Result<ControllableId, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x30);
        packet.header_mut().set_id0(design.0);
        packet.write(|writer| writer.write_string(name.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |packet| Ok(ControllableId(packet.header().id0())))
        })
    }

    /// Sends a chat message with a maximum of 512 characters to the given [`crate::Player`].
    #[inline]
    pub async fn chat_player(
        &self,
        player: PlayerId,
        message: impl AsRef<str>,
    ) -> Result<(), GameError> {
        self.chat_player_split(player, message).await?.await
    }

    /// Sends a chat message with a maximum of 512 characters to the given [`crate::Player`].
    pub async fn chat_player_split(
        &self,
        player: PlayerId,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let message = check_message_or_err(message)?;

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x20);
        packet.header_mut().set_id0(player.0);
        packet.write(|writer| writer.write_string_without_len(message.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sends a chat message with a maximum of 512 characters to all players in the given
    /// [`crate::Team`].
    #[inline]
    pub async fn chat_team(&self, team: TeamId, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_team_split(team, message).await?.await
    }

    /// Sends a chat message with a maximum of 512 characters to all players in the given
    /// [`crate::Team`].
    pub async fn chat_team_split(
        &self,
        team: TeamId,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let message = check_message_or_err(message)?;

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x21);
        packet.header_mut().set_id0(team.0);
        packet.write(|writer| writer.write_string_without_len(message.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sends a chat message with a maximum of 512 characters to all players in the connected
    /// [`crate::hierarchy::Galaxy`].
    #[inline]
    pub async fn chat_galaxy(&self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_galaxy_split(message).await?.await
    }

    /// Sends a chat message with a maximum of 512 characters to all players in the connected
    /// [`crate::hierarchy::Galaxy`].
    pub async fn chat_galaxy_split(
        &self,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let message = check_message_or_err(message)?;

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x22);
        packet.write(|writer| writer.write_string_without_len(message.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    #[inline]
    pub async fn kill_controllable(&self, controllable: ControllableId) -> Result<(), GameError> {
        self.kill_controllable_split(controllable).await?.await
    }

    pub async fn kill_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x31);
        packet.header_mut().set_id0(controllable.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    #[inline]
    pub async fn continue_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.continue_controllable_split(controllable).await?.await
    }

    pub async fn continue_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x32);
        packet.header_mut().set_id0(controllable.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sets the thruster fand nozzle of the [`Controllable`] at the same time. Please note, that
    /// you need to stay within the limits of your ships configuration. A postive thruster value
    /// make your ship advance forward. A negative thruster value negatively. Usually a ship is
    /// designed to be faster when flying forward.
    #[inline]
    pub async fn set_controllable_thruster_nozzle(
        &self,
        controllable: ControllableId,
        thruster: f64,
        nozzle: f64,
    ) -> Result<(), GameError> {
        self.set_controllable_thruster_nozzle_split(controllable, thruster, nozzle)
            .await?
            .await
    }

    /// Sets the thruster fand nozzle of the [`Controllable`] at the same time. Please note, that
    /// you need to stay within the limits of your ships configuration. A postive thruster value
    /// make your ship advance forward. A negative thruster value negatively. Usually a ship is
    /// designed to be faster when flying forward.
    pub async fn set_controllable_thruster_nozzle_split(
        &self,
        controllable: ControllableId,
        thruster: f64,
        nozzle: f64,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if !thruster.is_finite() || !nozzle.is_finite() {
            return Err(GameError::from(0x31));
        }

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x36);
        packet.header_mut().set_id0(controllable.0);
        packet.write(|writer| {
            writer.write_double(thruster);
            writer.write_double(nozzle);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sets the nozzle of the [`ControllableId`]. Please note, that you need to stay within the
    /// limits of your ships configuration. A positive nozzle value will increase the
    /// direction-angle, a negative will decrease it.
    ///
    /// The nozzle value of `0` will stop the turning.
    #[inline]
    pub async fn set_controllable_nozzle(
        &self,
        controllable: ControllableId,
        nozzle: f64,
    ) -> Result<(), GameError> {
        self.set_controllable_nozzle_split(controllable, nozzle)
            .await?
            .await
    }

    /// Sets the nozzle of the [`ControllableId`]. Please note, that you need to stay within the
    /// limits of your ships configuration. A positive nozzle value will increase the
    /// direction-angle, a negative will decrease it.
    ///
    /// The nozzle value of `0` will stop the turning.
    pub async fn set_controllable_nozzle_split(
        &self,
        controllable: ControllableId,
        nozzle: f64,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if !nozzle.is_finite() {
            return Err(GameError::from(0x31));
        }

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x35);
        packet.header_mut().set_id0(controllable.0);
        packet.write(|writer| {
            writer.write_double(nozzle);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sets the thruster of the [`ControllableId`]. Please note, that you need to stay within the
    /// limits of your ships configuration. A positive thruster value will make your ship advance
    /// forward. A negative thruster value nagetively. Usually, a ship is deisgned to be faster when
    /// flying forward.
    #[inline]
    pub async fn set_controllable_thruster(
        &self,
        controllable: ControllableId,
        thruster: f64,
    ) -> Result<(), GameError> {
        self.set_controllable_thruster_split(controllable, thruster)
            .await?
            .await
    }

    /// Sets the thruster of the [`ControllableId`]. Please note, that you need to stay within the
    /// limits of your ships configuration. A positive thruster value will make your ship advance
    /// forward. A negative thruster value nagetively. Usually, a ship is deisgned to be faster when
    /// flying forward.
    pub async fn set_controllable_thruster_split(
        &self,
        controllable: ControllableId,
        thruster: f64,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if !thruster.is_finite() {
            return Err(GameError::from(0x31));
        }

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x31);
        packet.header_mut().set_id0(controllable.0);
        packet.write(|writer| {
            writer.write_double(thruster);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    #[inline]
    pub async fn unregister_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.unregister_controllable_split(controllable)
            .await?
            .await
    }

    pub async fn unregister_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x33);
        packet.header_mut().set_id0(controllable.0);

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    pub async fn send_packet_on_new_session(
        &self,
        mut packet: Packet,
    ) -> Result<Session, GameError> {
        let session = self
            .sessions
            .get()
            .ok_or(GameErrorKind::SessionIdsExhausted)?;

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

impl From<async_channel::RecvError> for GameError {
    #[inline]
    fn from(_: async_channel::RecvError) -> Self {
        GameErrorKind::ConnectionClosed.into()
    }
}
