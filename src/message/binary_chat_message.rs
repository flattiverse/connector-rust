
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

downcast!(BinaryChatMessage);
pub trait BinaryChatMessage : ChatMessage {

    fn to(&self) -> &Arc<Player>;

    fn message(&self) -> &Vec<u8>;
}

pub struct BinaryChatMessageData {
    data:   ChatMessageData,
    to:     Arc<Player>,
    message:Vec<u8>,
}

impl BinaryChatMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<BinaryChatMessageData, Error> {
        Ok(BinaryChatMessageData {
            data:   ChatMessageData::from_packet(connector, packet, reader)?,
            to:     connector.player_for(reader.read_u16()?)?,
            message:{
                let count = reader.read_u16()? as usize;
                reader.read_bytes(count)?
            },
        })
    }
}

impl Borrow<ChatMessageData> for BinaryChatMessageData {
    fn borrow(&self) -> &ChatMessageData {
        &self.data
    }
}
impl BorrowMut<ChatMessageData> for BinaryChatMessageData {
    fn borrow_mut(&mut self) -> &mut ChatMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for BinaryChatMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &ChatMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for BinaryChatMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut ChatMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<BinaryChatMessageData> + BorrowMut<BinaryChatMessageData> + ChatMessage> BinaryChatMessage for T {
    fn to(&self) -> &Arc<Player> {
        &self.borrow().to
    }

    fn message(&self) -> &Vec<u8> {
        &self.borrow().message
    }
}

impl fmt::Display for BinaryChatMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] -{}- 0x",
               (self as &FlattiverseMessage).timestamp(),
               (self as &ChatMessage).from().name(),
        )?;
        for byte in self.message.iter() {
            write!(f, "{:x}", byte)?;
        }
        Ok(())
    }
}