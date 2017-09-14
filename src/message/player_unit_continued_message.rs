
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use UniversalEnumerable;

use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerUnitContinuedMessage);
pub trait PlayerUnitContinuedMessage : GameMessage {

    fn player_unit_player(&self) -> &Arc<Player>;

    fn player_unit(&self) -> &Arc<ControllableInfo>;
}

pub struct PlayerUnitContinuedMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
}

impl PlayerUnitContinuedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitContinuedMessageData, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitContinuedMessageData {
            data,
            player: player.clone(),
            info:   {
                let index = reader.read_unsigned_byte()?;
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
    fn player_unit_player(&self) -> &Arc<Player> {
        &self.borrow().player
    }

    fn player_unit(&self) -> &Arc<ControllableInfo> {
        &self.borrow().info
    }
}

impl fmt::Display for PlayerUnitContinuedMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' continued game.",
            (self as &FlattiverseMessage).timestamp(),
            (self as &PlayerUnitContinuedMessage).player_unit().kind(),
            (self as &PlayerUnitContinuedMessage).player_unit().name(),
            (self as &PlayerUnitContinuedMessage).player_unit_player().name(),
        )
    }
}