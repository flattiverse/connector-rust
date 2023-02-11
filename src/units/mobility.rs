use crate::units::orbits::Orbits;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Mobility {
    Still,
    Steady,
    Mobile,
}

impl From<&[Orbits]> for Mobility {
    #[inline]
    fn from(value: &[Orbits]) -> Self {
        if value.is_empty() {
            Self::Still
        } else {
            Self::Steady
        }
    }
}
