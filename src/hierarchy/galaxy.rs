use crate::error::{GameError, GameErrorKind};
use crate::events::FlattiverseEvent;
use crate::hierarchy::{Cluster, GalaxyConfig, RegionId, ShipConfig, TeamConfig};
use crate::hierarchy::{ClusterConfig, ClusterId};
use crate::network::{ConnectError, ConnectionEvent, ConnectionHandle, Packet};
use crate::player::Player;
use crate::team::Team;
use crate::unit::{Ship, ShipId};
use crate::{PlayerId, PlayerKind, TeamId, UniversalHolder, UpgradeId};
use async_channel::Receiver;
use num_enum::FromPrimitive;
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct GlaxyId(pub(crate) u16);

#[derive(Debug)]
pub struct Galaxy {
    id: GlaxyId,
    config: GalaxyConfig,

    clusters: UniversalHolder<ClusterId, Cluster>,
    ships: UniversalHolder<ShipId, Ship>,
    teams: UniversalHolder<TeamId, Team>,
    players: UniversalHolder<PlayerId, Player>,

    //
    connection: ConnectionHandle,
    receiver: Receiver<ConnectionEvent>,
    login_completed: bool,
}

impl Galaxy {
    pub(crate) async fn join(uri: &str, auth: &str, team: u8) -> Result<Self, GameError> {
        let connection = crate::network::connect(uri, auth, team)
            .await
            .map_err(|e| match e {
                ConnectError::GameError(error) => error,
                e => GameError::from(GameErrorKind::GenericException)
                    .with_info(format!("Failed to connect due to local issues: {e}")),
            })?;
        let (handle, receiver) = connection.spawn();

        Ok(Self {
            id: GlaxyId(0),
            config: GalaxyConfig::default(),

            clusters: UniversalHolder::with_capacity(256),
            ships: UniversalHolder::with_capacity(256),
            teams: UniversalHolder::with_capacity(256),
            players: UniversalHolder::with_capacity(256),

            connection: handle,
            receiver,
            login_completed: false,
        })
    }

    pub async fn receive(&mut self) -> Result<FlattiverseEvent, GameError> {
        loop {
            match self.receiver.recv().await {
                Err(_) => return Err(GameErrorKind::ConnectionClosed.into()),
                Ok(event) => {
                    if let Some(event) = self.on_connection_event(event)? {
                        return Ok(event);
                    }
                }
            }
        }
    }

    pub fn poll_receive(&mut self) -> Result<Option<FlattiverseEvent>, GameError> {
        loop {
            match self.receiver.try_recv() {
                Err(e) if e.is_closed() => return Err(GameErrorKind::ConnectionClosed.into()),
                Ok(event) => {
                    if let Some(event) = self.on_connection_event(event)? {
                        return Ok(Some(event));
                    }
                }
                _ => return Ok(None),
            }
        }
    }

    fn on_connection_event(
        &mut self,
        event: ConnectionEvent,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        match event {
            ConnectionEvent::PingMeasured(ping) => Ok(Some(FlattiverseEvent::PingMeasured(ping))),
            ConnectionEvent::Packet(packet) => self.on_packet(packet),
            ConnectionEvent::GameError(e) => Err(e),
            ConnectionEvent::Closed(reason) => {
                Err(GameError::from(GameErrorKind::ConnectionClosed).with_info_opt(reason))
            }
        }
    }

