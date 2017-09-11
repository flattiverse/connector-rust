
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;

use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessageData;

downcast!(PlayerUnitDeceasedMessage);
pub trait PlayerUnitDeceasedMessage : GameMessage {

    fn deceased_player_unit_player(&self) -> &Arc<RwLock<Player>>;

    fn deceased_player_unit(&self) -> &Arc<RwLock<ControllableInfo>>;
}

pub struct PlayerUnitDeceasedMessageData {
    data:   GameMessageData,
    player: Arc<RwLock<Player>>,
    info:   Arc<RwLock<ControllableInfo>>,
}

impl PlayerUnitDeceasedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitDeceasedMessageData, Error> {
        Ok(PlayerUnitDeceasedMessageData {
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

impl Borrow<GameMessageData> for PlayerUnitDeceasedMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitDeceasedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitDeceasedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitDeceasedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitDeceasedMessageData> + BorrowMut<PlayerUnitDeceasedMessageData> + GameMessage> PlayerUnitDeceasedMessage for T {
    fn deceased_player_unit_player(&self) -> &Arc<RwLock<Player>> {
        &self.borrow().player
    }

    fn deceased_player_unit(&self) -> &Arc<RwLock<ControllableInfo>> {
        &self.borrow().info
    }
}

impl fmt::Display for PlayerUnitDeceasedMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}