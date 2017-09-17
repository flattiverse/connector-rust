
use std::fmt;
use std::sync::Arc;

use Error;
use Player;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_chat_message::prelude::*;

pub struct UnicastChatMessage {
    data:   ChatMessageData,
    to:     Arc<Player>,
    message:String,
}

impl UnicastChatMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<UnicastChatMessage, Error> {
        Ok(UnicastChatMessage {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.player_for(reader.read_u16()?)?,
            message:reader.read_string()?,
        })
    }

    pub fn to(&self) -> &Arc<Player> {
        &self.to
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for UnicastChatMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl ChatMessage for UnicastChatMessage {
    fn from(&self) -> &Arc<Player> {
        self.data.from()
    }
}

impl fmt::Display for UnicastChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] <{}> {}", self.timestamp(), self.from().name(), self.message())
    }
}