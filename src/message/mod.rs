
mod date_time;
mod motd_message;
mod system_message;
mod chat_message;

pub use self::date_time::*;
pub use self::motd_message::*;
pub use self::system_message::*;
pub use self::chat_message::*;




use std::rc::Rc;
use std::fmt::Display;

use std::sync::Arc;
use std::sync::Mutex;

use Error;
use ConnectorData;
use net::Packet;
use net::BinaryReader;



pub trait FlattiverseMessage : Display {
    fn timestamp(&self) -> &DateTime;

    fn from_packet(data: Arc<ConnectorData>, packet: &Packet, reader: &mut BinaryReader) -> Result<Self, Error> where Self:Sized;
}



pub fn from_reader(data: Arc<ConnectorData>, packet: &Packet) -> Result<Box<FlattiverseMessage>, Error> {
    let path_sub = packet.path_sub();
    let reader = &mut packet.read() as &mut BinaryReader;

    match path_sub {
        0x00 => Ok(Box::new(SystemMessageData::from_packet(data, packet, reader)?)),
        0x08 => Ok(Box::new(MOTDMessageData::from_packet(data, packet, reader)?)),
        _ => Err(Error::UnknownMessageType)
    }
}