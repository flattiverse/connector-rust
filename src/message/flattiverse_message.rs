
use std::fmt;
use std::sync::Arc;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_flattiverse_message::prelude::*;

pub trait Message: fmt::Display + Send + Sync {
    fn timestamp(&self) -> &DateTime;
}

pub(crate) struct MessageData {
    timestamp: DateTime,
}

impl MessageData {
    pub fn from_packet(_: &Arc<Connector>, _: &Packet, reader: &mut BinaryReader) -> Result<MessageData, Error> {
        Ok(MessageData {
            timestamp: DateTime::from_ticks(reader.read_i64()?),
        })
    }
}

impl Message for MessageData {
    fn timestamp(&self) -> &DateTime {
        &self.timestamp
    }
}

impl fmt::Display for MessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
