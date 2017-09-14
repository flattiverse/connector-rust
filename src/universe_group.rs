
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

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
use ManagedArray;
use UniversalHolder;
use UniversalEnumerable;
use UniverseGroupFlowControl;
use PerformanceRequirement;

use controllable::Controllable;

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
    players:    RwLock<ManagedArray<Arc<Player>>>,

    tournament: RwLock<Option<Arc<Tournament>>>
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
        let players   = RwLock::new(ManagedArray::with_capacity(maximum_players as usize));

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
            tournament: RwLock::new(None),
        })
    }

    pub fn tournament(&self) -> Option<Arc<Tournament>> {
        let tournament = self.tournament.read().unwrap();
        if tournament.is_some() {
            match self.connector.upgrade() {
                None => {},
                Some(connector) => {
                    connector.register_task_quitely_if_unknown(Task::UsedTournament);
                }
            }
        };
        tournament.clone()
    }

    pub(crate) fn set_tournament(&self, tournament: Option<Arc<Tournament>>) -> Result<(), Error> {
        *self.tournament.write()? = tournament;
        Ok(())
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

    // TODO missing parameter 'CrystalCargoItem...crystals'
    /// The returned value is supposed to be a Ship
    pub fn register_ship(&self, class: &str, name: &str) -> Result<Arc<Controllable>, Error> {
        let connector = self.connector.upgrade().ok_or(Error::ConnectorNotAvailable)?;

        let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
        match player.universe_group().upgrade() {
            None => return Err(Error::PlayerNotInUniverseGroup),
            Some(group) => {
                let id_other = group.id();
                if id_other != self.id {
                    return Err(Error::PlayerAlreadyInAnotherUniverseGroup(id_other));
                }
            }
        };

        if !"@Ship".eq(class) && !Connector::check_name(class) {
            return Err(Error::InvalidClass);
        }

        if !Connector::check_name(name) {
            return Err(Error::InvalidName);
        }

        // TODO missing crystals check

        let block = connector.block_manager().block()?;
        let mut packet = Packet::new();
        let mut block  = block.lock()?;

        packet.set_command(0x80);
        packet.set_session(block.id());

        {
            let writer = packet.write() as &mut BinaryWriter;
            writer.write_string(class)?;
            writer.write_string(name)?;
            writer.write_u8(0)?; // TODO crystal count + crystals
        }

        connector.send(&packet)?;
        let response = block.wait()?;
        connector.controllable(response.path_ship())
    }

    pub fn part(&self) -> Result<(), Error> {
        let connector = self.connector.upgrade().ok_or(Error::ConnectorNotAvailable)?;

        let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
        match player.universe_group().upgrade() {
            None => return Err(Error::PlayerNotInUniverseGroup),
            Some(group) => {
                let id_other = group.id();
                if id_other != self.id {
                    return Err(Error::PlayerAlreadyInAnotherUniverseGroup(id_other));
                }
            }
        };

        if connector.has_flows()? {
            return Err(Error::StillOpenFlowControlsInUniverseGroup(self.id));
        }

        let block = connector.block_manager().block()?;
        let mut packet = Packet::new();
        let mut block  = block.lock()?;

        packet.set_command(0x06);
        packet.set_session(block.id());

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }

    // TODO missing #reset()

    pub fn chat(&self, message: &str) -> Result<(), Error> {
        if message.is_empty() || message.len() > 140 {
            return Err(Error::InvalidMessage);
        }

        let connector = self.connector.upgrade().ok_or(Error::ConnectorNotAvailable)?;

        let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
        match player.universe_group().upgrade() {
            None => return Err(Error::PlayerNotInUniverseGroup),
            Some(group) => {
                let id_other = group.id();
                if id_other != self.id {
                    return Err(Error::PlayerAlreadyInAnotherUniverseGroup(id_other));
                }
            }
        };

        let block = connector.block_manager().block()?;
        let mut packet = Packet::new();
        let mut block  = block.lock()?;

        packet.set_command(0x32);
        packet.set_session(block.id());

        {
            let writer = packet.write() as &mut BinaryWriter;
            writer.write_string(message)?;
        }

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }

    pub fn join(&self, name: &str, team: u8, clan: Option<&str>, password: Option<&str>) -> Result<(), Error> {
        let _ = self.team(team)?;

        if name.is_empty() || name.len() > 140 {
            return Err(Error::InvalidName);
        }

        let connector  = self.connector.upgrade().ok_or(Error::ConnectorNotAvailable)?;
        let block      = connector.block_manager().block()?;
        let mut packet = Packet::new();
        let mut block  = block.lock()?;

        packet.set_command(0x04);
        packet.set_session(block.id());
        packet.set_path_universe_group(self.id);
        packet.set_path_sub(team);

        {
            let writer = packet.write() as &mut BinaryWriter;
            writer.write_string(name)?;
            let mut header = 0x00;

            if clan.is_some() {
                header |= 0x01;
            }

            if password.is_some() {
                header |= 0x02;
            }

            writer.write_byte(header)?;

            if let Some(c) = clan {
                writer.write_string(c)?;
            }

            if let Some(p) = password {
                writer.write_string(p)?;
            }
        }

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn universe(&self, index: u8) -> Weak<Universe> {
        self.universes.read().unwrap().get_for_index_weak(index as usize)
    }

    pub(crate) fn set_universe(&self, index: u8, universe: Option<Arc<Universe>>) {
        self.universes.write().unwrap().set(index as usize, universe);
    }

    pub fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    pub fn game_type(&self) -> Option<GameType> {
        self.game_type
    }

    pub fn teams(&self) -> &RwLock<UniversalHolder<Team>> {
        &self.teams
    }

    pub fn team(&self, index: u8) -> Result<Arc<Team>, Error> {
        self.teams.read()?.get_for_index(index as usize).ok_or(Error::InvalidTeam(index))
    }

    pub fn team_weak(&self, index: u8) -> Weak<Team> {
        self.teams.read().unwrap().get_for_index_weak(index as usize)
    }

    pub(crate) fn set_team(&self, index: u8, team: Option<Arc<Team>>) {
        self.teams.write().unwrap().set(index as usize, team);
    }

    pub fn avg_tick_time(&self) -> &TimeSpan {
        &self.avg_tick_time
    }

    // TODO change public method to RwLockReadGuard
    pub fn players(&self) -> &RwLock<ManagedArray<Arc<Player>>> {
        &self.players
    }

    pub fn universes(&self) -> RwLockReadGuard<UniversalHolder<Universe>> {
        self.universes.read().unwrap()
    }

    pub fn maximum_ship_level(&self) -> u8 {
        self.maximum_ship_level
    }

    pub fn max_players(&self) -> u16 {
        self.maximum_players
    }
}

impl UniversalEnumerable for UniverseGroup {
    fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for UniverseGroup {
    fn eq(&self, other: &UniverseGroup) -> bool {
        self.id == other.id
    }
}