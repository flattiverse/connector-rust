
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use UniversalEnumerable;

use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessageData;
use message::PlayerUnitDeceasedMessage;
use message::PlayerUnitDeceasedMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

impl_downcast!(PlayerUnitCollidedWithPlayerUnitMessage);
pub trait PlayerUnitCollidedWithPlayerUnitMessage : PlayerUnitDeceasedMessage {

    fn collider_unit_player(&self) -> &Arc<RwLock<Player>>;

    fn collider_unit_info(&self) -> &Arc<RwLock<ControllableInfo>>;
}

pub struct PlayerUnitCollidedWithPlayerUnitMessageData {
    data:   PlayerUnitDeceasedMessageData,
    player: Arc<RwLock<Player>>,
    info:   Arc<RwLock<ControllableInfo>>,
}

impl PlayerUnitCollidedWithPlayerUnitMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitCollidedWithPlayerUnitMessageData, Error> {
        Ok(PlayerUnitCollidedWithPlayerUnitMessageData {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            player: connector.player_for(reader.read_u16()?)?,
            info:   {
                let player = connector.player_for(reader.read_u16()?)?;
                let player = player.read()?;
                player.controllable_info(reader.read_unsigned_byte()?).ok_or(Error::ControllableInfoNotAvailable)?
            }
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitCollidedWithPlayerUnitMessageData> + BorrowMut<PlayerUnitCollidedWithPlayerUnitMessageData> + PlayerUnitDeceasedMessage> PlayerUnitCollidedWithPlayerUnitMessage for T {
    fn collider_unit_player(&self) -> &Arc<RwLock<Player>> {
        &self.borrow().player
    }

    fn collider_unit_info(&self) -> &Arc<RwLock<ControllableInfo>> {
        &self.borrow().info
    }
}

impl fmt::Display for PlayerUnitCollidedWithPlayerUnitMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' has a deadly collision with {:?} from '{}'.",
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
            match self.info.read() {
                Err(_) => String::new(),
                Ok(ref read) => {
                    let mut string = String::new();
                    write!(string, "{:?}", read.kind())?;
                    string
                },
            },
            match self.player.read() {
                Err(_) => "",
                Ok(ref player) => player.name()
            }
        )
    }
}