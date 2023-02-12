use crate::units::blackhole::Blackhole;
use crate::units::comet::Comet;
use crate::units::explosion::Explosion;
use crate::units::meteoroid::Meteoroid;
use crate::units::moon::Moon;
use crate::units::planet::Planet;
use crate::units::player_unit::PlayerUnit;
use crate::units::shot::Shot;
use crate::units::sun::Sun;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum UnitKind {
    /// A [`Sun`], which may have corona or corona sections.
    #[serde(rename = "sun")]
    Sun(Sun),
    #[serde(rename = "planet")]
    Planet(Planet),
    #[serde(rename = "moon")]
    Moon(Moon),
    #[serde(rename = "meteoroid")]
    Meteoroid(Meteoroid),
    #[serde(rename = "comet")]
    Comet(Comet),
    // #[serde(rename = "asteroid")]
    // Asteroid,
    // /// A buoy, which may contain a message.
    // #[serde(rename = "buoy")]
    // Buoy,
    // /// A [`MissionTarget`], which you may have to shoot at.
    // #[serde(rename = "missionTarget")]
    // MissionTarget,
    /// A [`PlayerUnit`]. May be friendly. Or not.
    #[serde(rename = "playerUnit")]
    PlayerUnit(PlayerUnit),
    /// A shot. Better not touch.
    #[serde(rename = "shot")]
    Shot(Shot),
    /// An explosion. Hope you are far away.
    #[serde(rename = "explosion")]
    Explosion(Explosion),
    /// A [`BlackHole`], which may have any gravitational well or gravitational well sections.
    #[serde(rename = "blackhole")]
    BlackHole(Blackhole),
}
