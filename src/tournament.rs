
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;

use Team;
use Error;
use Connector;
use UniverseGroup;
use net::Packet;
use net::BinaryReader;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum TournamentStage {
    Preparation = 0,
    Commencing = 1,
    Running = 2,
    Ended = 3,
}

impl TournamentStage {
    pub fn from_id(id: u8) -> Result<TournamentStage, Error> {
        match id {
            0 => Ok(TournamentStage::Preparation),
            1 => Ok(TournamentStage::Commencing),
            2 => Ok(TournamentStage::Running),
            3 => Ok(TournamentStage::Ended),
            _ => Err(Error::InvalidTournamentStage(id))
        }
    }
}


#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum TournamentSet {
    Single = 0,
    BestOf3 = 1,
    BestOf5 = 2,
    BestOf7 = 3,
    BestOf9 = 4,
    DifferenceOf2 = 5,
    DifferenceOf3 = 6,
    DifferenceOf4 = 7,
    DifferenceOf5 = 8,
}

impl TournamentSet {
    pub fn from_id(id: u8) -> Result<TournamentSet, Error> {
        match id {
            0 => Ok(TournamentSet::Single),
            1 => Ok(TournamentSet::BestOf3),
            2 => Ok(TournamentSet::BestOf5),
            3 => Ok(TournamentSet::BestOf7),
            4 => Ok(TournamentSet::BestOf9),
            5 => Ok(TournamentSet::DifferenceOf2),
            6 => Ok(TournamentSet::DifferenceOf3),
            7 => Ok(TournamentSet::DifferenceOf4),
            8 => Ok(TournamentSet::DifferenceOf5),
            _ => Err(Error::InvalidTournamentSet(id))
        }
    }
}



pub struct TournamentTeam {
    pub(crate) team: Team,
    pub(crate) wins: u32,
    pub(crate) accounts: () // TODO missing
}

impl Borrow<Team> for TournamentTeam {
    fn borrow(&self) -> &Team {
        &self.team
    }
}

// TODO incomplete implementation
pub struct Tournament {
    connector:      Weak<Connector>,
    universe_group: Weak<RwLock<UniverseGroup>>,
    stage:          TournamentStage,
    set:            TournamentSet,
    // TODO missing teams
    test_mode:      bool,
    loaded:         bool,
}

impl Tournament {
    pub fn from_reader(connector: Weak<Connector>, universe_group: Weak<RwLock<UniverseGroup>>, packet: &Packet, reader: &mut BinaryReader) -> Result<Tournament, Error> {
        Ok(Tournament {
            connector,
            universe_group,
            test_mode:  reader.read_byte()? == 0x01,
            stage:      TournamentStage ::from_id(reader.read_byte()?)?,
            set:        TournamentSet   ::from_id(reader.read_byte()?)?,
            loaded:     false,
        })
    }

    pub fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    pub fn universe_group(&self) -> &Weak<RwLock<UniverseGroup>> {
        &self.universe_group
    }

    pub fn stage(&self) -> TournamentStage {
        self.stage
    }

    pub fn set(&self) -> TournamentSet {
        self.set
    }

    pub fn test_mode(&self) -> bool {
        self.test_mode
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }
}