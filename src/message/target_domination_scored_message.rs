
use std::fmt;
use std::sync::Arc;

use crate::Team;
use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub struct TargetDominationScoredMessage {
    data:   GameMessageData,
    name:   String,
    team:   Option<Arc<Team>>,
}

impl TargetDominationScoredMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TargetDominationScoredMessage, Error> {
        Ok(TargetDominationScoredMessage {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            name:   reader.read_string()?,
            team:   {
                let id = reader.read_unsigned_byte()?;
                if id != 255  {
                    let player = connector.player().upgrade();
                    let player = player.ok_or(Error::PlayerNotAvailable)?;
                    let group  = player.universe_group().upgrade();
                    let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
                    Some(group.team(id)?)
                } else {
                    None
                }
            },
        })
    }

    pub fn mission_target_name(&self) -> &str {
        &self.name
    }

    pub fn mission_target_team(&self) -> &Option<Arc<Team>> {
        &self.team
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for TargetDominationScoredMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for TargetDominationScoredMessage {

}

impl fmt::Display for TargetDominationScoredMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", self.timestamp())?;

        if let Some(ref team) = self.team {
            write!(f, "Team \"{}\" ", team.name())?;
        } else {
            write!(f, "Unknown Team ")?;
        }
        write!(f, "scored on MissionTarget \"{}\".", self.name)
    }
}