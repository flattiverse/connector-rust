
use std::fmt;
use std::sync::Arc;

use Error;
use Player;
use Connector;

use net::Packet;
use net::BinaryReader;

use unit::ControllableInfo;

use message::any_player_unit_build_message::prelude::*;

pub trait PlayerUnitBuildMessage : GameMessage {

    fn player(&self) -> &Arc<Player>;

    fn player_unit(&self) -> &Arc<ControllableInfo>;

    fn player_unit_builder(&self) -> &Arc<ControllableInfo>;
}

pub(crate) struct PlayerUnitBuildMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    unit:   Arc<ControllableInfo>,
    builder:Arc<ControllableInfo>,
}

impl PlayerUnitBuildMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitBuildMessageData, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let unit    = player.controllable_info(reader.read_unsigned_byte()?).ok_or(Error::ControllableInfoNotAvailable)?;
        let builder = player.controllable_info(reader.read_unsigned_byte()?).ok_or(Error::ControllableInfoNotAvailable)?;

        Ok(PlayerUnitBuildMessageData {
            data,
            player,
            unit,
            builder
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitBuildMessageData {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitBuildMessageData {

}

impl PlayerUnitBuildMessage for PlayerUnitBuildMessageData {
    fn player(&self) -> &Arc<Player> {
        &self.player
    }

    fn player_unit(&self) -> &Arc<ControllableInfo> {
        &self.unit
    }

    fn player_unit_builder(&self) -> &Arc<ControllableInfo> {
        &self.builder
    }
}

impl fmt::Display for PlayerUnitBuildMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}