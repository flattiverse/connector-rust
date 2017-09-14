
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use TimeSpan;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(UniverseGroupResetPendingMessage);
pub trait UniverseGroupResetPendingMessage : GameMessage {

    fn universe_group(&self) -> &Arc<UniverseGroup>;

    fn remaining_time(&self) -> &TimeSpan;
}

pub struct UniverseGroupResetPendingMessageData {
    data:   GameMessageData,
    group:  Arc<UniverseGroup>,
    time:   TimeSpan,
}

impl UniverseGroupResetPendingMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<UniverseGroupResetPendingMessageData, Error> {
        Ok(UniverseGroupResetPendingMessageData {
            data:  GameMessageData::from_packet(connector, packet, reader)?,
            group: connector.universe_group(reader.read_u16()?)?,
            time:  TimeSpan::from_reader(reader)?,
        })
    }
}

impl Borrow<GameMessageData> for UniverseGroupResetPendingMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for UniverseGroupResetPendingMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for UniverseGroupResetPendingMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for UniverseGroupResetPendingMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<UniverseGroupResetPendingMessageData> + BorrowMut<UniverseGroupResetPendingMessageData> + GameMessage> UniverseGroupResetPendingMessage for T {
    fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.borrow().group
    }

    fn remaining_time(&self) -> &TimeSpan {
        &self.borrow().time
    }
}

impl fmt::Display for UniverseGroupResetPendingMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] UniverseGroup {} pending reset in {} seconds.",
            (self as &FlattiverseMessage).timestamp(),
            self.group.name(),
            self.time.seconds()+1,
        )
    }
}