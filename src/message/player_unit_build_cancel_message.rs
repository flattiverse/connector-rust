
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_player_unit_build_message::prelude::*;

pub struct PlayerUnitBuildCancelledMessage {
    data: PlayerUnitBuildMessageData,
}

impl PlayerUnitBuildCancelledMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitBuildCancelledMessage, Error> {
        Ok(PlayerUnitBuildCancelledMessage {
            data: PlayerUnitBuildMessageData::from_packet(connector, packet, reader)?,
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitBuildCancelledMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitBuildCancelledMessage {

}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PlayerUnitBuildMessage for PlayerUnitBuildCancelledMessage {
    fn player(&self) -> &Arc<Player> {
        self.data.player()
    }

    fn player_unit(&self) -> &Arc<ControllableInfo> {
        self.data.player_unit()
    }

    fn player_unit_builder(&self) -> &Arc<ControllableInfo> {
        self.data.player_unit_builder()
    }
}


impl fmt::Display for PlayerUnitBuildCancelledMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} {} of {} cancelled building a {:?} with the name {}.",
               self.timestamp(),
               self.player_unit_builder().kind(),
               self.player_unit_builder().name(),
               self.player().name(),
               self.player_unit().kind(),
               self.player_unit().name(),
        )
    }
}