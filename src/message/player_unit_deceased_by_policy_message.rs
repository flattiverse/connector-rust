
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::ControllableInfo;

use crate::message::any_player_unit_deceased_message::prelude::*;

pub struct PlayerUnitDeceasedByPolicyMessage {
    data: PlayerUnitDeceasedMessageData
}

impl PlayerUnitDeceasedByPolicyMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitDeceasedByPolicyMessage, Error> {
        Ok(PlayerUnitDeceasedByPolicyMessage {
            data: PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitDeceasedByPolicyMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitDeceasedByPolicyMessage {

}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PlayerUnitDeceasedMessage for PlayerUnitDeceasedByPolicyMessage {
    fn deceased_player_unit_player(&self) -> &Arc<Player> {
        self.data.deceased_player_unit_player()
    }

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo> {
        self.data.deceased_player_unit()
    }
}

impl fmt::Display for PlayerUnitDeceasedByPolicyMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' has been killed because it tried to leave the universe and KillLonesomeUnits is set to true.",
            self.timestamp(),
            self.deceased_player_unit().kind(),
            self.deceased_player_unit().name(),
            self.deceased_player_unit_player().name(),
        )
    }
}