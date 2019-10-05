use crate::entity::Universe;
use crate::packet::Packet;
use backtrace::Backtrace;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
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

    pub fn update(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        match packet.command {
            crate::entity::command_id::S2C_UNIVERSE_META_INFO_UPDATED => {
                let universe = Universe::try_from(packet)?;
                info!("Received universe {:#?}", universe);
                if self.universes.len() < usize::from(packet.base_address) {
                    let diff = usize::from(packet.base_address) - self.universes.len();
                    self.universes.reserve(diff);
                    (0..diff).for_each(|_| self.universes.push(None));
                }
                self.universes[usize::from(packet.base_address)] = Some(universe);
            }
            crate::entity::command_id::S2C_LOGIN_COMPLETED => {
                info!("Login completed");
            }
            command => warn!("Unknown command: {}", command),
        }
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
            UpdateError::IoError(bt, e) => write!(f, "Internal IoError: {}, {:?}", e, bt),
        }
    }
}

impl From<IoError> for UpdateError {
    fn from(e: IoError) -> Self {
        UpdateError::IoError(Backtrace::new(), e)
    }
}
