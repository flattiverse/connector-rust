
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;


use message::SystemMessage;
use message::SystemMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(MOTDMessage);
pub trait MOTDMessage : SystemMessage {

}

pub struct MOTDMessageData {
    data: SystemMessageData,
    message: String,
}

impl MOTDMessageData {
    pub fn from_packet(arc: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<MOTDMessageData, Error> {
        Ok(MOTDMessageData {
            data:       SystemMessageData::from_packet(arc, packet, reader)?,
            message:    reader.read_string()?,
        })
    }
}

impl Borrow<SystemMessageData> for MOTDMessageData {
    fn borrow(&self) -> &SystemMessageData {
        &self.data
    }
}
impl BorrowMut<SystemMessageData> for MOTDMessageData {
    fn borrow_mut(&mut self) -> &mut SystemMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for MOTDMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        self.data.borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for MOTDMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        self.data.borrow_mut()
    }
}



impl<T: 'static + Borrow<MOTDMessageData> + BorrowMut<MOTDMessageData> + SystemMessage> MOTDMessage for T {

}

impl fmt::Display for MOTDMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.message.lines() {
            writeln!(f, "[{}] -MOTD- {}", (self.borrow() as &FlattiverseMessageData).timestamp(), line)?
        }
        Ok(())
    }
}