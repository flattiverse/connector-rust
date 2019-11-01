use std::error::Error;
use std::fmt::Display;
use std::io::Error as IoError;
use std::mem::replace;

use backtrace::Backtrace;

use crate::entity::command_id;
use crate::entity::Universe;
use crate::io::BinaryReader;
use crate::num_traits::FromPrimitive;
use crate::packet::Packet;
use crate::players::{Player, Team};

const DEFAULT_PLAYERS: usize = 16;
const DEFAULT_UNIVERSES: usize = 16;

#[derive(Clone)]
pub struct State {
    players: Vec<Option<Player>>,
    universes: Vec<Option<Universe>>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            players: vec_of_none!(DEFAULT_PLAYERS),
            universes: vec_of_none!(DEFAULT_UNIVERSES),
        }
    }

    pub fn universe(&self, index: usize) -> Option<&Universe> {
        self.universes.get(index).and_then(Option::as_ref)
    }

    pub(crate) fn update(&mut self, packet: &Packet) -> Result<Option<Event>, UpdateError> {
        debug!(
            "Updating state with command 0x{:02x} and base_address 0x{:02x}",
            packet.command, packet.base_address
        );
        Ok(Some(match packet.command {
            command_id::S2C_PLAYER_REMOVED => self.remove_player(packet)?,
            command_id::S2C_NEW_PLAYER => self.new_player(packet)?,
            command_id::S2C_PLAYER_DEFRAGMENTED => self.player_defragmented(packet)?,
            command_id::S2C_PLAYER_PING_UPDATE => self.player_ping_update(packet)?,
            command_id::S2C_LOGIN_RESPONSE => {
                if packet.helper != 0u8 {
                    return Err(UpdateError::LoginRefused(
                        RefuseReason::from_u8(packet.helper).expect("Failed to parse RefuseReason"),
                    ));
                } else {
                    return Ok(Some(Event::LoginCompleted));
                }
            }
            command_id::S2C_UNIVERSE_META_INFO_UPDATED => self.update_universe(packet)?,
            command_id::S2C_UNIVERSE_TEAM_META_INFO_UPDATE => self.update_universe_team(packet)?,
            command => {
                warn!("Unknown command: 0x{:02x}", command);
                return Ok(None);
            }
        }))
    }

    fn remove_player(&mut self, packet: &Packet) -> Result<Event, UpdateError> {
        let index = usize::from(packet.base_address);
        debug!("Going to forget player at index {}", index);
        Ok(Event::PlayerRemoved(
            index,
            replace(&mut self.players[index], None),
        ))
    }

    fn new_player(&mut self, packet: &Packet) -> Result<Event, UpdateError> {
        debug!("Going to add a new player at index {}", packet.base_address);
        expand_vec_of_none_if_necessary!(self.players, usize::from(packet.base_address));
        let player = map_payload_with_try_from!(packet, Player);
        let index = usize::from(packet.base_address);
        debug!("Received player: {:#?}", player);
        self.players[index] = player;
        Ok(Event::NewPlayer(
            index,
            self.players[index].as_ref().unwrap(),
        ))
    }

    fn player_defragmented(&mut self, packet: &Packet) -> Result<Event, UpdateError> {
        let index_new = usize::from(packet.base_address);
        debug!("Going to move player to new index {}", index_new);
        let index_old = usize::from((&mut packet.payload() as &mut dyn BinaryReader).read_u16()?);
        let player = replace(&mut self.players[index_old], None);
        debug!(
            "Player is moved from old index {}: {:#?}",
            index_old, player
        );
        self.players[index_new] = player;
        Ok(Event::PlayerDefragmented(
            index_old,
            index_new,
            self.players[index_new].as_ref().unwrap(),
        ))
    }

    fn player_ping_update(&mut self, packet: &Packet) -> Result<Event, UpdateError> {
        let index = usize::from(packet.base_address);
        debug!("Going to update ping for player at index {}", index);
        (&mut self.players)[index]
            .as_mut()
            .expect("Invalid player index for ping update")
            .update_ping(packet)?;
        Ok(Event::PingUpdated(
            index,
            self.players[index].as_ref().unwrap(),
        ))
    }

    fn update_universe(&mut self, packet: &Packet) -> Result<Event, UpdateError> {
        debug!(
            "Going to update universe at index {}, delete={}",
            packet.base_address,
            packet.payload.is_none()
        );
        let index = usize::from(packet.base_address);
        let universe = map_payload_with_try_from!(packet, Universe);
        debug!("Received universe {:#?}", universe);
        expand_vec_of_none_if_necessary!(self.universes, index);
        self.universes[index] = universe;
        Ok(Event::UniverseMetaInfoUpdated(
            index,
            self.universes[index].as_ref(),
        ))
    }

    fn update_universe_team(&mut self, packet: &Packet) -> Result<Event, UpdateError> {
        debug!(
            "Going to update team {} for universe at index {}, delete={}",
            packet.sub_address,
            packet.base_address,
            packet.payload.is_some()
        );
        let index_universe = usize::from(packet.base_address);
        let index_team = usize::from(packet.sub_address);
        let universe = self.universes[index_universe]
            .as_mut()
            .expect("Failed to update universe because unknown for given base_address");
        let team = map_payload_with_try_from!(packet, Team);
        expand_vec_of_none_if_necessary!(universe.teams, index_team);
        universe.teams[index_team] = team;
        Ok(Event::UniverseTeamMetaInfoUpdated(
            index_universe,
            universe,
            index_team,
            universe.teams[index_team].as_ref(),
        ))
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

#[derive(Debug)]
pub enum Event<'a> {
    PlayerRemoved(usize, Option<Player>),
    NewPlayer(usize, &'a Player),
    PlayerDefragmented(usize, usize, &'a Player),
    PingUpdated(usize, &'a Player),
    LoginCompleted,
    UniverseMetaInfoUpdated(usize, Option<&'a Universe>),
    UniverseTeamMetaInfoUpdated(usize, &'a Universe, usize, Option<&'a Team>),
    UniverseGalaxyMetaInfoUpdated(),
}

#[derive(Debug)]
pub struct PlayerJoinedEvent<'a> {
    player: &'a Player,
    universe: &'a Universe,
}
