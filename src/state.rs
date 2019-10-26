use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::io::Error as IoError;

use backtrace::Backtrace;

use crate::entity::command_id;
use crate::entity::Universe;
use crate::io::BinaryReader;
#[macro_use]
use crate::macros::*;
use crate::num_traits::FromPrimitive;
use crate::packet::Packet;
use crate::players::Team;
use std::mem::replace;

const DEFAULT_PLAYERS: usize = 16;
const DEFAULT_UNIVERSES: usize = 16;

pub struct State {
    players: Vec<Option<Player>>,
    universes: Vec<Option<Universe>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            players: vec_of_none!(DEFAULT_PLAYERS),
            universes: vec_of_none!(DEFAULT_UNIVERSES),
        }
    }

    pub(crate) fn update(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!("Updating state with command 0x{:02x}", packet.command);
        match packet.command {
            command_id::S2C_PLAYER_REMOVED => self.remove_player(packet)?,
            command_id::S2C_NEW_PLAYER => self.new_player(packet)?,
            command_id::S2C_PLAYER_DEFRAGMENTED => self.player_defragmented(packet)?,
            command_id::S2C_PLAYER_PING_UPDATE => self.player_ping_update(packet)?,
            command_id::S2C_LOGIN_RESPONSE => {
                if packet.helper != 0u8 {
                    return Err(UpdateError::LoginRefused(
                        RefuseReason::from_u8(packet.helper).expect("Failed to parse RefuseReason"),
                    ));
                }
            }
            command_id::S2C_UNIVERSE_META_INFO_UPDATED => self.update_universe(packet)?,
            command_id::S2C_UNIVERSE_TEAM_META_INFO_UPDATE => self.update_universe_team(packet)?,
            command => warn!("Unknown command: {}", command),
        }
        Ok(())
    }

    fn remove_player(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        let index = usize::from(packet.base_address);
        debug!("Going to forget player at index {}", index);
        self.players[index] = None;
        Ok(())
    }

    fn new_player(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!("Going to add a new player at index {}", packet.base_address);
        expand_vec_of_none_if_necessary!(self.players, usize::from(packet.base_address));
        let player = map_payload_with_try_from!(packet, Player);
        debug!("Received player: {:#?}", player);
        self.players[usize::from(packet.base_address)] = player;
        Ok((()))
    }

    fn player_defragmented(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        let index_new = usize::from(packet.base_address);
        debug!("Going to move player to new index {}", index_new);
        let index_old = usize::from((&mut packet.payload() as &mut dyn BinaryReader).read_u16()?);
        let player = replace(&mut self.players[index_old], None);
        debug!(
            "Player is moved from old index {}: {:#?}",
            index_old, player
        );
        self.players[index_new] = player;
        Ok(())
    }

    fn player_ping_update(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!(
            "Going to update ping for player at index {}",
            packet.base_address
        );
        (&mut self.players)[usize::from(packet.base_address)]
            .as_mut()
            .expect("Invalid player index for ping update")
            .update_ping(packet)?;
        Ok(())
    }

    fn update_universe(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!(
            "Going to update universe for {}, delete={}",
            packet.base_address,
            packet.payload.is_none()
        );
        let universe = map_payload_with_try_from!(packet, Universe);
        debug!("Received universe {:#?}", universe);
        expand_vec_of_none_if_necessary!(self.universes, usize::from(packet.base_address));
        self.universes[usize::from(packet.base_address)] = universe;
        Ok(())
    }

    fn update_universe_team(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!(
            "Going to update team {} for universe {}, delete={}",
            packet.sub_address,
            packet.base_address,
            packet.payload.is_some()
        );
        let universe = self.universes[usize::from(packet.base_address)]
            .as_mut()
            .expect("Failed to update universe because unknown for given base_address");
        let team = map_payload_with_try_from!(packet, Team);
        expand_vec_of_none_if_necessary!(universe.teams, usize::from(packet.sub_address));
        universe.teams[usize::from(packet.sub_address)] = team;
        Ok(())
    }
}

#[derive(Debug)]
pub enum UpdateError {
    LoginRefused(RefuseReason),
    IoError(Backtrace, IoError),
}

impl Error for UpdateError {}

impl Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            UpdateError::LoginRefused(reason) => {
                write!(f, "Login refused with reason: {:?}", reason)
            }
            UpdateError::IoError(_bt, e) => write!(f, "Internal IoError: {}", e),
        }
    }
}

impl From<IoError> for UpdateError {
    fn from(e: IoError) -> Self {
        UpdateError::IoError(Backtrace::new(), e)
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum RefuseReason {
    NotRefused = 0,
    AlreadyOnline = 1,
    Pending = 2,
    OptIn = 3,
    Banned = 4,
    ServerFull = 5,
}

#[derive(Debug, Clone)]
pub struct Player {
    id: i32,
    name: String,
    online: bool,
    ping: f32,
    account: u32,
}

impl Player {
    fn update_ping(&mut self, packet: &Packet) -> Result<(), IoError> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        self.ping = reader.read_single()?;
        Ok(())
    }
}

impl TryFrom<&Packet> for Player {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Player {
            id: i32::from(packet.base_address),
            account: packet.id,
            name: reader.read_string()?,
            online: reader.read_bool()?,
            ping: reader.read_single()?,
        })
    }
}
