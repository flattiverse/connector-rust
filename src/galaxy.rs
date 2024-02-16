use crate::cluster::Cluster;
use crate::error::{GameError, GameErrorKind};
use crate::events::FlattiverseEvent;
use crate::game_type::GameType;
use crate::network::{ConnectError, ConnectionEvent, ConnectionHandle, Packet};
use crate::player::Player;
use crate::team::Team;
use crate::unit::Ship;
use crate::PlayerKind;
use async_channel::Receiver;
use nohash_hasher::BuildNoHashHasher;
use num_enum::FromPrimitive;
use std::collections::HashMap;
use std::io::Write;

pub struct Galaxy {
    id: i32,
    name: String,
    description: String,
    game_type: GameType,
    max_players: i32,

    max_platforms_universe: i32,
    max_probes_universe: i32,
    max_drones_universe: i32,
    max_ships_universe: i32,
    max_bases_universe: i32,

    max_platforms_team: i32,
    max_probes_team: i32,
    max_drones_team: i32,
    max_ships_team: i32,
    max_bases_team: i32,

    max_platforms_player: i32,
    max_probes_player: i32,
    max_drones_player: i32,
    max_ships_player: i32,
    max_bases_player: i32,

    clusters: Vec<Option<Cluster>>,
    ships: Vec<Option<Ship>>,
    teams: Vec<Option<Team>>,
    players: HashMap<u8, Player, BuildNoHashHasher<u8>>,

    //
    connection: ConnectionHandle,
    receiver: Receiver<ConnectionEvent>,
}

impl Galaxy {
    pub async fn join(uri: &str, auth: &str, team: u8) -> Result<Self, GameError> {
        eprintln!("lol");
        std::io::stderr().flush().unwrap();
        let connection = crate::network::connect(uri, auth, team)
            .await
            .map_err(|e| match e {
                ConnectError::GameError(error) => error,
                e => GameError::from(GameErrorKind::GenericException)
                    .with_info(format!("Failed to connect due to local issues: {e}")),
            })?;
        let (handle, receiver) = connection.spawn();
        eprintln!("lol");
        std::io::stderr().flush().unwrap();

        Ok(Self {
            connection: handle,
            receiver,

            id: 0,
            name: String::default(),
            description: String::default(),
            game_type: GameType::Mission,
            max_players: 0,

            max_platforms_universe: 0,
            max_probes_universe: 0,
            max_drones_universe: 0,
            max_ships_universe: 0,
            max_bases_universe: 0,

            max_platforms_team: 0,
            max_probes_team: 0,
            max_drones_team: 0,
            max_ships_team: 0,
            max_bases_team: 0,

            max_platforms_player: 0,
            max_probes_player: 0,
            max_drones_player: 0,
            max_ships_player: 0,
            max_bases_player: 0,

            clusters: Self::filled_vec::<_, 256>(),
            ships: Self::filled_vec::<_, 256>(),
            teams: Self::filled_vec::<_, 256>(),

            players: HashMap::default(),
        })
    }

    fn filled_vec<T, const SIZE: usize>() -> Vec<Option<T>> {
        (0..SIZE).into_iter().map(|_| None).collect()
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
                    self.id = packet.header().param().into();
                    packet.read(|reader| {
                        self.name = reader.read_string();
                        self.description = reader.read_string();
                        self.game_type = GameType::from_primitive(reader.read_byte());
                        self.max_players = reader.read_int32();
                        self.max_platforms_universe = reader.read_int32();
                        self.max_probes_universe = reader.read_int32();
                        self.max_drones_universe = reader.read_int32();
                        self.max_ships_universe = reader.read_int32();
                        self.max_bases_universe = reader.read_int32();
                        self.max_platforms_team = reader.read_int32();
                        self.max_probes_team = reader.read_int32();
                        self.max_drones_team = reader.read_int32();
                        self.max_ships_team = reader.read_int32();
                        self.max_bases_team = reader.read_int32();
                        self.max_platforms_player = reader.read_int32();
                        self.max_probes_player = reader.read_int32();
                        self.max_drones_player = reader.read_int32();
                        self.max_ships_player = reader.read_int32();
                        self.max_bases_player = reader.read_int32();
                    });
                    Ok(Some(FlattiverseEvent::GalaxyUpdated(self.id)))
                }
                // cluster info
                0x11 => {
                    let cluster_id = packet.header().param0();
                    let cluster = packet.read(|reader| Cluster::new(cluster_id, self.id, reader));
                    {
                        let cluster_id = usize::from(cluster_id);
                        self.clusters[cluster_id] = Some(cluster);
                    }
                    Ok(Some(FlattiverseEvent::ClusterUpdated {
                        galaxy: self.id,
                        cluster: cluster_id,
                    }))
                }

                // team info
                0x12 => {
                    let team_id = packet.header().param0();
                    let team = packet.read(|reader| Team::new(team_id, reader));
                    {
                        let team_id = usize::from(team_id);
                        self.teams[team_id] = Some(team);
                    }
                    Ok(Some(FlattiverseEvent::TeamUpdated {
                        galaxy: self.id,
                        team: team_id,
                    }))
                }

                // ship info
                0x13 => {
                    let ship_id = packet.header().param0();
                    let ship = packet.read(|reader| Ship::new(ship_id, self.id, reader));
                    {
                        let ship_id = usize::from(ship_id);
                        self.ships[ship_id] = Some(ship);
                    }
                    Ok(Some(FlattiverseEvent::ShipUpdated {
                        galaxy: self.id,
                        ship: ship_id,
                    }))
                }

                // upgrade info
                0x14 => {
                    let upgrade_id = packet.header().param0();
                    let ship_id = packet.header().param1();
                    if let Some(ship) = &mut self.ships[usize::from(ship_id)] {
                        packet.read(|reader| ship.read_upgrade(upgrade_id, reader));
                        Ok(Some(FlattiverseEvent::UpgradeUpdated {
                            galaxy: self.id,
                            ship: ship_id,
                            upgrade: upgrade_id,
                        }))
                    } else {
                        Err(
                            GameError::from(GameErrorKind::Unspecified(0)).with_info(format!(
                                "Tried to update Upgrade of ship={ship_id} that does not exist"
                            )),
                        )
                    }
                }

                // new player joined info
                0x15 => {
                    let player_id = packet.header().player();
                    let team_id = packet.header().param1();
                    let player_kind = PlayerKind::from_primitive(packet.header().param0());
                    let player =
                        packet.read(|reader| Player::new(player_id, player_kind, team_id, reader));
                    self.players.insert(player_id, player);
                    Ok(Some(FlattiverseEvent::PlayerUpdated {
                        galaxy: self.id,
                        player: player_id,
                    }))
                }

                cmd => Err(GameError::from(GameErrorKind::Unspecified(0))
                    .with_info(format!("Unexpected command={cmd} for Galaxy={}", self.id))),
            }
        }
    }

    #[inline]
    pub fn receiver_queue_len(&self) -> usize {
        self.receiver.len()
    }
}
