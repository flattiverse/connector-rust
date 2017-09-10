
mod motd_message;
mod system_message;
mod chat_message;
mod unicast_chat_message;

pub use self::motd_message::*;
pub use self::system_message::*;
pub use self::chat_message::*;
pub use self::unicast_chat_message::*;


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
    fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<FlattiverseMessageData, Error> {
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
        0x00 => Ok(Box::new(SystemMessageData       ::from_packet(connector, packet, reader)?)),
        0x01 => Ok(Box::new(UnicastChatMessageData  ::from_packet(connector, packet, reader)?)),
        0x08 => Ok(Box::new(MOTDMessageData         ::from_packet(connector, packet, reader)?)),
        _ => Err(Error::UnknownMessageType)
    }
}