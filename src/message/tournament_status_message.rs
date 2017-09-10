
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Team;
use Error;
use Connector;
use UniverseGroup;
use UniversalEnumerable;
use TournamentStage;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

impl_downcast!(TournamentStatusMessage);
pub trait TournamentStatusMessage : GameMessage {

    fn universe_group(&self) -> &Arc<RwLock<UniverseGroup>>;

    fn tournament_stage(&self) -> &TournamentStage;
}

pub struct TournamentStatusMessageData {
    data:   GameMessageData,
    group:  Arc<RwLock<UniverseGroup>>,
    stage:  TournamentStage,
}

impl TournamentStatusMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<TournamentStatusMessageData, Error> {
        Ok(TournamentStatusMessageData {
            data:  GameMessageData::from_packet(connector, packet, reader)?,
            group: connector.universe_group(reader.read_u16()?)?,
            stage: TournamentStage::from_id(reader.read_byte()?)?,
        })
    }
}

impl Borrow<GameMessageData> for TournamentStatusMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for TournamentStatusMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for TournamentStatusMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for TournamentStatusMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<TournamentStatusMessageData> + BorrowMut<TournamentStatusMessageData> + GameMessage> TournamentStatusMessage for T {
    fn universe_group(&self) -> &Arc<RwLock<UniverseGroup>> {
        &self.borrow().group
    }

    fn tournament_stage(&self) -> &TournamentStage {
        &self.borrow().stage
    }
}

impl fmt::Display for TournamentStatusMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let group = self.group.read().unwrap();
        write!(f, "[{}] A tournament on {} ",
            (self as &FlattiverseMessage).timestamp(),
            group.name(),
        )?;

        match self.stage {
            TournamentStage::Preparation => write!(f, "has been setup for preparation stage."),
            TournamentStage::Commencing => write!(f, "has been prepared for start and tournament start is commencing."),
            TournamentStage::Running => write!(f, " has been started."),
            TournamentStage::Ended => write!(f, " has been finished."),
        }
    }
}