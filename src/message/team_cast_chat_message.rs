
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

pub trait TeamCastChatMessage: ChatMessage {

    fn to(&self) -> &Arc<RwLock<Player>>;

    fn message(&self) -> &str;
}

pub struct TeamCastChatMessageData {
    data:   ChatMessageData,
    to:     Arc<RwLock<Player>>,
    message:String,
}

impl TeamCastChatMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TeamCastChatMessageData, Error> {
        Ok(TeamCastChatMessageData {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.player_for(reader.read_u16()?)?,
            message:reader.read_string()?,
        })
    }
}

impl Borrow<ChatMessageData> for TeamCastChatMessageData {
    fn borrow(&self) -> &ChatMessageData {
        &self.data
    }
}
impl BorrowMut<ChatMessageData> for TeamCastChatMessageData {
    fn borrow_mut(&mut self) -> &mut ChatMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for TeamCastChatMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &ChatMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for TeamCastChatMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut ChatMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<TeamCastChatMessageData> + BorrowMut<TeamCastChatMessageData> + ChatMessage> TeamCastChatMessage for T {
    fn to(&self) -> &Arc<RwLock<Player>> {
        &self.borrow().to
    }

    fn message(&self) -> &str {
        &self.borrow().message
    }
}

impl fmt::Display for TeamCastChatMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] <T: {}> {}",
               (self as &FlattiverseMessage).timestamp(),
               match (self as &ChatMessage).from().read() {
                   Err(_) => "",
                   Ok(ref player) => player.name()
               },
               self.message
        )
    }
}