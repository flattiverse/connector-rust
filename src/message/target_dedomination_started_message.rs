
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Team;
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

impl_downcast!(TargetDedominationStartedMessage);
pub trait TargetDedominationStartedMessage : GameMessage {
    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>>;

    fn mission_target_team_old(&self) -> &Option<Arc<RwLock<Team>>>;
}

pub struct TargetDedominationStartedMessageData {
    data:       GameMessageData,
    name:       String,
    team:       Option<Arc<RwLock<Team>>>,
    team_old:   Option<Arc<RwLock<Team>>>,
}

impl TargetDedominationStartedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TargetDedominationStartedMessageData, Error> {
        Ok(TargetDedominationStartedMessageData {
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
                    let group  = group.read()?;
                    Some(group.team(id).clone().ok_or(Error::TeamNotAvailable)?)
                } else {
                    None
                }
            },
            team_old:   {
                let id = reader.read_unsigned_byte()?;
                if id != 255  {
                    let player = connector.player().upgrade();
                    let player = player.ok_or(Error::PlayerNotAvailable)?;
                    let player = player.read()?;
                    let group  = player.universe_group().upgrade();
                    let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
                    let group  = group.read()?;
                    Some(group.team(id).clone().ok_or(Error::TeamNotAvailable)?)
                } else {
                    None
                }
            },
        })
    }
}

impl Borrow<GameMessageData> for TargetDedominationStartedMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for TargetDedominationStartedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for TargetDedominationStartedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for TargetDedominationStartedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<TargetDedominationStartedMessageData> + BorrowMut<TargetDedominationStartedMessageData> + GameMessage> TargetDedominationStartedMessage for T {

    fn mission_target_name(&self) -> &str {
        &self.borrow().name
    }

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>> {
        &self.borrow().team
    }

    fn mission_target_team_old(&self) -> &Option<Arc<RwLock<Team>>> {
        &self.borrow().team_old
    }
}

impl fmt::Display for TargetDedominationStartedMessageData {
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
        write!(f, "finished the domination of MissionTarget \"{}\" currently owned by", self.name)?;

        if let Some(ref team) = self.team {
            if let Ok(ref team) = team.read() {
                write!(f, "Team \"{}\" ", team.name())?;
            } else {
                write!(f, "<defect Team> ")?;
            }
        } else {
            write!(f, "an unknown Team ")?;
        }
        write!(f, ".")
    }
}