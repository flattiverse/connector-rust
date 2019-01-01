
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::Message;
use crate::message::MessageData;

use crate::message::any_system_message::prelude::*;

pub trait SystemMessage : Message {
    fn message(&self) -> &str;
}

pub struct SystemMessageData {
    data: MessageData,
    message: String,
}

impl SystemMessageData {
    pub fn from_packet(arc: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<SystemMessageData, Error> {
        Ok(SystemMessageData {
            data:       MessageData::from_packet(arc, packet, reader)?,
            message:    reader.read_string()?,
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for SystemMessageData {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

impl SystemMessage for SystemMessageData {
    fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for SystemMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] - SYSTEM - {}", self.timestamp(), self.message())
    }
}