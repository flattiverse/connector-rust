
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;


use message::FlattiverseMessage;
use message::FlattiverseMessageData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(SystemMessage);
pub trait SystemMessage : FlattiverseMessage {
    fn message(&self) -> &String;
}

pub struct SystemMessageData {
    data: FlattiverseMessageData,
    message: String,
}

impl SystemMessageData {
    pub fn from_packet(arc: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<SystemMessageData, Error> {
        Ok(SystemMessageData {
            data:       FlattiverseMessageData::from_packet(arc, packet, reader)?,
            message:    reader.read_string()?,
        })
    }
}

impl Borrow<FlattiverseMessageData> for SystemMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        &self.data
    }
}
impl BorrowMut<FlattiverseMessageData> for SystemMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<SystemMessageData> + BorrowMut<SystemMessageData> + FlattiverseMessage> SystemMessage for T {
    fn message(&self) -> &String {
        &self.borrow().message
    }
}

impl fmt::Display for SystemMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] - SYSTEM - {}", self.timestamp(), self.message())
    }
}