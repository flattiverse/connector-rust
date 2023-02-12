use crate::events::Completable;
use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use crate::units::player_unit_system_upgradepath::PlayerUnitSystemUpgradePath;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerUnitSystem<T: Default + Completable<PlayerUnitSystemUpgradePath>> {
    pub level: u32,
    #[serde(skip_serializing_if = "is_zero", default)]
    pub value: f64,
    #[serde(skip_serializing_if = "is_zero", default)]
    pub area_increase: f64,
    #[serde(skip_serializing_if = "is_zero", default)]
    pub weight_increase: f64,
    pub kind: PlayerUnitSystemKind,
    #[serde(skip, default)]
    pub upgrade_path: Option<PlayerUnitSystemUpgradePath>,
    #[serde(skip, default)]
    pub max_level: u32,
    #[serde(skip, default)]
    pub specialization: T,
}

impl<T: Default + Completable<PlayerUnitSystemUpgradePath>> PlayerUnitSystem<T> {
    #[inline]
    pub fn area_increase(&self) -> f64 {
        self.upgrade_path
            .as_ref()
            .map(|s| s.area_increase)
            .unwrap_or_default()
    }

    #[inline]
    pub fn weight_increase(&self) -> f64 {
        self.upgrade_path
            .as_ref()
            .map(|s| s.weight_increase)
            .unwrap_or_default()
    }
}

impl<T: Default + Completable<PlayerUnitSystemUpgradePath>>
    Completable<(PlayerUnitSystemKind, &UniverseGroup)> for PlayerUnitSystem<T>
{
    fn complete(&mut self, (kind, group): &(PlayerUnitSystemKind, &UniverseGroup)) {
        let upgrade_path = group
            .get_player_unit_system_upgrade_path(*kind, self.level)
            .cloned();

        self.kind = *kind;
        self.upgrade_path = upgrade_path;
        if let Some(path) = &self.upgrade_path {
            self.specialization.complete(path);
        }
    }
}

fn is_zero(value: &f64) -> bool {
    *value == 0.0
}
