
use std::sync::Arc;
use std::sync::Weak;

use std::borrow::Borrow;


use crate::Team;
use crate::Error;
use crate::Connector;
use crate::UniverseGroup;

use crate::net::Packet;
use crate::net::BinaryReader;

pub struct TournamentTeam {
    team: Team,
    wins: u8,
}

impl TournamentTeam {
    pub fn from_reader(connector: Weak<Connector>, universe_group: &Arc<UniverseGroup>, packet: &Packet, reader: &mut BinaryReader) -> Result<TournamentTeam, Error> {
        Ok(TournamentTeam {
            team: Team::from_reader(connector, universe_group, packet, reader)?,
            wins: 0u8,
        })
    }

    pub fn wins(&self) -> u8 {
        self.wins
    }

    pub(crate) fn set_wins(&mut self, wins: u8) {
        self.wins = wins;
    }
}



impl Borrow<Team> for TournamentTeam {
    fn borrow(&self) -> &Team {
        &self.team
    }
}