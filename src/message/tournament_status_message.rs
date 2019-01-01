
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Connector;
use crate::UniverseGroup;
use crate::UniversalEnumerable;
use crate::TournamentStage;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub struct TournamentStatusMessage {
    data:   GameMessageData,
    group:  Arc<UniverseGroup>,
    stage:  TournamentStage,
}

impl TournamentStatusMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TournamentStatusMessage, Error> {
        Ok(TournamentStatusMessage {
            data:  GameMessageData::from_packet(connector, packet, reader)?,
            group: connector.universe_group(reader.read_u16()?)?,
            stage: TournamentStage::from_id(reader.read_byte()?)?,
        })
    }

    pub fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.group
    }

    pub fn tournament_stage(&self) -> &TournamentStage {
        &self.stage
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for TournamentStatusMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for TournamentStatusMessage {

}

impl fmt::Display for TournamentStatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] A tournament on {} ",
            self.timestamp(),
            self.group.name(),
        )?;

        match self.stage {
            TournamentStage::Preparation => write!(f, "has been setup for preparation stage."),
            TournamentStage::Commencing => write!(f, "has been prepared for start and tournament start is commencing."),
            TournamentStage::Running => write!(f, " has been started."),
            TournamentStage::Ended => write!(f, " has been finished."),
        }
    }
}