    fn on_packet(&mut self, mut packet: Packet) -> Result<Option<FlattiverseEvent>, GameError> {
        if packet.header().session() != 0 {
            Err(GameError::from(GameErrorKind::Unspecified(0))
                .with_info("At this point, no session specific packet should be handled"))
        } else {
            match packet.header().command() {
                // galaxy info
                0x10 => {
                    self.id = GlaxyId(packet.header().param());
                    packet.read(|reader| self.config.read(reader));
                    Ok(Some(FlattiverseEvent::GalaxyUpdated(self.id)))
                }
                // cluster info
                0x11 => {
                    let cluster_id = ClusterId(packet.header().param0());
                    self.clusters.set(
                        cluster_id,
                        packet.read(|reader| {
                            Cluster::new(cluster_id, self.id, self.connection.clone(), reader)
                        }),
                    );
                    Ok(Some(FlattiverseEvent::ClusterUpdated {
                        galaxy: self.id,
                        cluster: cluster_id,
                    }))
                }

                // region info
                0x12 => {
                    let cluster_id = ClusterId(packet.header().param0());
                    let region_id = RegionId(packet.header().param1());
                    packet.read(|reader| {
                        self.clusters[cluster_id].read_region(region_id, reader);
                    });
                    Ok(Some(FlattiverseEvent::RegionUpdated {
                        galaxy: self.id,
                        cluster: cluster_id,
                        region: region_id,
                    }))
                }

                // team info
                0x13 => {
                    let team_id = TeamId(packet.header().param0());
                    self.teams.set(
                        team_id,
                        packet.read(|reader| Team::new(team_id, self.connection.clone(), reader)),
                    );
                    Ok(Some(FlattiverseEvent::TeamUpdated {
                        galaxy: self.id,
                        team: team_id,
                    }))
                }

                // ship info
                0x14 => {
                    let ship_id = ShipId(packet.header().param0());
                    self.ships.set(
                        ship_id,
                        packet.read(|reader| {
                            Ship::new(ship_id, self.id, self.connection.clone(), reader)
                        }),
                    );
                    Ok(Some(FlattiverseEvent::ShipUpdated {
                        galaxy: self.id,
                        ship: ship_id,
                    }))
                }

                // upgrade info
                0x15 => {
                    let upgrade_id = UpgradeId(packet.header().param0());
                    let ship_id = ShipId(packet.header().param1());
                    packet.read(|reader| {
                        self.ships[ship_id].read_upgrade(upgrade_id, reader);
                    });
                    Ok(Some(FlattiverseEvent::UpgradeUpdated {
                        galaxy: self.id,
                        ship: ship_id,
                        upgrade: upgrade_id,
                    }))
                }

                // new player joined info
                0x16 => {
                    let player_id = PlayerId(packet.header().player());
                    let team_id = TeamId(packet.header().param1());
                    let player_kind = PlayerKind::from_primitive(packet.header().param0());
                    packet.read(|reader| {
                        self.players.set(
                            player_id,
                            Player::new(player_id, player_kind, team_id, reader),
                        );
                    });
                    Ok(Some(FlattiverseEvent::PlayerUpdated {
                        galaxy: self.id,
                        player: player_id,
                    }))
                }

                // tick completed
                0x20 => {
                    self.login_completed = true;
                    Ok(Some(FlattiverseEvent::TickCompleted))
                }

                cmd => Err(
                    GameError::from(GameErrorKind::Unspecified(0)).with_info(format!(
                        "Unexpected command={cmd} for {:?}, header={:?}",
                        self.id,
                        packet.header()
                    )),
                ),
            }
        }
    }

    /// Waits until the login proceedure has been completed for  this [`Galaxy`].
    pub async fn wait_login_completed(&mut self) -> Result<(), GameError> {
        while !self.login_completed {
            if let FlattiverseEvent::TickCompleted = self.receive().await? {
                break;
            }
        }
        Ok(())
    }

    /// Waits for the next [`FlattiverseEvent::TickCompleted`] for this [`Galaxy`].
    pub async fn wait_next_turn(&mut self) -> Result<(), GameError> {
        loop {
            if let FlattiverseEvent::TickCompleted = self.receive().await? {
                break;
            }
        }
        Ok(())
    }

    /// Sets the given values for this [`Galaxy`].
    /// See also [`ConnectionHandle::configure_galaxy`].
    #[inline]
    pub async fn configure(
        &self,
        config: &GalaxyConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_galaxy_split(self.id, config)
            .await
    }

    /// Creates a [`Cluster`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_cluster`].
    #[inline]
    pub async fn create_cluster(
        &self,
        config: &ClusterConfig,
    ) -> Result<impl Future<Output = Result<ClusterId, GameError>>, GameError> {
        self.connection.create_cluster_split(config).await
    }

    /// Creates a [`Team`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_team`].
    #[inline]
    pub async fn create_team(
        &self,
        config: &TeamConfig,
    ) -> Result<impl Future<Output = Result<TeamId, GameError>>, GameError> {
        self.connection.create_team_split(config).await
    }

    /// Creates a [`Ship`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_ship`].
    #[inline]
    pub async fn create_ship(
        &self,
        config: &ShipConfig,
    ) -> Result<impl Future<Output = Result<ShipId, GameError>>, GameError> {
        self.connection.create_ship_split(config).await
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.config.name
    }

    #[inline]
    pub fn config(&self) -> &GalaxyConfig {
        &self.config
    }

    #[inline]
    pub fn clusters(&self) -> &UniversalHolder<ClusterId, Cluster> {
        &self.clusters
    }

    #[inline]
    pub fn ships(&self) -> &UniversalHolder<ShipId, Ship> {
        &self.ships
    }

    #[inline]
    pub fn teams(&self) -> &UniversalHolder<TeamId, Team> {
        &self.teams
    }

    #[inline]
    pub fn players(&self) -> &UniversalHolder<PlayerId, Player> {
        &self.players
    }

    #[inline]
    pub fn connection(&self) -> &ConnectionHandle {
        &self.connection
    }

    #[inline]
    pub fn receiver_queue_len(&self) -> usize {
        self.receiver.len()
    }
}
