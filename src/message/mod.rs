
mod motd_message;
mod system_message;
mod chat_message;

pub use self::motd_message::*;
pub use self::system_message::*;
pub use self::chat_message::*;


use std::fmt::Display;

use std::sync::Arc;

use Error;
use Connector;
use net::Packet;
use net::BinaryReader;
use dotnet::DateTime;


pub trait FlattiverseMessage : Display + Send + Sync {
    fn timestamp(&self) -> &DateTime;

    fn from_packet(connector: Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Self, Error> where Self:Sized;
}


pub fn from_reader(connector: Arc<Connector>, packet: &Packet) -> Result<Box<FlattiverseMessage>, Error> {
    let path_sub = packet.path_sub();
    let reader = &mut packet.read() as &mut BinaryReader;

    match path_sub {
        0x00 => Ok(Box::new(SystemMessageData::from_packet(connector, packet, reader)?)),
        0x08 => Ok(Box::new(MOTDMessageData::from_packet(connector, packet, reader)?)),
        _ => Err(Error::UnknownMessageType)
    }
}