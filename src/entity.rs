use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use byteorder::ReadBytesExt;

use num_traits::FromPrimitive;

use crate::io::BinaryReader;
use crate::packet::Packet;

#[derive(Debug)]
pub struct Universe {
    id: u16,
    name: String,
    description: String,
    difficulty: Difficulty,
    mode: Mode,
    owner_id: u32,
    max_players: u16,
    max_ships_per_player: u8,
    max_ships_per_team: u16,
    status: Status,
    default_privileges: Privileges,
    avatar: Vec<u8>,
}

pub(crate) mod command_id {
    /// Issued, whenever a universe definition has been created, updated or when a
    /// universe has been deleted.
    ///
    /// data: nothing for a deleted universe, universe-data for a updated or new universe
    pub(crate) const S2C_UNIVERSE_META_INFO_UPDATED: u8 = 0x10;
}

impl TryFrom<&Packet> for Universe {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        if packet.command != command_id::S2C_UNIVERSE_META_INFO_UPDATED {
            panic!("Invalid command")
        }

        let reader = &mut &packet.payload.as_ref().unwrap()[..] as &mut dyn BinaryReader;

        Ok(Universe {
            id: packet.base_address,
            name: reader.read_string()?,
            description: reader.read_string()?,
            difficulty: Difficulty::from_u8(reader.read_u8()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            mode: Mode::from_u8(reader.read_u8()?)
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
        })
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Insane = 3,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Mode {
    Mission = 0,
    ShootTheFlag = 1,
    Domination = 2,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Status {
    Online = 0,
    Offline = 1,
    Maintenance = 2,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Privileges {
    Join = 1,
    ManageUnits = 2,
    ManageRegions = 4,
    ManageMaps = 8,
    ManageUniverse = 16,
}
