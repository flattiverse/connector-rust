
use std::fmt;
use std::sync::Arc;

use Error;
use Player;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_chat_message::prelude::*;

pub trait ChatMessage : Message {
    fn from(&self) -> &Arc<Player>;
}

pub(crate) struct ChatMessageData {
    data: MessageData,
    from: Arc<Player>
}


impl ChatMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<ChatMessageData, Error> {
        Ok(ChatMessageData {
            data:   MessageData::from_packet(connector, packet, reader)?,
            from:   connector.player_for(reader.read_u16()?)?
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for ChatMessageData {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

impl ChatMessage for ChatMessageData {
    fn from(&self) -> &Arc<Player> {
        &self.from
    }
}


impl fmt::Display for ChatMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}