
use std::fmt;
use std::sync::Arc;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_player_unit_deceased_message::prelude::*;

pub trait PlayerUnitDeceasedMessage : GameMessage {

    fn deceased_player_unit_player(&self) -> &Arc<Player>;

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo>;
}

pub struct PlayerUnitDeceasedMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
}

impl PlayerUnitDeceasedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitDeceasedMessageData, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitDeceasedMessageData {
            data,
            player: player.clone(),
            info:   {
                let index = reader.read_unsigned_byte()?;
                player.controllable_info(index).ok_or(Error::InvalidControllableInfo(index))?
            }
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitDeceasedMessageData {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitDeceasedMessageData {

}

impl PlayerUnitDeceasedMessage for PlayerUnitDeceasedMessageData {
    fn deceased_player_unit_player(&self) -> &Arc<Player> {
        &self.player
    }

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo> {
        &self.info
    }
}

impl fmt::Display for PlayerUnitDeceasedMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}