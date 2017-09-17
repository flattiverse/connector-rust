
use std::fmt;
use std::sync::Arc;

use Error;
use Player;
use Connector;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use unit::ControllableInfo;

use message::any_player_unit_deceased_message::prelude::*;

pub struct PlayerUnitCollidedWithPlayerUnitMessage {
    data:   PlayerUnitDeceasedMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
}

impl PlayerUnitCollidedWithPlayerUnitMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitCollidedWithPlayerUnitMessage, Error> {
        let data = PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitCollidedWithPlayerUnitMessage {
            data,
            player: player.clone(),
            info:   {
                player.controllable_info(reader.read_unsigned_byte()?).ok_or(Error::ControllableInfoNotAvailable)?
            }
        })
    }

    pub fn collider_unit_player(&self) -> &Arc<Player> {
        &self.player
    }

    pub fn collider_unit(&self) -> &Arc<ControllableInfo> {
        &self.info
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitCollidedWithPlayerUnitMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitCollidedWithPlayerUnitMessage {

}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PlayerUnitDeceasedMessage for PlayerUnitCollidedWithPlayerUnitMessage {
    fn deceased_player_unit_player(&self) -> &Arc<Player> {
        self.data.deceased_player_unit_player()
    }

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo> {
        self.data.deceased_player_unit()
    }
}

impl fmt::Display for PlayerUnitCollidedWithPlayerUnitMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' has a deadly collision with {:?} from '{}'.",
            self.timestamp(),
            self.deceased_player_unit().kind(),
            self.deceased_player_unit().name(),
            self.deceased_player_unit_player().name(),
            self.info.kind(),
            self.player.name(),
        )
    }
}