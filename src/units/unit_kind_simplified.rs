use crate::units::unit_kind::UnitKind;
use serde_derive::{Deserialize, Serialize};

/// Has the same members as [`UnitKind`] but does not contain any value
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum SimpleUnitKind {
    #[serde(rename = "sun")]
    Sun,
    #[serde(rename = "planet")]
    Planet,
    #[serde(rename = "moon")]
    Moon,
    #[serde(rename = "meteoroid")]
    Meteoroid,
    #[serde(rename = "comet")]
    Comet,
    #[serde(rename = "buoy")]
    Buoy,
    #[serde(rename = "missionTarget")]
    MissionTarget,
    #[serde(rename = "playerUnit")]
    PlayerUnit,
    #[serde(rename = "shot")]
    Shot,
    #[serde(rename = "explosion")]
    Explosion,
    #[serde(rename = "blackhole")]
    BlackHole,
}

impl From<&UnitKind> for SimpleUnitKind {
    fn from(kind: &UnitKind) -> Self {
        match kind {
            UnitKind::Sun(_) => SimpleUnitKind::Sun,
            UnitKind::Planet(_) => SimpleUnitKind::Planet,
            UnitKind::Moon(_) => SimpleUnitKind::Moon,
            UnitKind::Meteoroid(_) => SimpleUnitKind::Meteoroid,
            UnitKind::Comet(_) => SimpleUnitKind::Comet,
            UnitKind::Buoy(_) => SimpleUnitKind::Buoy,
            UnitKind::MissionTarget(_) => SimpleUnitKind::MissionTarget,
            UnitKind::PlayerUnit(_) => SimpleUnitKind::PlayerUnit,
            UnitKind::Shot(_) => SimpleUnitKind::Shot,
            UnitKind::Explosion(_) => SimpleUnitKind::Explosion,
            UnitKind::BlackHole(_) => SimpleUnitKind::BlackHole,
            UnitKind::Reduced(reduced) => reduced.probable_kind,
        }
    }
}

impl UnitKind {
    #[inline]
    pub fn simplified(&self) -> SimpleUnitKind {
        SimpleUnitKind::from(self)
    }
}
