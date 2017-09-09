
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::sync::Mutex;

use Task;
use Team;
use Error;
use Player;
use TimeSpan;
use GameType;
use Universe;
use Connector;
use ConnectorArc;
use IndexList;
use Difficulty;
use Tournament;
use UniversalHolder;
use UniverseGroupFlowControl;
use PerformanceRequirement;

use net::Packet;
use net::BinaryReader;
use net::BinaryWriter;
use net::is_set_u8;

pub struct UniverseGroup {
    id:         u16,
    name:       String,
    game_type:  Option<GameType>,
    difficulty: Difficulty,

    performance_requirement:    PerformanceRequirement,

    max_tick_time:  TimeSpan,
    avg_tick_time:  TimeSpan,

    password_required:      bool,
    achievement_required:   bool,
    maximum_ship_level:      u8,
    maximum_players:        u16,

    max_platforms_per_player:   u8,
    max_probes_per_player:      u8,
    max_drones_per_player:      u8,
    max_ships_per_player:       u8,
    max_bases_per_player:       u8,
    max_platforms_per_team:     u16,
    max_probes_per_team:        u16,
    max_drones_per_team:        u16,
    max_ships_per_team:         u16,
    max_bases_per_team:         u16,

    kill_lonesome_units:        bool,
    description:                String,

    connector:  Weak<Connector>,

    universes:  RwLock<UniversalHolder<Universe>>,
    teams:      RwLock<UniversalHolder<Team>>,
    players:    RwLock<UniversalHolder<Player>>,

    tournament: Option<Arc<Mutex<Tournament>>>
}

impl UniverseGroup {

    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet) -> Result<UniverseGroup, Error> {
        let reader = &mut packet.read() as &mut BinaryReader;

        let name = reader.read_string()?;
        let game_type   = GameType::from_id(reader.read_byte()?);
        let difficulty = Difficulty::from_id(reader.read_byte()?)?;

        let performance_requirement = PerformanceRequirement::from_id(reader.read_byte()?)?;

        let max_tick_time = TimeSpan::new(reader.read_i64()?);
        let avg_tick_time = TimeSpan::new(reader.read_i64()?);

        let header = reader.read_byte()?;

        let password_required      = is_set_u8(header, 0x01);
        let achievement_required   = is_set_u8(header, 0x02);

        let maximum_ship_level       = reader.read_unsigned_byte()?;
        let maximum_players          = reader.read_u16()?;

        let max_platforms_per_player = reader.read_byte()?;
        let max_probes_per_player    = reader.read_byte()?;
        let max_drones_per_player    = reader.read_byte()?;
        let max_ships_per_player     = reader.read_byte()?;
        let max_bases_per_player     = reader.read_byte()?;

        let max_platforms_per_team   = reader.read_u16()?;
        let max_probes_per_team      = reader.read_u16()?;
        let max_drones_per_team      = reader.read_u16()?;
        let max_ships_per_team       = reader.read_u16()?;
        let max_bases_per_team       = reader.read_u16()?;

        let kill_lonesome_units      = reader.read_bool()?;
        let description              = reader.read_string()?;

        let universes = RwLock::new(UniversalHolder::new(IndexList::new(false, 16)));
        let teams     = RwLock::new(UniversalHolder::new(IndexList::new(false, 16)));
        let players   = RwLock::new(UniversalHolder::new(IndexList::new(false, maximum_players as usize)));

        Ok(UniverseGroup {
            connector:  Arc::downgrade(connector),
            id:         packet.path_universe_group(),

            name,
            game_type,
            difficulty,
            performance_requirement,
            max_tick_time,
            avg_tick_time,

            password_required,
            achievement_required,

            maximum_ship_level,
            maximum_players,
            max_platforms_per_player,
            max_probes_per_player,
            max_drones_per_player,
            max_ships_per_player,
            max_bases_per_player,

            max_platforms_per_team,
            max_probes_per_team,
            max_drones_per_team,
            max_ships_per_team,
            max_bases_per_team,

            kill_lonesome_units,
            description,

            universes,
            teams,
            players,
            tournament: None,
        })
    }

    pub fn tournament(&self) -> &Option<Arc<Mutex<Tournament>>> {
        if self.tournament.is_some() {
            match self.connector.upgrade() {
                None => {},
                Some(connector) => {
                    connector.register_task_quitely_if_unknown(Task::UsedTournament);
                }
            }
        };
        &self.tournament
    }

    pub fn set_tournament(&mut self, tournament: Option<Arc<Mutex<Tournament>>>) {
        self.tournament = tournament;
    }

    /// The avatar of this [UniverseGroup] as jpeg/raw (bitmap)
    pub fn avatar_raw(&self) -> Result<Vec<u8>, Error> {
        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let block = connector.block_manager().block()?;
                let mut packet = Packet::new();

                {
                    let block = block.lock()?;
                    packet.set_command(0x20);
                    packet.set_session(block.id());
                    packet.set_path_universe_group(self.id);
                }

                connector.send(&packet)?;
                let response = block.lock()?.wait()?;

                Ok(Vec::from(response.read()))
            }
        }
    }

    pub fn new_flow_control(&self) -> Result<Arc<UniverseGroupFlowControl>, Error> {
        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let flow = connector.register_flow_control()?;
                flow.setup()?;
                Ok(flow)
            }
        }
    }


    pub fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    pub fn game_type(&self) -> Option<GameType> {
        None
    }

    pub fn team(&self, index: u8) -> &Option<Arc<RwLock<Team>>> {
        &None
    }
}

impl PartialEq for UniverseGroup {
    fn eq(&self, other: &UniverseGroup) -> bool {
        self.id == other.id
    }
}