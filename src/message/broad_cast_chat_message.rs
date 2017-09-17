
use std::fmt;
use std::sync::Arc;

use Error;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use message::any_chat_message::prelude::*;

pub struct BroadCastChatMessage {
    data:   ChatMessageData,
    to:     Arc<UniverseGroup>,
    message:String,
}

impl BroadCastChatMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<BroadCastChatMessage, Error> {
        Ok(BroadCastChatMessage {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.universe_group(reader.read_u16()?)?,
            message:reader.read_string()?,
        })
    }

    pub fn to(&self) -> &Arc<UniverseGroup> {
        &self.to
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for BroadCastChatMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl ChatMessage for BroadCastChatMessage {
    fn from(&self) -> &Arc<Player> {
        self.data.from()
    }
}

impl fmt::Display for BroadCastChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] <U: {}> {}", self.timestamp(), self.to().name(), self.message())
    }
}