
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;

use message::FlattiverseMessage;
use message::FlattiverseMessageData;

use net::Packet;
use net::BinaryReader;


pub trait GameMessage : FlattiverseMessage {

}

pub struct GameMessageData {
    data: FlattiverseMessageData
}


impl GameMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<GameMessageData, Error> {
        Ok(GameMessageData {
            data:   FlattiverseMessageData::from_packet(connector, packet, reader)?
        })
    }
}

impl Borrow<FlattiverseMessageData> for GameMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        &self.data
    }
}
impl BorrowMut<FlattiverseMessageData> for GameMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<GameMessageData> + FlattiverseMessage> GameMessage for T {

}


impl fmt::Display for GameMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}