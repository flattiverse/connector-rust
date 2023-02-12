use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Mobility {
    Still,
    Steady,
    Mobile,
}

impl From<&[Orbit]> for Mobility {
    #[inline]
    fn from(value: &[Orbit]) -> Self {
        if value.is_empty() {
            Self::Still
        } else {
            Self::Steady
        }
    }
}
