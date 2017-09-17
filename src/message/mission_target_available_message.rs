
use std::fmt;
use std::sync::Arc;

use Team;
use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use message::any_game_message::prelude::*;

pub struct MissionTargetAvailableMessage {
    data:   GameMessageData,
    name:   String,
    team:   Arc<Team>,
}

impl MissionTargetAvailableMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<MissionTargetAvailableMessage, Error> {
        Ok(MissionTargetAvailableMessage {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            team:   {
                let id = reader.read_unsigned_byte()?;
                let player = connector.player().upgrade();
                let player = player.ok_or(Error::PlayerNotAvailable)?;
                let group  = player.universe_group().upgrade();
                let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
                group.team(id)?
            },
            name:   reader.read_string()?,
        })
    }

    pub fn mission_target_name(&self) -> &str {
        &self.name
    }

    pub fn mission_target_team(&self) -> &Arc<Team> {
        &self.team
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for MissionTargetAvailableMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for MissionTargetAvailableMessage {

}

impl fmt::Display for MissionTargetAvailableMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] MissionTarget \"{}\" of Team {} is available again.",
            self.timestamp(),
            self.name,
            self.team.name(),
        )
    }
}