
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::UnitKind;
use crate::unit::ControllableInfo;

use crate::message::any_player_unit_deceased_message::prelude::*;

pub struct PlayerUnitCollidedWithUnitMessage {
    data:   PlayerUnitDeceasedMessageData,
    kind:   UnitKind,
    name:   String,
}

impl PlayerUnitCollidedWithUnitMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitCollidedWithUnitMessage, Error> {
        Ok(PlayerUnitCollidedWithUnitMessage {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            kind:   UnitKind::from_id(reader.read_byte()?),
            name:   reader.read_string()?,
        })
    }

    pub fn collider_unit_kind(&self) -> UnitKind {
        self.kind
    }

    pub fn collider_unit_name(&self) -> &str {
        &self.name
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitCollidedWithUnitMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitCollidedWithUnitMessage {

}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PlayerUnitDeceasedMessage for PlayerUnitCollidedWithUnitMessage {
    fn deceased_player_unit_player(&self) -> &Arc<Player> {
        self.data.deceased_player_unit_player()
    }

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo> {
        self.data.deceased_player_unit()
    }
}

impl fmt::Display for PlayerUnitCollidedWithUnitMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' collided with {:?} '{}'.",
            self.timestamp(),
            self.deceased_player_unit().kind(),
            self.deceased_player_unit().name(),
            self.deceased_player_unit_player().name(),
            self.kind,
            self.name
        )
    }
}