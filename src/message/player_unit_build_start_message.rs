
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use Player;
use UniversalEnumerable;
use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;
use message::PlayerUnitBuildMessage;
use message::PlayerUnitBuildMessageData;

impl_downcast!(PlayerUnitBuildStartMessage);
pub trait PlayerUnitBuildStartMessage : PlayerUnitBuildMessage {
}

pub struct PlayerUnitBuildStartMessageData {
    data:   PlayerUnitBuildMessageData,
}

impl PlayerUnitBuildStartMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitBuildStartMessageData, Error> {
        Ok(PlayerUnitBuildStartMessageData {
            data: PlayerUnitBuildMessageData::from_packet(connector, packet, reader)?,
        })
    }
}


impl Borrow<PlayerUnitBuildMessageData> for PlayerUnitBuildStartMessageData {
    fn borrow(&self) -> &PlayerUnitBuildMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitBuildMessageData> for PlayerUnitBuildStartMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitBuildMessageData {
        &mut self.data
    }
}
impl Borrow<GameMessageData> for PlayerUnitBuildStartMessageData {
    fn borrow(&self) -> &GameMessageData {
        (self.borrow() as &PlayerUnitBuildMessageData).borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitBuildStartMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        (self.borrow_mut() as &mut PlayerUnitBuildMessageData).borrow_mut()
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitBuildStartMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitBuildStartMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitBuildStartMessageData> + BorrowMut<PlayerUnitBuildStartMessageData> + PlayerUnitBuildMessage> PlayerUnitBuildStartMessage for T {

}

impl fmt::Display for PlayerUnitBuildStartMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} {} of {} starts building a {:?} with the name {}.",
               (self as &PlayerUnitBuildMessage).timestamp(),
               (self as &PlayerUnitBuildMessage).player_unit_builder().read().unwrap().kind(),
               (self as &PlayerUnitBuildMessage).player_unit_builder().read().unwrap().name(),
               (self as &PlayerUnitBuildMessage).player().read().unwrap().name(),
               (self as &PlayerUnitBuildMessage).player_unit().read().unwrap().kind(),
               (self as &PlayerUnitBuildMessage).player_unit().read().unwrap().name(),
        )
    }
}