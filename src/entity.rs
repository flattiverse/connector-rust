use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::ops::RangeInclusive;

use num_traits::FromPrimitive;

use crate::command;
use crate::io::BinaryReader;
use crate::packet::Packet;
use crate::players::Team;

const DEFAULT_TEAMS: usize = 16;
const DEFAULT_GALAXIES: usize = 32;

#[derive(Debug, Clone)]
pub struct Universe {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) difficulty: Difficulty,
    pub(crate) mode: UniverseMode,
    pub(crate) owner_id: u32,
    pub(crate) max_players: u16,
    pub(crate) max_players_per_team: u16,
    pub(crate) max_ships_per_player: u8,
    pub(crate) max_ships_per_team: u16,
    pub(crate) status: Status,
    pub(crate) default_privileges: Privileges,
    pub(crate) avatar: Vec<u8>,
    pub(crate) teams: Vec<Option<Team>>,
    pub(crate) galaxies: Vec<Option<Galaxy>>,
    pub(crate) systems: Vec<System>,
}

impl Universe {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn mode(&self) -> UniverseMode {
        self.mode
    }

    pub fn owner_iod(&self) -> u32 {
        self.owner_id
    }

    pub fn max_players(&self) -> u16 {
        self.max_players
    }

    pub fn max_ships_per_player(&self) -> u8 {
        self.max_ships_per_player
    }

    pub fn max_ships_per_team(&self) -> u16 {
        self.max_ships_per_team
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn default_privileges(&self) -> Privileges {
        self.default_privileges
    }

    pub fn avatar(&self) -> &[u8] {
        &self.avatar
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter().filter_map(Option::as_ref)
    }

    pub fn team(&self, id: u8) -> Option<&Team> {
        self.teams.get(usize::from(id)).and_then(Option::as_ref)
    }

    pub fn galaxies(&self) -> impl Iterator<Item = &Galaxy> {
        self.galaxies.iter().filter_map(Option::as_ref)
    }

    pub fn galaxy(&self, id: u8) -> Option<&Galaxy> {
        self.galaxies.get(usize::from(id)).and_then(Option::as_ref)
    }

    pub fn systems(&self) -> impl Iterator<Item = &System> {
        self.systems.iter()
    }

    #[must_use]
    pub fn join(&self) -> Packet {
        debug!(
            "Issuing join request for universe[{}] '{}' and auto-select team",
            self.id, self.name,
        );
        let mut packet = Packet::default();
        packet.command = command::id::C2S_UNIVERSE_JOIN;
        packet.sub_address = 0x00; // auto selection
        packet
    }

    #[must_use]
    pub fn join_with_team(&self, team_id: u8) -> Packet {
        debug!(
            "Issuing join request for universe[{}] '{}' on team[{}] '{}'",
            self.id,
            self.name,
            team_id,
            self.teams
                .get(usize::from(team_id))
                .and_then(Option::<Team>::as_ref)
                .map(Team::name)
                .unwrap_or("")
        );
        let mut packet = Packet::default();
        packet.command = command::id::C2S_UNIVERSE_JOIN;
        packet.base_address = self.id;
        packet.sub_address = team_id;
        packet
    }

    #[must_use]
    pub fn part(&self) -> Packet {
        debug!(
            "Issuing part request for universe[{}] '{}'",
            self.id, self.name
        );
        let mut packet = Packet::default();
        packet.command = command::id::C2S_UNIVERSE_PART;
        packet.base_address = self.id;
        packet
    }
}

impl TryFrom<&Packet> for Universe {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Universe {
            id: packet.base_address,
            name: reader.read_string()?,
            description: reader.read_string()?,
            difficulty: Difficulty::from_u8(reader.read_byte()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            mode: UniverseMode::from_u8(reader.read_byte()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            owner_id: reader.read_u32()?,
            max_players: reader.read_uint16()?,
            max_players_per_team: reader.read_uint16()?,
            max_ships_per_player: reader.read_byte()?,
            max_ships_per_team: reader.read_uint16()?,
            status: Status::from_u8(reader.read_byte()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            default_privileges: Privileges::from(reader.read_byte()?),
            avatar: Vec::default(),
            teams: vec_of_none!(DEFAULT_TEAMS),
            galaxies: vec_of_none!(DEFAULT_GALAXIES),
            systems: Vec::default(),
        })
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Insane = 3,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum UniverseMode {
    Mission = 0,
    ShootTheFlag = 1,
    Domination = 2,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum Status {
    Online = 0,
    Offline = 1,
    Maintenance = 2,
}

#[derive(Debug, Copy, Clone)]
pub struct Privileges(u8);

impl From<u8> for Privileges {
    fn from(value: u8) -> Self {
        Privileges(value)
    }
}

impl Privileges {
    pub const fn is_nothing(self) -> bool {
        self.0 == 0
    }

    pub const fn allowed_to_join(self) -> bool {
        self.0 & 1 != 0
    }

    pub const fn allowed_to_manage_units(self) -> bool {
        self.0 & 2 != 0
    }

    pub const fn allowed_to_manage_regions(self) -> bool {
        self.0 & 4 != 0
    }

    pub const fn allowed_to_manage_systems(self) -> bool {
        self.0 & 8 != 0
    }

    pub const fn allowed_to_manage_universes(self) -> bool {
        self.0 & 16 != 0
    }
}

#[derive(Debug, Clone)]
pub struct Galaxy {
    id: u8,
    name: String,
    spawn: bool,
}

impl Galaxy {
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether you can spawn into this galaxy
    pub fn spawn(&self) -> bool {
        self.spawn
    }
}

impl TryFrom<&Packet> for Galaxy {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        Ok(Galaxy {
            id: packet.sub_address,
            name: reader.read_string()?,
            spawn: reader.read_bool()?,
        })
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum SystemKind {
    /// The ships hull, a higher level indicates more hull points TODO ...
    Hull = 0x00,

    /// The ships armor, a higher level indicates more effective hit-points for TODO ...
    Armor = 0x01,

    /// The ships primary and short range scanner
    Scanner0 = 0x08,

    /// The ships secondary and long range scanner
    Scanner1 = 0x09,

    /// A higher leveled engine allows greater acceleration
    Engine = 0x10,

    /// A higher leveled thruster allows a greater turn-rate acceleration
    Thruster = 0x11,

    /// Meaning things like a Solar**Cell**
    /// A Higher level (Solar)Cell allows faster harvesting (of energy from suns)
    Cell = 0x18,

    /// The storage capacity of your ship
    Battery = 0xC0,
}

#[derive(Debug, Clone)]
pub struct System {
    kind: SystemKind,
    levels: RangeInclusive<u8>,
}

impl System {
    pub fn kind(&self) -> SystemKind {
        self.kind
    }

    pub fn levels(&self) -> RangeInclusive<u8> {
        self.levels.clone()
    }

    // The inclusive start level of this system
    pub fn level_start(&self) -> u8 {
        *self.levels.start()
    }

    /// The inclusive end level for this system
    pub fn level_end(&self) -> u8 {
        *self.levels.end()
    }

    pub(crate) fn vec_from(packet: &Packet) -> Result<Vec<System>, IoError> {
        let count = packet.payload().len() / 3; // TODO
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(System {
                kind: SystemKind::from_u8(reader.read_byte()?)
                    .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
                levels: RangeInclusive::new(reader.read_byte()?, reader.read_byte()?),
            });
        }
        Ok(vec)
    }
}
