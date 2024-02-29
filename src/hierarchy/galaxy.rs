use crate::atomics::Atomic;
use crate::error::{GameError, GameErrorKind};
use crate::events::FlattiverseEvent;
use crate::hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, GalaxyConfig, Region, RegionId, ShipDesign,
    ShipDesignConfig, ShipDesignId, ShipUpgrade, ShipUpgradeId, TeamConfig,
};
use crate::hierarchy::{ClusterConfig, ClusterId};
use crate::network::{ConnectError, ConnectionHandle, Packet};
use crate::player::Player;
use crate::team::Team;
use crate::unit::UnitKind;
use crate::{Controllable, ControllableId, PlayerId, PlayerKind, TeamId, UniversalArcHolder};
use arc_swap::ArcSwap;
use async_channel::{Receiver, TryRecvError};
use num_enum::FromPrimitive;
use num_enum::TryFromPrimitive;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct GalaxyId(pub(crate) u16);

#[derive(Debug)]
pub struct Galaxy {
    pub(crate) id: Atomic<GalaxyId>,
    pub(crate) config: ArcSwap<GalaxyConfig>,

    pub(crate) clusters: UniversalArcHolder<ClusterId, Cluster>,
    pub(crate) ship_designs: UniversalArcHolder<ShipDesignId, ShipDesign>,
    pub(crate) teams: UniversalArcHolder<TeamId, Team>,
    pub(crate) controllables: UniversalArcHolder<ControllableId, Controllable>,
    pub(crate) players: UniversalArcHolder<PlayerId, Player>,

    //
    pub(crate) login_completed: Atomic<bool>,
    pub(crate) connection: ConnectionHandle,
    pub(crate) events: Receiver<FlattiverseEvent>,
}

impl Galaxy {
    pub(crate) async fn join(uri: &str, auth: &str, team: u8) -> Result<Arc<Self>, GameError> {
        let connection = crate::network::connect(uri, auth, team)
            .await
            .map_err(|e| match e {
                ConnectError::GameError(error) => error,
                e => GameError::from(GameErrorKind::GenericException)
                    .with_info(format!("Failed to connect due to local issues: {e}")),
            })?;

        Ok(connection.spawn())
    }

    pub async fn next_event(&self) -> Result<FlattiverseEvent, GameError> {
        self.events
            .recv()
            .await
            .map_err(|_| GameErrorKind::ConnectionClosed.into())
    }

