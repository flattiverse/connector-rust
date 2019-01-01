
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Player;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_chat_message::prelude::*;

pub struct BinaryChatMessage {
    data:   ChatMessageData,
    to:     Arc<Player>,
    message:Vec<u8>,
}

impl BinaryChatMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<BinaryChatMessage, Error> {
        Ok(BinaryChatMessage {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.player_for(reader.read_u16()?)?,
            message:{
                let count = reader.read_unsigned_byte()? as usize;
                reader.read_bytes(count)?
            },
        })
    }

    pub fn to(&self) -> &Arc<Player> {
        &self.to
    }

    pub fn message(&self) -> &Vec<u8> {
        &self.message
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for BinaryChatMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl ChatMessage for BinaryChatMessage {
    fn from(&self) -> &Arc<Player> {
        self.data.from()
    }
}

impl fmt::Display for BinaryChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] -{}- 0x", self.timestamp(), self.from().name())?;
        for byte in self.message.iter() {
            write!(f, "{:x}", byte)?;
        }
        Ok(())
    }
}