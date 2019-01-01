
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::ControllableInfo;

use crate::message::any_game_message::prelude::*;

pub struct PlayerUnitContinuedMessage {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
}

impl PlayerUnitContinuedMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitContinuedMessage, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitContinuedMessage {
            data,
            player: player.clone(),
            info:   {
                let index = reader.read_unsigned_byte()?;
                player.controllable_info(index).ok_or_else(|| Error::InvalidControllableInfo(index))?
            }
        })
    }

    pub fn player_unit_player(&self) -> &Arc<Player> {
        &self.player
    }

    pub fn player_unit(&self) -> &Arc<ControllableInfo> {
        &self.info
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitContinuedMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitContinuedMessage {

}

impl fmt::Display for PlayerUnitContinuedMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' continued game.",
            self.timestamp(),
            self.player_unit().kind(),
            self.player_unit().name(),
            self.player_unit_player().name(),
        )
    }
}