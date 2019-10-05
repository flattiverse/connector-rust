use crate::entity::command_id;
use crate::entity::Universe;
use crate::packet::Packet;
use crate::players::Team;
use backtrace::Backtrace;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::io::Error as IoError;

const DEFAULT_UNIVERSES: usize = 16;

pub struct Server {
    universes: Vec<Option<Universe>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            universes: {
                let mut vec = Vec::with_capacity(DEFAULT_UNIVERSES);
                (0..DEFAULT_UNIVERSES).for_each(|_| vec.push(None));
                vec
            },
        }
    }

    pub(crate) fn update(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        match packet.command {
            command_id::S2C_LOGIN_COMPLETED => info!("Login completed"),
            command_id::S2C_UNIVERSE_META_INFO_UPDATED => self.update_universe(packet)?,
            command_id::S2C_UNIVERSE_TEAM_META_INFO_UPDATE => self.update_universe_team(packet)?,
            command => warn!("Unknown command: {}", command),
        }
        Ok(())
    }

    fn update_universe(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!(
            "Going to update universe for {}, delete={}",
            packet.base_address,
            packet.payload.is_none()
        );
        let universe = if packet.payload.is_some() {
            Some(Universe::try_from(packet)?)
        } else {
            None
        };
        debug!("Received universe {:#?}", universe);
        if self.universes.len() < usize::from(packet.base_address) {
            let diff = usize::from(packet.base_address) - self.universes.len();
            self.universes.reserve(diff);
            (0..diff).for_each(|_| self.universes.push(None));
        }
        self.universes[usize::from(packet.base_address)] = universe;
        Ok(())
    }

    fn update_universe_team(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        debug!(
            "Going to update team {} for universe {}, delete={}",
            packet.sub_address,
            packet.base_address,
            packet.payload.is_some()
        );
        let universe = self.universes[usize::from(packet.base_address)]
            .as_mut()
            .unwrap();
        let team = if packet.payload.is_some() {
            Some(Team::try_from(packet)?)
        } else {
            None
        };
        universe.teams[usize::from(packet.sub_address)] = team;
        Ok(())
    }
}

#[derive(Debug)]
pub enum UpdateError {
    IoError(Backtrace, IoError),
}

impl Error for UpdateError {}

impl Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            UpdateError::IoError(_bt, e) => write!(f, "Internal IoError: {}", e),
        }
    }
}

impl From<IoError> for UpdateError {
    fn from(e: IoError) -> Self {
        UpdateError::IoError(Backtrace::new(), e)
    }
}
