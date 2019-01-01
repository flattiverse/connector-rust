
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub trait GameMessage : Message {

}

pub(crate) struct GameMessageData {
    data: MessageData
}


impl GameMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<GameMessageData, Error> {
        Ok(GameMessageData {
            data: MessageData::from_packet(connector, packet, reader)?
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for GameMessageData {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

impl GameMessage for GameMessageData {

}


impl fmt::Display for GameMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}