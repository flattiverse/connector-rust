
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

downcast!(PlayerUnitShotByPlayerUnitMessage);
pub trait PlayerUnitShotByPlayerUnitMessage : PlayerUnitDeceasedMessage {

    fn collider_unit_player(&self) -> &Arc<Player>;

    fn collider_unit_info(&self) -> &Arc<ControllableInfo>;
}

pub struct PlayerUnitShotByPlayerUnitMessageData {
    data:   PlayerUnitDeceasedMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
}

impl PlayerUnitShotByPlayerUnitMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitShotByPlayerUnitMessageData, Error> {
        Ok(PlayerUnitShotByPlayerUnitMessageData {
            data:   PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            player: connector.player_for(reader.read_u16()?)?,
            info:   {
                let player = connector.player_for(reader.read_u16()?)?;
                player.controllable_info(reader.read_unsigned_byte()?).ok_or(Error::ControllableInfoNotAvailable)?
            }
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitShotByPlayerUnitMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitShotByPlayerUnitMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitShotByPlayerUnitMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitShotByPlayerUnitMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitShotByPlayerUnitMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitShotByPlayerUnitMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitShotByPlayerUnitMessageData> + BorrowMut<PlayerUnitShotByPlayerUnitMessageData> + PlayerUnitDeceasedMessage> PlayerUnitShotByPlayerUnitMessage for T {
    fn collider_unit_player(&self) -> &Arc<Player> {
        &self.borrow().player
    }

    fn collider_unit_info(&self) -> &Arc<ControllableInfo> {
        &self.borrow().info
    }
}

impl fmt::Display for PlayerUnitShotByPlayerUnitMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' has been killed by {:?} from '{}'.",
            (self as &FlattiverseMessage).timestamp(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().kind(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().name(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit_player().name(),
            self.info.kind(),
            self.player.name(),
        )
    }
}