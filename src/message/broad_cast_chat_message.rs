
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use message::ChatMessage;
use message::ChatMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

pub trait BroadCastChatMessage: ChatMessage {

    fn to(&self) -> &Arc<RwLock<UniverseGroup>>;

    fn message(&self) -> &str;
}

pub struct BroadCastChatMessageData {
    data:   ChatMessageData,
    to:     Arc<RwLock<UniverseGroup>>,
    message:String,
}

impl BroadCastChatMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<BroadCastChatMessageData, Error> {
        Ok(BroadCastChatMessageData {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.universe_group(reader.read_u16()?)?,
            message:reader.read_string()?,
        })
    }
}

impl Borrow<ChatMessageData> for BroadCastChatMessageData {
    fn borrow(&self) -> &ChatMessageData {
        &self.data
    }
}
impl BorrowMut<ChatMessageData> for BroadCastChatMessageData {
    fn borrow_mut(&mut self) -> &mut ChatMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for BroadCastChatMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &ChatMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for BroadCastChatMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut ChatMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<BroadCastChatMessageData> + BorrowMut<BroadCastChatMessageData> + ChatMessage> BroadCastChatMessage for T {
    fn to(&self) -> &Arc<RwLock<UniverseGroup>> {
        &self.borrow().to
    }

    fn message(&self) -> &str {
        &self.borrow().message
    }
}

impl fmt::Display for BroadCastChatMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] <U: {}> {}",
               (self as &FlattiverseMessage).timestamp(),
               match (self as &BroadCastChatMessage).to().read() {
                   Err(_) => "",
                   Ok(ref group) => group.name()
               },
               self.message
        )
    }
}