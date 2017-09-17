
use std::fmt;
use std::sync::Arc;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_system_message::prelude::*;


pub struct MOTDMessage {
    data: SystemMessageData,
}

impl MOTDMessage {
    pub fn from_packet(arc: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<MOTDMessage, Error> {
        Ok(MOTDMessage {
            data: SystemMessageData::from_packet(arc, packet, reader)?,
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for MOTDMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl SystemMessage for MOTDMessage {
    fn message(&self) -> &str {
        self.data.message()
    }
}

impl fmt::Display for MOTDMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for line in self.message().lines() {
            if first {
               first = false;
            } else {
                writeln!(f)?;
            }
            write!(f, "[{}] -MOTD- {}", self.timestamp(), line)?
        }
        Ok(())
    }
}