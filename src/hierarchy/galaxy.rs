use crate::error::{GameError, GameErrorKind};
use crate::events::FlattiverseEvent;
use crate::hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, GalaxyConfig, Region, RegionId, ShipDesign,
    ShipDesignConfig, ShipDesignId, ShipUpgrade, ShipUpgradeId, TeamConfig,
};
use crate::hierarchy::{ClusterConfig, ClusterId};
use crate::network::{ConnectError, ConnectionEvent, ConnectionHandle, Packet};
use crate::player::Player;
use crate::team::Team;
use crate::unit::UnitKind;
use crate::{Controllable, ControllableId, PlayerId, PlayerKind, TeamId, UniversalHolder};
use num_enum::FromPrimitive;
use num_enum::TryFromPrimitive;
use std::future::Future;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct GalaxyId(pub(crate) u16);

#[derive(Debug)]
pub struct Galaxy {
    id: GalaxyId,
    config: GalaxyConfig,

    clusters: UniversalHolder<ClusterId, Cluster>,
    ship_designs: UniversalHolder<ShipDesignId, ShipDesign>,
    teams: UniversalHolder<TeamId, Team>,
    controllables: UniversalHolder<ControllableId, Controllable>,
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
            id: GalaxyId(0),
            config: GalaxyConfig::default(),

            clusters: UniversalHolder::with_capacity(256),
            ship_designs: UniversalHolder::with_capacity(256),
            teams: UniversalHolder::with_capacity(256),
            controllables: UniversalHolder::with_capacity(256),
            players: UniversalHolder::with_capacity(256),

