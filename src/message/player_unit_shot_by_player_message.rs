
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

pub struct PlayerUnitShotByPlayerUnitMessage {
    data:   PlayerUnitDeceasedMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
}

impl PlayerUnitShotByPlayerUnitMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitShotByPlayerUnitMessage, Error> {
        Ok(PlayerUnitShotByPlayerUnitMessage {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            player: connector.player_for(reader.read_u16()?)?,
            info:   {
                let player = connector.player_for(reader.read_u16()?)?;
                player.controllable_info(reader.read_unsigned_byte()?).ok_or(Error::ControllableInfoNotAvailable)?
            }
        })
    }

    pub fn aggressor_unit_player(&self) -> &Arc<Player> {
        &self.player
    }

    pub fn aggressor_unit(&self) -> &Arc<ControllableInfo> {
        &self.info
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitShotByPlayerUnitMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitShotByPlayerUnitMessage {

}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PlayerUnitDeceasedMessage for PlayerUnitShotByPlayerUnitMessage {
    fn deceased_player_unit_player(&self) -> &Arc<Player> {
        self.data.deceased_player_unit_player()
    }

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo> {
        self.data.deceased_player_unit()
    }
}

impl fmt::Display for PlayerUnitShotByPlayerUnitMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' has been killed by {:?} from '{}'.",
            self.timestamp(),
            self.deceased_player_unit().kind(),
            self.deceased_player_unit().name(),
            self.deceased_player_unit_player().name(),
            self.info.kind(),
            self.player.name(),
        )
    }
}