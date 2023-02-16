use crate::events::Completable;
use crate::units::player_unit_system_upgradepath::PlayerUnitSystemUpgradePath;
use serde_derive::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub enum PlayerUnitSystemKind {
    #[default]
    Hull,
    Shield,
    Armor,
    Thruster,
    Nozzle,
    Scanner,
    Analyzer,
    CellsEnergy,
    CellsParticles,
    BatteryEnergy,
    BatteryParticles,
    WeaponAmmunition,
    WeaponLauncher,
    WeaponPayloadDamage,
    WeaponPayloadRadius,
    WeaponFactory,
    WeaponStorage,
    CargoIron,
    CargoCarbon,
    CargoSilicon,
    CargoPlatinum,
    CargoGold,
    CargoSpecial,
    ExtractorIron,
    ExtractorCarbon,
    ExtractorSilicon,
    ExtractorPlatinum,
    ExtractorGold,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RegularSystem {
    pub max_value: f64,
}

impl Completable<PlayerUnitSystemUpgradePath> for RegularSystem {
    fn complete(&mut self, path: &PlayerUnitSystemUpgradePath) {
        self.max_value = path.value0;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ArmorSystem {
    pub max_value: f64,
    pub iron_usage: f64,
    pub platinum_usage: f64,
}

impl Completable<PlayerUnitSystemUpgradePath> for ArmorSystem {
    fn complete(&mut self, path: &PlayerUnitSystemUpgradePath) {
        self.max_value = path.value0;
        self.iron_usage = path.value1;
        self.platinum_usage = path.value2;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EnergyConsumingSystem {
    pub max_value: f64,
    pub energy_usage: f64,
    pub particle_usage: f64,
}

impl Completable<PlayerUnitSystemUpgradePath> for EnergyConsumingSystem {
    fn complete(&mut self, path: &PlayerUnitSystemUpgradePath) {
        self.max_value = path.value0;
        self.energy_usage = path.value1;
        self.particle_usage = path.value2;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ScannerSystem {
    pub max_range: f64,
    pub max_angle: f64,
    pub energy_usage_per_surface_unit: f64,
}

impl Completable<PlayerUnitSystemUpgradePath> for ScannerSystem {
    fn complete(&mut self, path: &PlayerUnitSystemUpgradePath) {
        self.max_range = path.value0;
        self.max_angle = path.value1;
        self.energy_usage_per_surface_unit = path.value2;
    }
}