            connection: handle,
            receiver,
            login_completed: false,
        })
    }

    pub async fn receive(&mut self) -> Result<FlattiverseEvent, GameError> {
        loop {
            match self.receiver.recv().await {
                None => return Err(GameErrorKind::ConnectionClosed.into()),
                Some(event) => {
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
                Err(TryRecvError::Disconnected) => {
                    return Err(GameErrorKind::ConnectionClosed.into());
                }
                Ok(event) => {
                    if let Some(event) = self.on_connection_event(event)? {
                        return Ok(Some(event));
                    }
                }
                Err(TryRecvError::Empty) => return Ok(None),
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
            info!(
                "Processing packet with command=0x{:02x}",
                packet.header().command()
            );
            match packet.header().command() {
                // galaxy created
                0x40 |
                // galaxy updated
                0x50 => {
                    self.update(packet);
                    Ok(Some(FlattiverseEvent::GalaxyUpdated(self.id)))
                },

                // cluster created
                0x41 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.get(cluster_id).is_none(), "{cluster_id:?} is already populated: {:?}", self.clusters.get(cluster_id));
                    self.clusters.set(
                        cluster_id,
                        packet.read(|reader| Cluster::new(cluster_id, self.id, self.connection.clone(), reader))
                    );
                    Ok(Some(FlattiverseEvent::ClusterCreated {
                        galaxy: self.id,
                        cluster: cluster_id,
                    }))
                },

                // cluster updated
                0x51 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated");
                    packet.read(|reader| self.clusters[cluster_id].update(reader));
                    Ok(Some(FlattiverseEvent::ClusterUpdated {
                        galaxy: self.id,
                        cluster: cluster_id,
                    }))
                }

                // cluster removed
                0x71 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated");
                    self.clusters[cluster_id].deactivate();
                    self.clusters.remove(cluster_id);
                    Ok(Some(FlattiverseEvent::ClusterRemoved {
                        galaxy: self.id,
                        cluster: cluster_id,
                    }))
                }

                // region created
                0x42 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let region_id = RegionId(packet.header().id1());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated");
                    debug_assert!(self.clusters[cluster_id].regions().get(region_id).is_none(), "{region_id:?} for {cluster_id:?} is already populated: {:?}", self.clusters[cluster_id].regions().get(region_id));
                    self.clusters[cluster_id].regions_mut().set(
                        region_id,
                        packet.read(|reader| Region::new(self.id, cluster_id, region_id, self.connection.clone(), reader))
                    );
                    Ok(Some(FlattiverseEvent::RegionCreated {
                        galaxy: self.id,
                        cluster: cluster_id,
                        region: region_id
                    }))
                }

                //  region updated
                0x52 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let region_id = RegionId(packet.header().id1());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated.");
                    debug_assert!(self.clusters[cluster_id].regions().get(region_id).is_some(), "{region_id:?} for {cluster_id:?} is not populated.");
                    packet.read(|reader| self.clusters[cluster_id][region_id].update(reader));
                    Ok(Some(FlattiverseEvent::RegionUpdated {
                        galaxy: self.id,
                        cluster: cluster_id,
                        region: region_id,
                    }))
                }

                //  region removed
                0x72 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let region_id = RegionId(packet.header().id1());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated.");
                    debug_assert!(self.clusters[cluster_id].regions().get(region_id).is_some(), "{region_id:?} for {cluster_id:?} is not populated.");
                    self.clusters[cluster_id][region_id].deactivate();
                    self.clusters[cluster_id].regions_mut().remove(region_id);
                    Ok(Some(FlattiverseEvent::RegionRemoved {
                        galaxy: self.id,
                        cluster: cluster_id,
                        region: region_id,
                    }))
                }

                // team created
                0x43 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.get(team_id).is_none(), "{team_id:?} is already populated: {:?}", self.teams.get(team_id));
                    self.teams.set(team_id, packet.read(|reader| Team::new(
                        team_id,
                        self.connection.clone(),
                        reader
                    )));
                    Ok(Some(FlattiverseEvent::TeamCreated {
                        galaxy: self.id,
                        team: team_id
                    }))
                }

                // team updated
                0x53 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.get(team_id).is_some(), "{team_id:?} is not populated.");
                    packet.read(|reader| self.teams[team_id].update(reader));
                    Ok(Some(FlattiverseEvent::TeamUpdated {
                        galaxy: self.id,
                        team: team_id,
                    }))
                }

                // team dynamic update (score of the team updated)
                0x63 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.get(team_id).is_some(), "{team_id:?} is not populated.");
                    packet.read(|reader| self.teams[team_id].dynamic_update(reader));
                    Ok(Some(FlattiverseEvent::TeamUpdated {
                        galaxy: self.id,
                        team: team_id,
                    }))
                }

                // team removed
                0x73 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.get(team_id).is_some(), "{team_id:?} is not populated.");
                    self.teams[team_id].deactivate();
                    self.teams.remove(team_id);
                    Ok(Some(FlattiverseEvent::TeamRemoved {
                        galaxy: self.id,
                        team: team_id,
                    }))
                }

                // ship design created
                0x44 => {
                    let ship_design_id = ShipDesignId(packet.header().id0());
                    debug_assert!(self.ship_designs.get(ship_design_id).is_none(), "{ship_design_id:?} is already populated: {:?}", self.ship_designs.get(ship_design_id));
                    self.ship_designs.set(
                        ship_design_id,
                        packet.read(|reader| ShipDesign::new(ship_design_id, self.id, self.connection.clone(), reader))
                    );
                    Ok(Some(FlattiverseEvent::ShipDesignCreated {
                        galaxy: self.id,
                        ship_design: ship_design_id,
                    }))
                }

                // ship design updated
                0x54 => {
                    todo!()
                }

                // ship design removed
                0x74 => {
                    todo!()
                }

                // ship upgrade created
                0x45 => {
                    let upgrade_id = ShipUpgradeId(packet.header().id0());
                    let ship_design_id = ShipDesignId(packet.header().id1());
                    debug_assert!(self.ship_designs.get(ship_design_id).is_some(), "{ship_design_id:?} is not populated");
                    debug_assert!(self.ship_designs[ship_design_id].upgrades().get(upgrade_id).is_none(), "{upgrade_id:?} for {ship_design_id:?} is already populated: {:?}", self.ship_designs[ship_design_id].upgrades().get(upgrade_id));
                    self.ship_designs[ship_design_id].upgrades_mut().set(
                        upgrade_id,
                        packet.read(|reader| ShipUpgrade::new(upgrade_id, self.id, ship_design_id, self.connection.clone(), reader))
                    );
                    Ok(Some(FlattiverseEvent::UpgradeUpdated {
                        galaxy: self.id,
                        ship: ship_design_id,
                        upgrade: upgrade_id
                    }))
                }

                // ship upgrade updated
                0x55 => {
                    todo!()
                }

                // ship upgrade removed
                0x75 => {
                    todo!()
                }

                // player created
                0x46 => {
                    let player_id = PlayerId(packet.header().id0());
                    let team_id = TeamId(packet.header().id1());
                    let player_kind = PlayerKind::from_primitive(packet.header().param0());
                    debug_assert!(self.players.get(player_id).is_none(), "{player_id:?} is already populated: {:?}", self.players.get(player_id));
                    debug_assert!(self.teams.get(team_id).is_some(), "{team_id:?} is not populated.");
                    self.players.set(
                        player_id,
                        packet.read(|reader| Player::new(player_id, player_kind, team_id, reader))
                    );
                    Ok(Some(FlattiverseEvent::PlayerJoined {
                        galaxy: self.id,
                        player: player_id,
                    }))
                }

                // player dynamic update (score)
                0x66 => {
                    todo!()
                }

                // player removed
                0x76 => {
                    let player_id = PlayerId(packet.header().id0());
                    debug_assert!(self.players.get(player_id).is_some(), "{player_id:?} is not populated.");
                    self.players[player_id].deactivate();
                    self.players.remove(player_id);
                    Ok(Some(FlattiverseEvent::PlayerParted {
                        galaxy: self.id,
                        player: player_id,
                    }))
                }

                // controllable info created
                0x47 => {
                    let player_id = PlayerId(packet.header().id0());
                    let controllable_info_id = ControllableInfoId(packet.header().id1());
                    let reduced = packet.header().param0() == 1;
                    debug_assert!(self.players.get(player_id).is_some(), "{player_id:?} is not populated.");
                    debug_assert!(self.players[player_id].controllables_info().get(controllable_info_id).is_none(), "{controllable_info_id:?} for {player_id:?} is already populated: {:?}", self.players[player_id].controllables_info().get(controllable_info_id));
                    self.players[player_id].controllables_info_mut().set(
                        controllable_info_id,
                        packet.read(|reader| ControllableInfo::new(
                            self.id,
                            controllable_info_id,
                            player_id,
                            reader,
                            reduced
                        ))
                    );
                    Ok(Some(FlattiverseEvent::ControllableInfoCreated {
                        galaxy: self.id,
                        player: player_id,
                        controllable_info: controllable_info_id,
                    }))
                }

                // controllable info updated (live)
                0x57 => {
                    warn!("Not implemented: Controllable Info Updated");
                    Ok(None)
                }

                // controllable info dynamic update (scores)
                0x67 => {
                    let player_id = PlayerId(packet.header().id0());
                    let controllable_info_id = ControllableInfoId(packet.header().id1());
                    let reduced = packet.header().param0() == 1;
                    debug_assert!(self.players.get(player_id).is_some(), "{player_id:?} is not populated.");
                    debug_assert!(self.players[player_id].controllables_info().get(controllable_info_id).is_some(), "{controllable_info_id:?} for {player_id:?} is not populated.");
                    packet.read(|reader| self.players[player_id][controllable_info_id].dynamic_update(reader, reduced));
                    Ok(Some(FlattiverseEvent::ControllableInfoUpdated {
                        galaxy: self.id,
                        player: player_id,
                        controllable_info: controllable_info_id,
                    }))
                }

                // controllable info removed
                0x77 => {
                    let player_id = PlayerId(packet.header().id0());
                    let controllable_info_id = ControllableInfoId(packet.header().id1());
                    debug_assert!(self.players.get(player_id).is_some(), "{player_id:?} is not populated.");
                    debug_assert!(self.players[player_id].controllables_info().get(controllable_info_id).is_some(), "{controllable_info_id:?} is not populated.");
                    self.players[player_id][controllable_info_id].deactivate();
                    self.players[player_id].controllables_info_mut().remove(controllable_info_id);
                    Ok(Some(FlattiverseEvent::ControllableInfoRemoved {
                        galaxy: self.id,
                        player: player_id,
                        controllable_info: controllable_info_id
                    }))
                }

                // controllable created
                0x48 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.get(controllable_id).is_none(), "{controllable_id:?} is already populated: {:?}", self.controllables.get(controllable_id));
                    self.controllables.set(
                        controllable_id,
                        packet.read(|reader| Controllable::new(self.id, controllable_id, reader, self.connection.clone()))
                    );
                    Ok(Some(FlattiverseEvent::ControllableJoined {
                        galaxy: self.id,
                        controllable: controllable_id
                    }))
                }

                // controllable update: list of configured upgrades, change of base-data (max_hull, ...)
                0x58 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.get(controllable_id).is_some(), "{controllable_id:?} is not populated.");
                    packet.read(|reader| self.controllables[controllable_id].update(reader));
                    Ok(Some(FlattiverseEvent::ControllableUpdated {
                        galaxy: self.id,
                        controllable: controllable_id
                    }))
                }

                // controllable dynamics update: position, movement, energy, hull...
                0x68 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.get(controllable_id).is_some(), "{controllable_id:?} is not populated.");
                    packet.read(|reader| self.controllables[controllable_id].dynamic_update(reader));
                    Ok(Some(FlattiverseEvent::ControllableUpdated {
                        galaxy: self.id,
                        controllable: controllable_id
                    }))
                }

                // controllable removed
                0x78 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.get(controllable_id).is_some(), "{controllable_id:?} is not populated.");
                    self.controllables[controllable_id].deactivate();
                    self.controllables.remove(controllable_id);
                    Ok(Some(FlattiverseEvent::ControllableRemoved {
                        galaxy: self.id,
                        controllable: controllable_id
                    }))
                }

                // we see a new unit which we didn't see before
                0x1c => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let unit_kind = UnitKind::try_from_primitive(packet.header().param0())
                        .expect("Unknown UnitKind ");
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated");
                    packet
                        .read(|reader| self.clusters[cluster_id].see_new_unit(unit_kind, reader))
                        .map(Some)
                }

                // a unit we see has been updated.
                0x1D => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated");
                    packet.read(|reader| self.clusters[cluster_id].see_update_unit(reader))
                }

                // a once known unit vanished.
                0x1E => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.get(cluster_id).is_some(), "{cluster_id:?} is not populated");
                    packet.read(|reader| self.clusters[cluster_id].see_unit_no_more(reader.read_string()))
                }

                // tick completed
                0x20 => {
                    self.login_completed = true;
                    Ok(Some(FlattiverseEvent::TickCompleted))
                }

                cmd => Err(
                    GameError::from(GameErrorKind::Unspecified(0)).with_info(format!(
                        "Unexpected command=0x{cmd:02x} for {:?}, header={:?}",
                        self.id,
                        packet.header()
                    )),
                ),
            }
        }
    }

    fn update(&mut self, mut packet: Packet) {
        self.id = GalaxyId(packet.header().param());
        packet.read(|reader| self.config.read(reader));
    }

    /// Waits until the login proceedure has been completed for  this [`Galaxy`].
    pub async fn wait_login_completed(&mut self) -> Result<(), GameError> {
        while !self.login_completed {
            let event = self.receive().await?;
            debug!("{event:?}");
            if let FlattiverseEvent::TickCompleted = event {
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

    /// Creates a [`ShipDesign`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_ship_design`].
    #[inline]
    pub async fn create_ship_design(
        &self,
        config: &ShipDesignConfig,
    ) -> Result<impl Future<Output = Result<ShipDesignId, GameError>>, GameError> {
        self.connection.create_ship_design_split(config).await
    }

    /// Registers a new ship with the given name and [`crate::hierarchy::ShipDesign`]. The name must
    /// obey naming conventions and the chosen design must have set `free_spawn`. All
    /// [`crate::hierarchy::ShipDesign`]s which don't have `free_spawn` set (=`false`) must be built
    /// in game and can't be just registered.
    /// See also [`ConnectionHandle::register_ship`].
    #[inline]
    pub async fn register_ship(
        &self,
        name: impl Into<String>,
        design: ShipDesignId,
    ) -> Result<impl Future<Output = Result<ControllableId, GameError>>, GameError> {
        let name = name.into();
        self.connection.register_ship_split(name, design).await
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
    pub fn ship_designs(&self) -> &UniversalHolder<ShipDesignId, ShipDesign> {
        &self.ship_designs
    }

    #[inline]
    pub fn teams(&self) -> &UniversalHolder<TeamId, Team> {
        &self.teams
    }

    #[inline]
    pub fn controllables(&self) -> &UniversalHolder<ControllableId, Controllable> {
        &self.controllables
    }

    #[inline]
    pub fn players(&self) -> &UniversalHolder<PlayerId, Player> {
        &self.players
    }

    #[inline]
    pub fn connection(&self) -> &ConnectionHandle {
        &self.connection
    }
}
