
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniversalEnumerable;

use unit::UnitKind;

use net::Packet;
use net::BinaryReader;

use message::GameMessageData;
use message::PlayerUnitDeceasedMessage;
use message::PlayerUnitDeceasedMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerUnitCollidedWithUnitMessage);
pub trait PlayerUnitCollidedWithUnitMessage : PlayerUnitDeceasedMessage {

    fn collider_unit_kind(&self) -> UnitKind;

    fn collider_unit_name(&self) -> &str;
}

pub struct PlayerUnitCollidedWithUnitMessageData {
    data:   PlayerUnitDeceasedMessageData,
    kind:   UnitKind,
    name:   String,
}

impl PlayerUnitCollidedWithUnitMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitCollidedWithUnitMessageData, Error> {
        Ok(PlayerUnitCollidedWithUnitMessageData {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            kind:   UnitKind::from_id(reader.read_byte()?),
            name:   reader.read_string()?,
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitCollidedWithUnitMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitCollidedWithUnitMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitCollidedWithUnitMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitCollidedWithUnitMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitCollidedWithUnitMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitCollidedWithUnitMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitCollidedWithUnitMessageData> + BorrowMut<PlayerUnitCollidedWithUnitMessageData> + PlayerUnitDeceasedMessage> PlayerUnitCollidedWithUnitMessage for T {
    fn collider_unit_kind(&self) -> UnitKind {
        self.borrow().kind
    }

    fn collider_unit_name(&self) -> &str {
        &self.borrow().name
    }
}

impl fmt::Display for PlayerUnitCollidedWithUnitMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' collided with {:?} '{}'.",
            (self as &FlattiverseMessage).timestamp(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().kind(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().name(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit_player().name(),
            self.kind,
            self.name
        )
    }
}