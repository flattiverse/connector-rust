
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use Error;
use Connector;
use UniverseGroup;
use net::Packet;
use net::BinaryReader;

use ManagedArray;
use TournamentTeam;

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

struct TournamentMut {
    stage:          TournamentStage,
    set:            TournamentSet,
    test_mode:      bool,
    loaded:         bool,
}

// TODO incomplete implementation
pub struct Tournament {
    connector:      Weak<Connector>,
    universe_group: Weak<UniverseGroup>,
    teams:          ManagedArray<Arc<RwLock<TournamentTeam>>>,
    mutable:        RwLock<TournamentMut>,
}

impl Tournament {
    pub fn from_reader(connector: Weak<Connector>, universe_group: &Arc<UniverseGroup>, _: &Packet, reader: &mut BinaryReader) -> Result<Tournament, Error> {
        Ok(Tournament {
            connector,
            universe_group: Arc::downgrade(universe_group),
            mutable: RwLock::new(TournamentMut {
                loaded:     false,
                test_mode:  reader.read_byte()? == 0x01,
                stage:      TournamentStage ::from_id(reader.read_byte()?)?,
                set:        TournamentSet   ::from_id(reader.read_byte()?)?,
            }),
            teams: {
                let group = universe_group;
                let len = group.teams().len();
                let teams: ManagedArray<Arc<RwLock<TournamentTeam>>> = ManagedArray::with_capacity(len);

                for i in 0..len {
                    match teams.get(i) {
                        &None => break,
                        &Some(ref team) => {
                            team.write()?.set_wins(reader.read_unsigned_byte()?);
                        }
                    }
                }

                // TODO entries missing

                teams
            },
        })
    }

    pub fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    pub fn universe_group(&self) -> &Weak<UniverseGroup> {
        &self.universe_group
    }

    pub fn stage(&self) -> TournamentStage {
        self.mutable.read().unwrap().stage
    }

    pub fn set(&self) -> TournamentSet {
        self.mutable.read().unwrap().set
    }

    pub fn test_mode(&self) -> bool {
        self.mutable.read().unwrap().test_mode
    }

    pub fn loaded(&self) -> bool {
        self.mutable.read().unwrap().loaded
    }

    pub fn update(&self, packet: &Packet) -> Result<(), Error> {
        let mut mutable = self.mutable.write()?;
        if packet.read().len() == 0 {
            mutable.stage = TournamentStage::Ended;
            return Ok(());
        }

        let reader = &mut packet.read() as &mut BinaryReader;
        mutable.test_mode  = reader.read_byte()? == 0x01;
        mutable.stage      = TournamentStage::from_id(reader.read_byte()?)?;
        mutable.set        = TournamentSet  ::from_id(reader.read_byte()?)?;

        for i in 0..self.teams.len() {
            if let &Some(ref team) = self.teams.get(i) {
                team.write()?.set_wins(reader.read_byte()?);
            }
        }

        Ok(())
    }
}