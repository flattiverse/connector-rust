
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use Player;
use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessageData;

downcast!(PlayerUnitBuildMessage);
pub trait PlayerUnitBuildMessage : GameMessage {

    fn player(&self) -> &Arc<Player>;

    fn player_unit(&self) -> &Arc<ControllableInfo>;

    fn player_unit_builder(&self) -> &Arc<ControllableInfo>;
}

pub struct PlayerUnitBuildMessageData {
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

impl Borrow<GameMessageData> for PlayerUnitBuildMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitBuildMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitBuildMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitBuildMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitBuildMessageData> + BorrowMut<PlayerUnitBuildMessageData> + GameMessage> PlayerUnitBuildMessage for T {
    fn player(&self) -> &Arc<Player> {
        &self.borrow().player
    }

    fn player_unit(&self) -> &Arc<ControllableInfo> {
        &self.borrow().unit
    }

    fn player_unit_builder(&self) -> &Arc<ControllableInfo> {
        &self.borrow().builder
    }
}

impl fmt::Display for PlayerUnitBuildMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}