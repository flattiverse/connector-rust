
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

use Error;
use DateTime;
use Connector;
use FlattiverseMessage;

use net::Packet;
use net::BinaryReader;

pub struct SystemMessageData {
    timestamp: DateTime,
    message: String,
}

pub trait SystemMessage : FlattiverseMessage {
    fn message(&self) -> &String;
}

impl FlattiverseMessage for SystemMessageData {
    fn timestamp(&self) -> &DateTime {
        &self.timestamp
    }
    fn from_packet(connector: Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Self, Error> {
        Ok(SystemMessageData {
            timestamp: DateTime::from_ticks(reader.read_i64()?),
            message:   reader.read_string()?
        })
    }
}

impl SystemMessage for SystemMessageData {
    fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for SystemMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] - SYSTEM - {}", self.timestamp(), self.message())
    }
}