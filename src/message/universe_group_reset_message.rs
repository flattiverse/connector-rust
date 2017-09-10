
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Team;
use Error;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

impl_downcast!(UniverseGroupResetMessage);
pub trait UniverseGroupResetMessage : GameMessage {

    fn universe_group(&self) -> &Arc<RwLock<UniverseGroup>>;
}

pub struct UniverseGroupResetMessageData {
    data:   GameMessageData,
    group:  Arc<RwLock<UniverseGroup>>,
}

impl UniverseGroupResetMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<UniverseGroupResetMessageData, Error> {
        Ok(UniverseGroupResetMessageData {
            data:  GameMessageData::from_packet(connector, packet, reader)?,
            group: connector.universe_group(reader.read_u16()?)?,
        })
    }
}

impl Borrow<GameMessageData> for UniverseGroupResetMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for UniverseGroupResetMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for UniverseGroupResetMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for UniverseGroupResetMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<UniverseGroupResetMessageData> + BorrowMut<UniverseGroupResetMessageData> + GameMessage> UniverseGroupResetMessage for T {
    fn universe_group(&self) -> &Arc<RwLock<UniverseGroup>> {
        &self.borrow().group
    }
}

impl fmt::Display for UniverseGroupResetMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let group = self.group.read().unwrap();
        write!(f, "[{}] UniverseGroup {} has been reset.",
            (self as &FlattiverseMessage).timestamp(),
            group.name(),
        )
    }
}