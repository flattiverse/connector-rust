
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
use message::PlayerUnitDeceasedMessage;
use message::PlayerUnitDeceasedMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerUnitLoggedOffMessage);
pub trait PlayerUnitLoggedOffMessage : PlayerUnitDeceasedMessage {

}

pub struct PlayerUnitLoggedOffMessageData {
    data:   PlayerUnitDeceasedMessageData,
}

impl PlayerUnitLoggedOffMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitLoggedOffMessageData, Error> {
        Ok(PlayerUnitLoggedOffMessageData {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitLoggedOffMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitLoggedOffMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitLoggedOffMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitLoggedOffMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitLoggedOffMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitLoggedOffMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitLoggedOffMessageData> + BorrowMut<PlayerUnitLoggedOffMessageData> + PlayerUnitDeceasedMessage> PlayerUnitLoggedOffMessage for T {

}

impl fmt::Display for PlayerUnitLoggedOffMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' logged of.",
            (self as &FlattiverseMessage).timestamp(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().kind(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().name(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit_player().name(),
        )
    }
}