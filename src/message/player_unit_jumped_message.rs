
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;

use controllable::Controllable;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerUnitJumpedMessage);
pub trait PlayerUnitJumpedMessage : GameMessage {

    fn controllable(&self) -> &Arc<RwLock<Controllable>>;

    fn inter_universe(&self) -> bool;
}

pub struct PlayerUnitJumpedMessageData {
    data:   GameMessageData,
    info:   Arc<RwLock<Controllable>>,
    inter:  bool,
}

impl PlayerUnitJumpedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitJumpedMessageData, Error> {
        Ok(PlayerUnitJumpedMessageData {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            inter:  reader.read_bool()?,
            info:   {
                let index = reader.read_unsigned_byte()?;
                connector.controllable(index).ok_or(Error::InvalidControllableInfo(index))?
            }
        })
    }
}

impl Borrow<GameMessageData> for PlayerUnitJumpedMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitJumpedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitJumpedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitJumpedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitJumpedMessageData> + BorrowMut<PlayerUnitJumpedMessageData> + GameMessage> PlayerUnitJumpedMessage for T {
    fn controllable(&self) -> &Arc<RwLock<Controllable>> {
        &self.borrow().info
    }

    fn inter_universe(&self) -> bool {
        self.borrow().inter
    }
}

impl fmt::Display for PlayerUnitJumpedMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.info.clone();
        let info = info.read().unwrap();
        write!(f, "[{}] {:?} {}",
               (self as &FlattiverseMessage).timestamp(),
               info.kind(),
               info.name(),
        )?;
        if self.inter {
            write!(f, " accomplished a inter-universe jump.")
        } else {
            write!(f, " accomplished a jump.")
        }
    }
}