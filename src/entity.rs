use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use byteorder::ReadBytesExt;

use num_traits::FromPrimitive;

use crate::io::BinaryReader;
use crate::packet::Packet;
use crate::players::Team;

const DEFAULT_TEAMS: usize = 16;

#[derive(Debug)]
pub struct Universe {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) difficulty: Difficulty,
    pub(crate) mode: UniverseMode,
    pub(crate) owner_id: u32,
    pub(crate) max_players: u16,
    pub(crate) max_ships_per_player: u8,
    pub(crate) max_ships_per_team: u16,
    pub(crate) status: Status,
    pub(crate) default_privileges: Privileges,
    pub(crate) avatar: Vec<u8>,
    pub(crate) teams: Vec<Option<Team>>,
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
}

pub(crate) mod command_id {
    /// Issued after the login has completed. This marks also that the client
    /// has received all necessary information about `Universe`s and thereof.
    ///
    /// data: none
    pub(crate) const S2C_LOGIN_COMPLETED: u8 = 0x0F;

    /// Issued whenever a universe definition has been created, updated or when a
    /// universe has been deleted.
    ///
    /// data: nothing for a deleted universe, universe-data for an updated or new universe
    pub(crate) const S2C_UNIVERSE_META_INFO_UPDATED: u8 = 0x10;

    /// Issued whenever a team definition has been created, updated or when a team has
    /// been deleted.
    ///
    /// data: nothing for a deleted team, team-data for an updated or newly created team
    pub(crate) const S2C_UNIVERSE_TEAM_META_INFO_UPDATE: u8 = 0x11;
}

impl TryFrom<&Packet> for Universe {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Universe {
            id: packet.base_address,
            name: reader.read_string()?,
            description: reader.read_string()?,
            difficulty: Difficulty::from_u8(reader.read_u8()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            mode: UniverseMode::from_u8(reader.read_u8()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            owner_id: reader.read_u32()?,
            max_players: reader.read_u16()?,
            max_ships_per_player: reader.read_u8()?,
            max_ships_per_team: reader.read_u16()?,
            status: Status::from_u8(reader.read_u8()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            default_privileges: Privileges::from_u8(reader.read_u8()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            avatar: Vec::default(),
            teams: vec_of_none!(DEFAULT_TEAMS),
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
    Join = 1,
    ManageUnits = 2,
    ManageRegions = 4,
    ManageMaps = 8,
    ManageUniverse = 16,
}
