
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Connector;
use crate::UniverseGroup;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub struct UniverseGroupResetMessage {
    data:   GameMessageData,
    group:  Arc<UniverseGroup>,
}

impl UniverseGroupResetMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<UniverseGroupResetMessage, Error> {
        Ok(UniverseGroupResetMessage {
            data:  GameMessageData::from_packet(connector, packet, reader)?,
            group: connector.universe_group(reader.read_u16()?)?,
        })
    }
    
    pub fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.group
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for UniverseGroupResetMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for UniverseGroupResetMessage {

}

impl fmt::Display for UniverseGroupResetMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] UniverseGroup {} has been reset.",
            self.timestamp(),
            self.group.name(),
        )
    }
}