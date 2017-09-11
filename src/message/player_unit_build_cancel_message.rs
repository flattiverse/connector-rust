
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

downcast!(PlayerUnitBuildCancelledMessage);
pub trait PlayerUnitBuildCancelledMessage : PlayerUnitBuildMessage {
}

pub struct PlayerUnitBuildCancelledMessageData {
    data:   PlayerUnitBuildMessageData,
}

impl PlayerUnitBuildCancelledMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitBuildCancelledMessageData, Error> {
        Ok(PlayerUnitBuildCancelledMessageData {
            data: PlayerUnitBuildMessageData::from_packet(connector, packet, reader)?,
        })
    }
}


impl Borrow<PlayerUnitBuildMessageData> for PlayerUnitBuildCancelledMessageData {
    fn borrow(&self) -> &PlayerUnitBuildMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitBuildMessageData> for PlayerUnitBuildCancelledMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitBuildMessageData {
        &mut self.data
    }
}
impl Borrow<GameMessageData> for PlayerUnitBuildCancelledMessageData {
    fn borrow(&self) -> &GameMessageData {
        (self.borrow() as &PlayerUnitBuildMessageData).borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitBuildCancelledMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        (self.borrow_mut() as &mut PlayerUnitBuildMessageData).borrow_mut()
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitBuildCancelledMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitBuildCancelledMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitBuildCancelledMessageData> + BorrowMut<PlayerUnitBuildCancelledMessageData> + PlayerUnitBuildMessage> PlayerUnitBuildCancelledMessage for T {

}

impl fmt::Display for PlayerUnitBuildCancelledMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} {} of {} cancelled building a {:?} with the name {}.",
               (self as &PlayerUnitBuildMessage).timestamp(),
               (self as &PlayerUnitBuildMessage).player_unit_builder().read().unwrap().kind(),
               (self as &PlayerUnitBuildMessage).player_unit_builder().read().unwrap().name(),
               (self as &PlayerUnitBuildMessage).player().read().unwrap().name(),
               (self as &PlayerUnitBuildMessage).player_unit().read().unwrap().kind(),
               (self as &PlayerUnitBuildMessage).player_unit().read().unwrap().name(),
        )
    }
}