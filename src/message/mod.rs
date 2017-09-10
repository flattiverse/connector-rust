
mod motd_message;
mod game_message;
mod chat_message;
mod system_message;
mod binary_chat_message;
mod unicast_chat_message;
mod team_cast_chat_message;
mod broad_cast_chat_message;
mod player_unit_reset_message;
mod player_unit_deceased_message;
mod player_unit_logged_off_message;
mod player_unit_shot_by_unit_message;
mod player_unit_shot_by_player_message;
mod player_unit_committed_suicide_message;
mod player_unit_collided_with_unit_message;
mod player_unit_collided_with_player_message;

pub use self::motd_message::*;
pub use self::game_message::*;
pub use self::chat_message::*;
pub use self::system_message::*;
pub use self::binary_chat_message::*;
pub use self::unicast_chat_message::*;
pub use self::team_cast_chat_message::*;
pub use self::broad_cast_chat_message::*;
pub use self::player_unit_reset_message::*;
pub use self::player_unit_deceased_message::*;
pub use self::player_unit_logged_off_message::*;
pub use self::player_unit_shot_by_unit_message::*;
pub use self::player_unit_shot_by_player_message::*;
pub use self::player_unit_committed_suicide_message::*;
pub use self::player_unit_collided_with_unit_message::*;
pub use self::player_unit_collided_with_player_message::*;



use std::fmt;
use std::fmt::Display;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use net::Packet;
use net::BinaryReader;
use dotnet::DateTime;


pub trait FlattiverseMessage : Display + Send + Sync {
    fn timestamp(&self) -> &DateTime;
}

pub struct FlattiverseMessageData {
    timestamp: DateTime,
}

impl FlattiverseMessageData {
    fn from_packet(_: &Arc<Connector>, _: &Packet, reader: &mut BinaryReader) -> Result<FlattiverseMessageData, Error> {
        Ok(FlattiverseMessageData {
            timestamp: DateTime::from_ticks(reader.read_i64()?),
        })
    }
}

impl<T: 'static + Borrow<FlattiverseMessageData> + BorrowMut<FlattiverseMessageData> + Display + Send + Sync> FlattiverseMessage for T {
    fn timestamp(&self) -> &DateTime {
        &self.borrow().timestamp
    }
}

impl fmt::Display for FlattiverseMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}



pub fn from_reader(connector: &Arc<Connector>, packet: &Packet) -> Result<Box<FlattiverseMessage>, Error> {
    let path_sub = packet.path_sub();
    let reader = &mut packet.read() as &mut BinaryReader;

    match path_sub {
        0x00 => Ok(Box::new(SystemMessageData                           ::from_packet(connector, packet, reader)?)),
        0x01 => Ok(Box::new(UnicastChatMessageData                      ::from_packet(connector, packet, reader)?)),
        0x02 => Ok(Box::new(TeamCastChatMessageData                     ::from_packet(connector, packet, reader)?)),
        0x03 => Ok(Box::new(BroadCastChatMessageData                    ::from_packet(connector, packet, reader)?)),
        0x04 => Ok(Box::new(BroadCastChatMessageData                    ::from_packet(connector, packet, reader)?)),
        0x08 => Ok(Box::new(MOTDMessageData                             ::from_packet(connector, packet, reader)?)),
        0x10 => Ok(Box::new(PlayerUnitCommittedSuicideMessageData       ::from_packet(connector, packet, reader)?)),
        0x11 => Ok(Box::new(PlayerUnitCollidedWithUnitMessageData       ::from_packet(connector, packet, reader)?)),
        0x12 => Ok(Box::new(PlayerUnitCollidedWithPlayerUnitMessageData ::from_packet(connector, packet, reader)?)),
        0x13 => Ok(Box::new(PlayerUnitShotByUnitMessageData             ::from_packet(connector, packet, reader)?)),
        0x14 => Ok(Box::new(PlayerUnitShotByPlayerUnitMessageData       ::from_packet(connector, packet, reader)?)),
        0x15 => Ok(Box::new(PlayerUnitLoggedOffMessageData              ::from_packet(connector, packet, reader)?)),
        0x16 => Ok(Box::new(PlayerUnitResetMessageData                  ::from_packet(connector, packet, reader)?)),
        _ => Err(Error::UnknownMessageType)
    }
}