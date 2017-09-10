
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use unit::UnitKind;
use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessageData;
use message::PlayerUnitDeceasedMessage;
use message::PlayerUnitDeceasedMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

impl_downcast!(PlayerUnitShotByUnitMessage);
pub trait PlayerUnitShotByUnitMessage : PlayerUnitDeceasedMessage {

    fn collider_unit_kind(&self) -> UnitKind;

    fn collider_unit_name(&self) -> &str;
}

pub struct PlayerUnitShotByUnitMessageData {
    data:   PlayerUnitDeceasedMessageData,
    kind:   UnitKind,
    name:   String,
}

impl PlayerUnitShotByUnitMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitShotByUnitMessageData, Error> {
        Ok(PlayerUnitShotByUnitMessageData {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            kind:   UnitKind::from_id(reader.read_byte()?),
            name:   reader.read_string()?,
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitShotByUnitMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitShotByUnitMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitShotByUnitMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitShotByUnitMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitShotByUnitMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitShotByUnitMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitShotByUnitMessageData> + BorrowMut<PlayerUnitShotByUnitMessageData> + PlayerUnitDeceasedMessage> PlayerUnitShotByUnitMessage for T {
    fn collider_unit_kind(&self) -> UnitKind {
        self.borrow().kind
    }

    fn collider_unit_name(&self) -> &str {
        &self.borrow().name
    }
}

impl fmt::Display for PlayerUnitShotByUnitMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' has been killed by {:?} '{}'.",
            (self as &FlattiverseMessage).timestamp(),
            match (self as &PlayerUnitDeceasedMessage).deceased_player_unit().read() {
                Err(_) => String::new(),
                Ok(ref read) => {
                    let mut string = String::new();
                    write!(string, "{:?}", read.kind())?;
                    string
                },
            },
            match (self as &PlayerUnitDeceasedMessage).deceased_player_unit().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
            match (self as &PlayerUnitDeceasedMessage).deceased_player_unit_player().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
            self.kind,
            self.name
        )
    }
}