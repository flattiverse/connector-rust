use serde_derive::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct GameRegionId(pub(crate) usize);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameRegion {
    #[serde(rename = "regionId")]
    pub id: GameRegionId,
    #[serde(rename = "teams")]
    pub team_mask: u16,
    pub name: Option<String>,
    #[serde(rename = "startLocation")]
    pub start_location: bool,
    #[serde(rename = "safeZone")]
    pub safe_zone: bool,
    #[serde(rename = "slowRestore")]
    pub slow_restore: bool,
    #[serde(flatten)]
    pub region: Region,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
