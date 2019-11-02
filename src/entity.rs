use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

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

    pub fn galaxies(&self) -> impl Iterator<Item = &Galaxy> {
        self.galaxies.iter().filter_map(Option::as_ref)
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
            default_privileges: Privileges::from_u8(reader.read_byte()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            avatar: Vec::default(),
            teams: vec_of_none!(DEFAULT_TEAMS),
            galaxies: vec_of_none!(DEFAULT_GALAXIES),
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

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum Privileges {
    Nothing = 0,
    Join = 1,
    ManageUnits = 2,
    ManageRegions = 4,
    ManageMaps = 8,
    ManageUniverse = 16,
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
