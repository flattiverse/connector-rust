
use Error;
use ConnectorData;
use net::Packet;
use net::BinaryReader;
use message::DateTime;
use message::FlattiverseMessage;
use message::SystemMessage;
use message::SystemMessageData;

use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct MOTDMessageData {
    system_message: SystemMessageData
}

pub trait MOTDMessage : SystemMessage {

}

impl FlattiverseMessage for MOTDMessageData {
    fn timestamp(&self) -> &DateTime {
        &self.system_message.timestamp()
    }

    fn from_packet(data: Arc<ConnectorData>, packet: &Packet, reader: &mut BinaryReader) -> Result<Self, Error> {
        Ok(MOTDMessageData {
            system_message: SystemMessageData::from_packet(data, packet, reader)?
        })
    }
}

impl SystemMessage for MOTDMessageData {
    fn message(&self) -> &String {
        &self.system_message.message()
    }
}

impl MOTDMessage for MOTDMessageData {

}

impl fmt::Display for MOTDMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.message().lines() {
            writeln!(f, "[{}] -MOTD- {}", self.timestamp(), line)?
        }
        Ok(())
    }
}