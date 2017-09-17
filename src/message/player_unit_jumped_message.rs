
use std::fmt;
use std::sync::Arc;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::AnyControllable;

use message::any_game_message::prelude::*;

pub struct PlayerUnitJumpedMessage {
    data:   GameMessageData,
    info:   AnyControllable,
    inter:  bool,
}

impl PlayerUnitJumpedMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitJumpedMessage, Error> {
        Ok(PlayerUnitJumpedMessage {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            inter:  reader.read_bool()?,
            info:   {
                let index = reader.read_unsigned_byte()?;
                connector.controllable(index)?.clone()
            }
        })
    }

    pub fn controllable(&self) -> &AnyControllable {
        &self.info
    }

    pub fn is_inter_universe(&self) -> bool {
        self.inter
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitJumpedMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitJumpedMessage {

}

impl fmt::Display for PlayerUnitJumpedMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} {}",
               self.timestamp(),
               match self.info {
                   AnyControllable::Platform(_) => "Platform",
                   AnyControllable::Probe   (_) => "Probe",
                   AnyControllable::Drone   (_) => "Drone",
                   AnyControllable::Base    (_) => "Base",
                   AnyControllable::Ship    (_) => "Ship",
               },
               self.info.name(),
        )?;
        if self.inter {
            write!(f, " accomplished a inter-universe jump.")
        } else {
            write!(f, " accomplished a jump.")
        }
    }
}