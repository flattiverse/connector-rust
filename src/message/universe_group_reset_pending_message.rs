
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::TimeSpan;
use crate::Connector;
use crate::UniverseGroup;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub struct UniverseGroupResetPendingMessage {
    data:   GameMessageData,
    group:  Arc<UniverseGroup>,
    time:   TimeSpan,
}

impl UniverseGroupResetPendingMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<UniverseGroupResetPendingMessage, Error> {
        Ok(UniverseGroupResetPendingMessage {
            data:  GameMessageData::from_packet(connector, packet, reader)?,
            group: connector.universe_group(reader.read_u16()?)?,
            time:  TimeSpan::from_reader(reader)?,
        })
    }

    pub fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.group
    }

    pub fn remaining_time(&self) -> &TimeSpan {
        &self.time
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for UniverseGroupResetPendingMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for UniverseGroupResetPendingMessage {

}

impl fmt::Display for UniverseGroupResetPendingMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] UniverseGroup {} pending reset in {} seconds.",
            self.timestamp(),
            self.group.name(),
            self.time.seconds()+1,
        )
    }
}