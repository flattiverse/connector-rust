use crate::entity::command_id;
use crate::entity::Universe;
use crate::packet::Packet;
use crate::players::Team;
#[macro_use]
use crate::macros::*;
use crate::com::RefuseReason;
use crate::num_traits::FromPrimitive;
use backtrace::Backtrace;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::io::Error as IoError;

const DEFAULT_UNIVERSES: usize = 16;

pub struct State {
    universes: Vec<Option<Universe>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            universes: vec_of_none!(DEFAULT_UNIVERSES),
        }
    }

    pub(crate) fn update(&mut self, packet: &Packet) -> Result<(), UpdateError> {
        match packet.command {
            command_id::S2C_LOGIN_COMPLETED => {
                if packet.helper != 0u8 {
                    return Err(UpdateError::LoginRefused(
                        RefuseReason::from_u8(packet.helper).expect("Failed to parse RefuseReason"),
                    ));
                }
                self.universes
                    .iter()
                    .flat_map(|u| u.as_ref())
                    .for_each(|u| info!("Universe: {:#?}", u));
            }
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
        let universe = map_payload_with_try_from!(packet, Universe);
        debug!("Received universe {:#?}", universe);
        expand_vec_of_none_if_necessary!(self.universes, usize::from(packet.base_address));
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
            .expect("Failed to update universe because unknown for given base_address");
        let team = map_payload_with_try_from!(packet, Team);
        expand_vec_of_none_if_necessary!(universe.teams, usize::from(packet.sub_address));
        universe.teams[usize::from(packet.sub_address)] = team;
        Ok(())
    }
}

#[derive(Debug)]
pub enum UpdateError {
    LoginRefused(RefuseReason),
    IoError(Backtrace, IoError),
}

impl Error for UpdateError {}

impl Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            UpdateError::LoginRefused(reason) => {
                write!(f, "Login refused with reason: {:?}", reason)
            }
            UpdateError::IoError(_bt, e) => write!(f, "Internal IoError: {}", e),
        }
    }
}

impl From<IoError> for UpdateError {
    fn from(e: IoError) -> Self {
        UpdateError::IoError(Backtrace::new(), e)
    }
}
