
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

downcast!(TargetDominationFinishedMessage);
pub trait TargetDominationFinishedMessage : GameMessage {
    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>>;
}

pub struct TargetDominationFinishedMessageData {
    data:   GameMessageData,
    name:   String,
    team:   Option<Arc<RwLock<Team>>>,
}

impl TargetDominationFinishedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TargetDominationFinishedMessageData, Error> {
        Ok(TargetDominationFinishedMessageData {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            name:   reader.read_string()?,
            team:   {
                let id = reader.read_unsigned_byte()?;
                if id != 255  {
                    let player = connector.player().upgrade();
                    let player = player.ok_or(Error::PlayerNotAvailable)?;
                    let player = player.read()?;
                    let group  = player.universe_group().upgrade();
                    let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
                    Some(group.team(id)?)
                } else {
                    None
                }
            },
        })
    }
}

impl Borrow<GameMessageData> for TargetDominationFinishedMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for TargetDominationFinishedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for TargetDominationFinishedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for TargetDominationFinishedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<TargetDominationFinishedMessageData> + BorrowMut<TargetDominationFinishedMessageData> + GameMessage> TargetDominationFinishedMessage for T {

    fn mission_target_name(&self) -> &str {
        &self.borrow().name
    }

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>> {
        &self.borrow().team
    }
}

impl fmt::Display for TargetDominationFinishedMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", (self as &FlattiverseMessage).timestamp())?;

        if let Some(ref team) = self.team {
            if let Ok(ref team) = team.read() {
                write!(f, "Team \"{}\" ", team.name())?;
            } else {
                write!(f, "<defect Team> ")?;
            }
        } else {
            write!(f, "Unknown Team ")?;
        }
        write!(f, "finished the domination of MissionTarget \"{}\". 350 tick counter is running now.", self.name)
    }
}