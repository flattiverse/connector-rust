
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Team;
use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(MissionTargetAvailableMessage);
pub trait MissionTargetAvailableMessage : GameMessage {
    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Arc<RwLock<Team>>;
}

pub struct MissionTargetAvailableMessageData {
    data:   GameMessageData,
    name:   String,
    team:   Arc<RwLock<Team>>,
}

impl MissionTargetAvailableMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<MissionTargetAvailableMessageData, Error> {
        Ok(MissionTargetAvailableMessageData {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            team:   {
                let id = reader.read_unsigned_byte()?;
                let player = connector.player().upgrade();
                let player = player.ok_or(Error::PlayerNotAvailable)?;
                let player = player.read()?;
                let group  = player.universe_group().upgrade();
                let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
                let group  = group.read()?;
                group.team(id).clone().ok_or(Error::TeamNotAvailable)?
            },
            name:   reader.read_string()?,
        })
    }
}

impl Borrow<GameMessageData> for MissionTargetAvailableMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for MissionTargetAvailableMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for MissionTargetAvailableMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for MissionTargetAvailableMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<MissionTargetAvailableMessageData> + BorrowMut<MissionTargetAvailableMessageData> + GameMessage> MissionTargetAvailableMessage for T {

    fn mission_target_name(&self) -> &str {
        &self.borrow().name
    }

    fn mission_target_team(&self) -> &Arc<RwLock<Team>> {
        &self.borrow().team
    }
}

impl fmt::Display for MissionTargetAvailableMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] MissionTarget \"{}\" of Team {} is available again.",
            (self as &FlattiverseMessage).timestamp(),
            self.name,
            match self.team.read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
        )
    }
}