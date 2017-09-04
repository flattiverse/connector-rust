
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use Error;
use Player;
use DateTime;
use Connector;
use FlattiverseMessage;

use net::Packet;
use net::BinaryReader;

pub struct ChatMessageData {
    timestamp: DateTime,
    from: Arc<Player>
}

pub trait ChatMessage : FlattiverseMessage {
    fn from(&self) -> &Arc<Player>;
}

impl FlattiverseMessage for ChatMessageData {
    fn timestamp(&self) -> &DateTime {
        &self.timestamp
    }

    fn from_packet(connector: Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Self, Error> where Self: Sized {
        Ok(ChatMessageData {
            timestamp: DateTime::from_ticks(reader.read_i64()?),
            from:      connector.player(reader.read_u16()?).expect("Player data missing")
        })
    }
}

impl ChatMessage for ChatMessageData {
    fn from(&self) -> &Arc<Player> {
        &self.from
    }
}

impl fmt::Display for ChatMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}