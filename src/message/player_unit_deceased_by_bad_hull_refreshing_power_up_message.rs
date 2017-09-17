
use std::fmt;
use std::sync::Arc;

use Error;
use Player;
use Connector;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use unit::ControllableInfo;

use message::any_player_unit_deceased_message::prelude::*;

pub struct PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {
    data:       PlayerUnitDeceasedMessageData,
    power_up:   String,
}

impl PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage, Error> {
        Ok(PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {
            data:       PlayerUnitDeceasedMessageData::from_packet(connector, packet, reader)?,
            power_up:   reader.read_string()?,
        })
    }

    pub fn hull_refreshing_power_up(&self) -> &str {
        &self.power_up
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {

}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PlayerUnitDeceasedMessage for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {
    fn deceased_player_unit_player(&self) -> &Arc<Player> {
        self.data.deceased_player_unit_player()
    }

    fn deceased_player_unit(&self) -> &Arc<ControllableInfo> {
        self.data.deceased_player_unit()
    }
}

impl fmt::Display for PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' deceased by HullRefreshingPowerUp {} containing negative amount of hullpoints.",
            self.timestamp(),
            self.deceased_player_unit().kind(),
            self.deceased_player_unit().name(),
            self.deceased_player_unit_player().name(),
            self.power_up
        )
    }
}