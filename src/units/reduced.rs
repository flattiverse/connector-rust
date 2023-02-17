use crate::units::mobility::Mobility;
use crate::units::unit_kind_simplified::SimpleUnitKind;
use serde_derive::{Deserialize, Serialize};

/// A unit that has not been properly identified yet. Use the analyzer system to find out the actual
/// kind.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reduced {
    #[serde(rename = "probableKind")]
    pub probable_kind: SimpleUnitKind,
}

impl Reduced {
    /// Returns the most pessimistic [`Mobility`] for each kind
    pub fn mobility(&self) -> Mobility {
        match self.probable_kind {
            SimpleUnitKind::Sun => Mobility::Steady,
            SimpleUnitKind::Planet => Mobility::Steady,
            SimpleUnitKind::Moon => Mobility::Steady,
            SimpleUnitKind::Meteoroid => Mobility::Steady,
            SimpleUnitKind::Comet => Mobility::Steady,
            SimpleUnitKind::Buoy => Mobility::Steady,
            SimpleUnitKind::MissionTarget => Mobility::Steady,
            SimpleUnitKind::PlayerUnit => Mobility::Mobile,
            SimpleUnitKind::Shot => Mobility::Steady,
            SimpleUnitKind::Explosion => Mobility::Steady,
            SimpleUnitKind::BlackHole => Mobility::Steady,
            SimpleUnitKind::Resource => Mobility::Steady,
        }
    }
}
