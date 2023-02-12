use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use crate::units::player_unit_system_upgradepath::PlayerUnitSystemUpgradePath;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerUnitSystem {
    pub level: i32,
    pub value: Option<f64>,
    pub kind: PlayerUnitSystemKind,
    pub system: PlayerUnitSystemUpgradePath,
}

impl PlayerUnitSystem {
    #[inline]
    pub fn area_increase(&self) -> f64 {
        self.system.area_increase
    }

    #[inline]
    pub fn weight_increase(&self) -> f64 {
        self.system.weight_increase
    }

    pub fn system(&self) -> &PlayerUnitSystemUpgradePath {
        todo!()
    }

    pub fn max_level(&self) -> i32 {
        todo!()
    }
}
