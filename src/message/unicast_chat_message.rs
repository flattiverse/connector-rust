
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use net::Packet;
use net::BinaryReader;

use message::ChatMessage;
use message::ChatMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(UnicastChatMessage);
pub trait UnicastChatMessage : ChatMessage {

    fn to(&self) -> &Arc<Player>;

    fn message(&self) -> &str;
}

pub struct UnicastChatMessageData {
    data:   ChatMessageData,
    to:     Arc<Player>,
    message:String,
}

impl UnicastChatMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<UnicastChatMessageData, Error> {
        Ok(UnicastChatMessageData {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.player_for(reader.read_u16()?)?,
            message:reader.read_string()?,
        })
    }
}

impl Borrow<ChatMessageData> for UnicastChatMessageData {
    fn borrow(&self) -> &ChatMessageData {
        &self.data
    }
}
impl BorrowMut<ChatMessageData> for UnicastChatMessageData {
    fn borrow_mut(&mut self) -> &mut ChatMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for UnicastChatMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &ChatMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for UnicastChatMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut ChatMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<UnicastChatMessageData> + BorrowMut<UnicastChatMessageData> + ChatMessage> UnicastChatMessage for T {
    fn to(&self) -> &Arc<Player> {
        &self.borrow().to
    }

    fn message(&self) -> &str {
        &self.borrow().message
    }
}

impl fmt::Display for UnicastChatMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] <{}> {}",
               (self as &FlattiverseMessage).timestamp(),
               (self as &ChatMessage).from().name(),
               self.message
        )
    }
}