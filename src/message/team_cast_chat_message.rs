
use std::fmt;
use std::sync::Arc;

use Team;
use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_chat_message::prelude::*;


pub struct TeamCastChatMessage {
    data:   ChatMessageData,
    to:     Arc<Team>,
    message:String,
}

impl TeamCastChatMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TeamCastChatMessage, Error> {
        Ok(TeamCastChatMessage {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     {
                let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
                let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
                group.team(reader.read_unsigned_byte()?)?
            },
            message:reader.read_string()?,
        })
    }

    pub fn to(&self) -> &Arc<Team> {
        &self.to
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for TeamCastChatMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl ChatMessage for TeamCastChatMessage {
    fn from(&self) -> &Arc<Player> {
        self.data.from()
    }
}

impl fmt::Display for TeamCastChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] <T: {}> {}", self.timestamp(), self.to().name(), self.message())
    }
}