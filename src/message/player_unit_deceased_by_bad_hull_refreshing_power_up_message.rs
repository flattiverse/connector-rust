
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use message::GameMessageData;
use message::PlayerUnitDeceasedMessage;
use message::PlayerUnitDeceasedMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage);
pub trait PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage : PlayerUnitDeceasedMessage {

    fn hull_refreshing_power_up(&self) -> &str;
}

pub struct PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    data:       PlayerUnitDeceasedMessageData,
    power_up:   String,
}

impl PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData, Error> {
        Ok(PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
            data:       PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            power_up:   reader.read_string()?,
        })
    }
}



impl Borrow<GameMessageData> for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data.borrow()
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        self.data.borrow_mut()
    }
}
impl Borrow<PlayerUnitDeceasedMessageData> for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn borrow(&self) -> &PlayerUnitDeceasedMessageData {
        &self.data
    }
}
impl BorrowMut<PlayerUnitDeceasedMessageData> for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitDeceasedMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &PlayerUnitDeceasedMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut PlayerUnitDeceasedMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData> + BorrowMut<PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData> + PlayerUnitDeceasedMessage> PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage for T {
    fn hull_refreshing_power_up(&self) -> &str {
        &self.borrow().power_up
    }
}

impl fmt::Display for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' deceased by HullRefreshingPowerUp {} containing negative amount of hullpoints.",
            (self as &FlattiverseMessage).timestamp(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().kind(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit().name(),
            (self as &PlayerUnitDeceasedMessage).deceased_player_unit_player().name(),
            self.power_up
        )
    }
}