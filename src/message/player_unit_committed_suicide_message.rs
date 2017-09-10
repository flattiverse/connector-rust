
use std::fmt;
use std::fmt::Write;
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

impl_downcast!(PlayerUnitCommittedSuicideMessage);
pub trait PlayerUnitCommittedSuicideMessage : PlayerUnitDeceasedMessage {

}

pub struct PlayerUnitCommittedSuicideMessageData {
    data:   PlayerUnitDeceasedMessageData,
}

impl PlayerUnitCommittedSuicideMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitCommittedSuicideMessageData, Error> {
        Ok(PlayerUnitCommittedSuicideMessageData {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitCommittedSuicideMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitCommittedSuicideMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitCommittedSuicideMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitCommittedSuicideMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitCommittedSuicideMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitCommittedSuicideMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitCommittedSuicideMessageData> + BorrowMut<PlayerUnitCommittedSuicideMessageData> + PlayerUnitDeceasedMessage> PlayerUnitCommittedSuicideMessage for T {

}

impl fmt::Display for PlayerUnitCommittedSuicideMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' committed suicide",
            (self as &FlattiverseMessage).timestamp(),
            match (self as &PlayerUnitDeceasedMessage).deceased_player_unit().read() {
                Err(_) => String::new(),
                Ok(ref read) => {
                    let mut string = String::new();
                    write!(string, "{:?}", read.kind())?;
                    string
                },
            },
            match (self as &PlayerUnitDeceasedMessage).deceased_player_unit().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
            match (self as &PlayerUnitDeceasedMessage).deceased_player_unit_player().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            }
        )
    }
}