    pub fn poll_next_event(&self) -> Result<Option<FlattiverseEvent>, GameError> {
        match self.events.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Closed) => Err(GameErrorKind::ConnectionClosed.into()),
        }
    }

    pub(crate) fn on_packet(
        self: &Arc<Self>,
        mut packet: Packet,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        if packet.header().session() != 0 {
            Err(GameError::from(GameErrorKind::Unspecified(0))
                .with_info("At this point, no session specific packet should be handled"))
        } else {
            info!(
                "Processing packet with command=0x{:02x}",
                packet.header().command()
            );
            match packet.header().command() {
                // message to player
                0x30 => {
                    let player_id = PlayerId(packet.header().id0());
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::PlayerChatMessageReceived {
                        time: crate::runtime::now(),
                        player: self.players.get(player_id),
                        message: packet.read(|reader| reader.read_remaining_as_string()),
                    }))
                }
                // message to team
                0x31 => {
                    let player_id = PlayerId(packet.header().id0());
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::TeamChatMessageReceived {
                        time: crate::runtime::now(),
                        player: self.players.get(player_id),
                        message: packet.read(|reader| reader.read_remaining_as_string()),
                    }))
                }
                // message to galaxy
                0x32 => {
                    let player_id = PlayerId(packet.header().id0());
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::GalaxyChatMessageReceived {
                        time: crate::runtime::now(),
                        player: self.players.get(player_id),
                        message: packet.read(|reader| reader.read_remaining_as_string()),
                    }))
                }

                // galaxy created
                0x40 |
                // galaxy updated
                0x50 => {
                    self.update(packet);
                    Ok(Some(FlattiverseEvent::GalaxyUpdated{
                        galaxy: Arc::clone(self)
                    }))
                },

                // cluster created
                0x41 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.has_not(cluster_id), "{cluster_id:?} is already populated: {:?}", self.clusters.get(cluster_id));
                    Ok(Some(FlattiverseEvent::ClusterCreated {
                        cluster: self.clusters.populate(packet.read(|reader| {
                            Cluster::new(
                                Arc::clone(self),
                                cluster_id,
                                reader
                            )
                        })),
                    }))
                },

                // cluster updated
                0x51 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated");
                    Ok(Some(FlattiverseEvent::ClusterUpdated {
                        cluster: packet.read(|reader| {
                            let cluster = self.clusters.get(cluster_id);
                            cluster.update(reader);
                            cluster
                        }),
                    }))
                }

                // cluster removed
                0x71 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated");
                    Ok(Some(FlattiverseEvent::ClusterRemoved {
                        cluster: {
                            let cluster = self.clusters.remove(cluster_id);
                            cluster.deactivate();
                            cluster
                        },
                    }))
                }

                // region created
                0x42 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let region_id = RegionId(packet.header().id1());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated");
                    debug_assert!(self.clusters.get(cluster_id).regions().has_not(region_id), "{region_id:?} for {cluster_id:?} is already populated: {:?}", self.clusters.get(cluster_id).regions().get(region_id));
                    let cluster = self.clusters.get(cluster_id);
                    Ok(Some(FlattiverseEvent::RegionCreated {
                        region: cluster.regions().populate(packet.read(|reader| {
                            Region::new(
                                Arc::clone(self),
                                Arc::clone(&cluster),
                                region_id,
                                reader
                            )
                        })),
                    }))
                }

                //  region updated
                0x52 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let region_id = RegionId(packet.header().id1());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated.");
                    debug_assert!(self.clusters.get(cluster_id).regions().has(region_id), "{region_id:?} for {cluster_id:?} is not populated.");
                    let cluster = self.clusters.get(cluster_id);
                    Ok(Some(FlattiverseEvent::RegionUpdated {
                        region: packet.read(|reader| {
                            let region = cluster.regions().get(region_id);
                            region.update(reader);
                            region
                        }),
                    }))
                }

                //  region removed
                0x72 => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let region_id = RegionId(packet.header().id1());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated.");
                    debug_assert!(self.clusters.get(cluster_id).regions().has(region_id), "{region_id:?} for {cluster_id:?} is not populated.");
                    let cluster = self.clusters.get(cluster_id);
                    Ok(Some(FlattiverseEvent::RegionRemoved {
                        region: {
                            let region = cluster.regions().remove(region_id);
                            region.deactivate();
                            region
                        },
                    }))
                }

                // team created
                0x43 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.has_not(team_id), "{team_id:?} is already populated: {:?}", self.teams.get_opt(team_id));
                    Ok(Some(FlattiverseEvent::TeamCreated {
                        team: self.teams.populate(packet.read(|reader| Team::new(
                            Arc::clone(self),
                            team_id,
                            reader
                        )))
                    }))
                }

                // team updated
                0x53 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.has(team_id), "{team_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::TeamUpdated {
                        team: packet.read(|reader| {
                            let team = self.teams.get(team_id);
                            team.update(reader);
                            team
                        }),
                    }))
                }

                // team dynamic update (score of the team updated)
                0x63 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.has(team_id), "{team_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::TeamUpdated {
                        team: packet.read(|reader| {
                            let team = self.teams.get(team_id);
                            team.dynamic_update(reader);
                            team
                        }),
                    }))
                }

                // team removed
                0x73 => {
                    let team_id = TeamId(packet.header().id0());
                    debug_assert!(self.teams.has(team_id), "{team_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::TeamRemoved {
                        team: {
                            let team = self.teams.remove(team_id);
                            team.deactivate();
                            team
                        },
                    }))
                }

                // ship design created
                0x44 => {
                    let ship_design_id = ShipDesignId(packet.header().id0());
                    debug_assert!(self.ship_designs.has_not(ship_design_id), "{ship_design_id:?} is already populated: {:?}", self.ship_designs.get(ship_design_id));
                    Ok(Some(FlattiverseEvent::ShipDesignCreated {
                        ship_design: self.ship_designs.populate(packet.read(|reader| {
                            ShipDesign::new(
                                Arc::clone(&self),
                                ship_design_id,
                                reader
                            )
                        })),
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
                    debug_assert!(self.ship_designs.has(ship_design_id), "{ship_design_id:?} is not populated");
                    debug_assert!(self.ship_designs.get(ship_design_id).upgrades().has_not(upgrade_id), "{upgrade_id:?} for {ship_design_id:?} is already populated: {:?}", self.ship_designs.get(ship_design_id).upgrades().get(upgrade_id));
                    let ship = self.ship_designs.get(ship_design_id);
                    Ok(Some(FlattiverseEvent::UpgradeUpdated {
                        upgrade: ship.upgrades().populate(packet.read(|reader| {
                            ShipUpgrade::new(
                                Arc::clone(&self),
                                Arc::clone(&ship),
                                upgrade_id,
                                reader
                            )
                        })),
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
                    debug_assert!(self.players.has_not(player_id), "{player_id:?} is already populated: {:?}", self.players.get(player_id));
                    debug_assert!(self.teams.has(team_id), "{team_id:?} is not populated.");
                    let team = self.teams.get(team_id);
                    Ok(Some(FlattiverseEvent::PlayerJoined {
                        player: packet.read(|reader| {
                            self.players.populate(Player::new(Arc::clone(&self), player_id, player_kind, team, reader))
                        }),
                    }))
                }

                // player dynamic update (score)
                0x66 => {
                    todo!()
                }

                // player removed
                0x76 => {
                    let player_id = PlayerId(packet.header().id0());
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    Ok(Some(FlattiverseEvent::PlayerParted {
                        player: {
                            let player = self.players.remove(player_id);
                            player.deactivate();
                            player
                        },
                    }))
                }

                // controllable info created
                0x47 => {
                    let player_id = PlayerId(packet.header().id0());
                    let controllable_info_id = ControllableInfoId(packet.header().id1());
                    let reduced = packet.header().param0() == 1;
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    debug_assert!(self.players.get(player_id).controllable_info().has(controllable_info_id), "{controllable_info_id:?} for {player_id:?} is already populated: {:?}", self.players.get(player_id).controllable_info().get(controllable_info_id));
                    let player = self.players.get(player_id);
                    Ok(Some(FlattiverseEvent::ControllableInfoCreated {
                        controllable_info: player.controllable_info().populate(packet.read(|reader| {
                            ControllableInfo::new(
                                Arc::clone(self),
                                controllable_info_id,
                                Arc::clone(&player),
                                reader,
                                reduced
                            )
                        })),
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
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    debug_assert!(self.players.get(player_id).controllable_info().has(controllable_info_id), "{controllable_info_id:?} for {player_id:?} is not populated.");
                    let player = self.players.get(player_id);
                    let controllable_info = player.controllable_info().get(controllable_info_id);
                    Ok(Some(FlattiverseEvent::ControllableInfoUpdated {
                        controllable_info: packet.read(|reader| {
                            controllable_info.dynamic_update(reader, reduced);
                            controllable_info
                        }),
                    }))
                }

                // controllable info removed
                0x77 => {
                    warn!("controllable info removed: {:?}", packet.header());
                    let player_id = PlayerId(packet.header().id0());
                    let controllable_info_id = ControllableInfoId(packet.header().id1());
                    debug_assert!(self.players.has(player_id), "{player_id:?} is not populated.");
                    debug_assert!(self.players.get(player_id).controllable_info().has(controllable_info_id), "{controllable_info_id:?} is not populated.");
                    let player = self.players.get(player_id);
                    let controllable_info = player.controllable_info().remove(controllable_info_id);
                    Ok(Some(FlattiverseEvent::ControllableInfoRemoved {
                        controllable_info: {
                            controllable_info.deactivate();
                            controllable_info
                        }
                    }))
                }

                // controllable created
                0x48 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.has_not(controllable_id), "{controllable_id:?} is already populated: {:?}", self.controllables.get_opt(controllable_id));
                    Ok(Some(FlattiverseEvent::ControllableJoined {
                        controllable: self.controllables.populate(packet.read(|reader|
                            Controllable::new(Arc::clone(self), controllable_id, reader)
                        ))
                    }))
                }

                // controllable update: list of configured upgrades, change of base-data (max_hull, ...)
                0x58 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.has(controllable_id), "{controllable_id:?} is not populated.");
                    let controllable = self.controllables.get(controllable_id);
                    packet.read(|reader| controllable.update(reader));
                    Ok(Some(FlattiverseEvent::ControllableUpdated {
                        controllable
                    }))
                }

                // controllable dynamics update: position, movement, energy, hull...
                0x68 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.has(controllable_id), "{controllable_id:?} is not populated.");
                    let controllable = self.controllables.get(controllable_id);
                    packet.read(|reader| controllable.dynamic_update(reader));
                    Ok(Some(FlattiverseEvent::ControllableUpdated {
                        controllable
                    }))
                }

                // controllable removed
                0x78 => {
                    let controllable_id = ControllableId(packet.header().id0());
                    debug_assert!(self.controllables.has(controllable_id), "{controllable_id:?} is not populated.");
                    let controllable = self.controllables.remove(controllable_id);
                    controllable.deactivate();
                    Ok(Some(FlattiverseEvent::ControllableRemoved {
                        controllable
                    }))
                }

                // we see a new unit which we didn't see before
                0x1c => {
                    let cluster_id = ClusterId(packet.header().id0());
                    let unit_kind = UnitKind::try_from_primitive(packet.header().param0())
                        .expect("Unknown UnitKind ");
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated");
                    packet
                        .read(|reader| self.clusters.get(cluster_id).see_new_unit(unit_kind, reader))
                        .map(Some)
                }

                // a unit we see has been updated.
                0x1D => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated");
                    packet.read(|reader| self.clusters.get(cluster_id).see_update_unit(reader))
                }

                // a once known unit vanished.
                0x1E => {
                    let cluster_id = ClusterId(packet.header().id0());
                    debug_assert!(self.clusters.has(cluster_id), "{cluster_id:?} is not populated");
                    packet.read(|reader| self.clusters.get(cluster_id).see_unit_no_more(reader.read_string()))
                }

                // tick completed
                0x20 => {
                    self.login_completed.store(true);
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

    fn update(&self, mut packet: Packet) {
        self.id.store(GalaxyId(packet.header().param()));
        packet.read(|reader| self.config.store(Arc::new(GalaxyConfig::from(reader))));
    }

    /// Waits until the login proceedure has been completed for  this [`Galaxy`].
    pub async fn wait_login_completed(&self) -> Result<(), GameError> {
        while !self.login_completed.load() {
            self.wait_next_turn().await?;
        }
        Ok(())
    }

    /// Waits for the next [`FlattiverseEvent::TickCompleted`] for this [`Galaxy`].
    pub async fn wait_next_turn(&self) -> Result<(), GameError> {
        loop {
            if let FlattiverseEvent::TickCompleted = self.next_event().await? {
                break;
            }
        }
        Ok(())
    }

    /// Sets the given values for this [`Galaxy`].
    /// See also [`ConnectionHandle::configure_galaxy`].
    #[inline]
    pub async fn configure(&self, config: &GalaxyConfig) -> Result<(), GameError> {
        self.connection
            .configure_galaxy(self.id.load(), config)
            .await
    }

    /// Creates a [`Cluster`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_cluster`].
    #[inline]
    pub async fn create_cluster(&self, config: &ClusterConfig) -> Result<Arc<Cluster>, GameError> {
        let cluster_id = self.connection.create_cluster(config).await?;
        Ok(self.clusters.get(cluster_id))
    }

    /// Creates a [`Team`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_team`].
    #[inline]
    pub async fn create_team(&self, config: &TeamConfig) -> Result<Arc<Team>, GameError> {
        let team_id = self.connection.create_team(config).await?;
        Ok(self.teams.get(team_id))
    }

    /// Creates a [`ShipDesign`] within this [`Galaxy`].
    /// See also [`ConnectionHandle::create_ship_design`].
    #[inline]
    pub async fn create_ship_design(
        &self,
        config: &ShipDesignConfig,
    ) -> Result<Arc<ShipDesign>, GameError> {
        let ship_design_id = self.connection.create_ship_design(config).await?;
        Ok(self.ship_designs.get(ship_design_id))
    }

    /// Registers a new ship with the given name and [`crate::hierarchy::ShipDesign`]. The name must
    /// obey naming conventions and the chosen design must have set `free_spawn`. All
    /// [`crate::hierarchy::ShipDesign`]s which don't have `free_spawn` set (=`false`) must be built
    /// in game and can't be just registered.
    /// See also [`ConnectionHandle::register_ship`].
    #[inline]
    pub async fn register_ship(
        &self,
        name: impl AsRef<str>,
        design: ShipDesignId,
    ) -> Result<Arc<Controllable>, GameError> {
        let controllable_id = self.connection.register_ship(name, design).await?;
        Ok(self.controllables.get(controllable_id))
    }

    /// Sends a chat message with a maximum of 512 characters to all players in this [`Galaxy`].
    #[inline]
    pub async fn chat(&mut self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.connection.chat_galaxy(message).await
    }

    #[inline]
    pub fn config(&self) -> impl Deref<Target = Arc<GalaxyConfig>> {
        self.config.load()
    }

    #[inline]
    pub fn get_cluster(&self, id: ClusterId) -> Arc<Cluster> {
        self.clusters.get(id)
    }

    #[inline]
    pub fn clusters(&self) -> &UniversalArcHolder<ClusterId, Cluster> {
        &self.clusters
    }

    #[inline]
    pub fn get_ship_design(&self, id: ShipDesignId) -> Arc<ShipDesign> {
        self.ship_designs.get(id)
    }

    #[inline]
    pub fn ship_designs(&self) -> &UniversalArcHolder<ShipDesignId, ShipDesign> {
        &self.ship_designs
    }

    #[inline]
    pub fn get_team(&self, id: TeamId) -> Arc<Team> {
        self.teams.get(id)
    }

    #[inline]
    pub fn teams(&self) -> &UniversalArcHolder<TeamId, Team> {
        &self.teams
    }

    #[inline]
    pub fn get_controllable(&self, id: ControllableId) -> Arc<Controllable> {
        self.controllables.get(id)
    }

    #[inline]
    pub fn controllables(&self) -> &UniversalArcHolder<ControllableId, Controllable> {
        &self.controllables
    }

    #[inline]
    pub fn get_player(&self, id: PlayerId) -> Arc<Player> {
        self.players.get(id)
    }

    #[inline]
    pub fn players(&self) -> &UniversalArcHolder<PlayerId, Player> {
        &self.players
    }

    #[inline]
    pub fn connection(&self) -> &ConnectionHandle {
        &self.connection
    }

    #[inline]
    pub fn connected(&self) -> bool {
        !self.events.is_closed()
    }
}
