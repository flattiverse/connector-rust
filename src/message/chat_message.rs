
use std::fmt;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use DateTime;
use Connector;

use message::FlattiverseMessage;
use message::FlattiverseMessageData;

use net::Packet;
use net::BinaryReader;


pub trait ChatMessage : FlattiverseMessage {
    fn from(&self) -> &Weak<RwLock<Player>>;
}

pub struct ChatMessageData {
    data: FlattiverseMessageData,
    from: Weak<RwLock<Player>>
}


impl ChatMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<ChatMessageData, Error> {
        Ok(ChatMessageData {
            data:   FlattiverseMessageData::from_packet(connector, packet, reader)?,
            from:   connector.weak_player_for(reader.read_u16()?)?
        })
    }
}

impl Borrow<FlattiverseMessageData> for ChatMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        &self.data
    }
}
impl BorrowMut<FlattiverseMessageData> for ChatMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<ChatMessageData> + FlattiverseMessage> ChatMessage for T {
    fn from(&self) -> &Weak<RwLock<Player>> {
        &self.borrow().from
    }
}


impl fmt::Display for ChatMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}