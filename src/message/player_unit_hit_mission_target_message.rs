
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

use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

impl_downcast!(PlayerUnitContinuedMessage);
pub trait PlayerUnitContinuedMessage : GameMessage {

    fn player_unit_player(&self) -> &Arc<RwLock<Player>>;

    fn player_unit(&self) -> &Arc<RwLock<ControllableInfo>>;
}

pub struct PlayerUnitContinuedMessageData {
    data:   GameMessageData,
    player: Arc<RwLock<Player>>,
    info:   Arc<RwLock<ControllableInfo>>,
}

impl PlayerUnitContinuedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitContinuedMessageData, Error> {
        Ok(PlayerUnitContinuedMessageData {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            player: connector.player_for(reader.read_u16()?)?,
            info:   {
                let player = connector.player_for(reader.read_u16()?)?;
                let index = reader.read_unsigned_byte()?;
                let player = player.read()?;
                player.controllable_info(index).ok_or(Error::InvalidControllableInfo(index))?
            }
        })
    }
}

impl Borrow<GameMessageData> for PlayerUnitContinuedMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitContinuedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitContinuedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitContinuedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitContinuedMessageData> + BorrowMut<PlayerUnitContinuedMessageData> + GameMessage> PlayerUnitContinuedMessage for T {
    fn player_unit_player(&self) -> &Arc<RwLock<Player>> {
        &self.borrow().player
    }

    fn player_unit(&self) -> &Arc<RwLock<ControllableInfo>> {
        &self.borrow().info
    }
}

impl fmt::Display for PlayerUnitContinuedMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' continued game.",

            (self as &FlattiverseMessage).timestamp(),
            match (self as &PlayerUnitContinuedMessage).player_unit().read() {
                Err(_) => String::new(),
                Ok(ref read) => {
                    let mut string = String::new();
                    write!(string, "{:?}", read.kind())?;
                    string
                },
            },
            match (self as &PlayerUnitContinuedMessage).player_unit().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
            match (self as &PlayerUnitContinuedMessage).player_unit_player().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
        )
    }
}