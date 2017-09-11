
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use message::GameMessageData;
use message::FlattiverseMessageData;
use message::PlayerUnitBuildMessage;
use message::PlayerUnitBuildMessageData;

downcast!(PlayerUnitBuildFinishedMessage);
pub trait PlayerUnitBuildFinishedMessage : PlayerUnitBuildMessage {
}

pub struct PlayerUnitBuildFinishedMessageData {
    data:   PlayerUnitBuildMessageData,
}

impl PlayerUnitBuildFinishedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitBuildFinishedMessageData, Error> {
        Ok(PlayerUnitBuildFinishedMessageData {
            data: PlayerUnitBuildMessageData::from_packet(connector, packet, reader)?,
        })
    }
}


impl Borrow<PlayerUnitBuildMessageData> for PlayerUnitBuildFinishedMessageData {
    fn borrow(&self) -> &PlayerUnitBuildMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitBuildMessageData> for PlayerUnitBuildFinishedMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitBuildMessageData {
        &mut self.data
    }
}
impl Borrow<GameMessageData> for PlayerUnitBuildFinishedMessageData {
    fn borrow(&self) -> &GameMessageData {
        (self.borrow() as &PlayerUnitBuildMessageData).borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitBuildFinishedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        (self.borrow_mut() as &mut PlayerUnitBuildMessageData).borrow_mut()
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitBuildFinishedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitBuildFinishedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitBuildFinishedMessageData> + BorrowMut<PlayerUnitBuildFinishedMessageData> + PlayerUnitBuildMessage> PlayerUnitBuildFinishedMessage for T {

}

impl fmt::Display for PlayerUnitBuildFinishedMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} {} of {} finished building the {:?} with the name {}.",
               (self as &PlayerUnitBuildMessage).timestamp(),
               (self as &PlayerUnitBuildMessage).player_unit_builder().read().unwrap().kind(),
               (self as &PlayerUnitBuildMessage).player_unit_builder().read().unwrap().name(),
               (self as &PlayerUnitBuildMessage).player().read().unwrap().name(),
               (self as &PlayerUnitBuildMessage).player_unit().read().unwrap().kind(),
               (self as &PlayerUnitBuildMessage).player_unit().read().unwrap().name(),
        )
    }
}