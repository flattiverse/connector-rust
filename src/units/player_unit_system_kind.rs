use crate::events::Completable;
use crate::units::player_unit_system_upgradepath::PlayerUnitSystemUpgradePath;
use serde_derive::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub enum PlayerUnitSystemKind {
    #[default]
    #[serde(rename = "hull", alias = "Hull")]
    Hull,
    #[serde(rename = "shield", alias = "Shield")]
    Shield,
    #[serde(rename = "armor", alias = "Armor")]
    Armor,
    #[serde(rename = "thruster", alias = "Thruster")]
    Thruster,
    #[serde(rename = "nozzle", alias = "Nozzle")]
    Nozzle,
    #[serde(rename = "scanner", alias = "Scanner")]
    Scanner,
    #[serde(rename = "analyzer", alias = "Analyzer")]
    Analyzer,
    #[serde(rename = "cellsenergy", alias = "CellsEnergy")]
    CellsEnergy,
    #[serde(rename = "cellsparticles", alias = "CellsParticles")]
    CellsParticles,
    #[serde(rename = "batteryenergy", alias = "BatteryEnergy")]
    BatteryEnergy,
    #[serde(rename = "batteryparticles", alias = "BatteryParticles")]
    BatteryParticles,
    #[serde(rename = "weaponammunition", alias = "WeaponAmmunition")]
    WeaponAmmunition,
    #[serde(rename = "weaponlauncher", alias = "WeaponLauncher")]
    WeaponLauncher,
    #[serde(rename = "weaponpayloaddamage", alias = "WeaponPayloadDamage")]
    WeaponPayloadDamage,
    #[serde(rename = "weaponpayloadradius", alias = "WeaponPayloadRadius")]
    WeaponPayloadRadius,
    #[serde(rename = "weaponfactory", alias = "WeaponFactory")]
    WeaponFactory,
    #[serde(rename = "weaponstorage", alias = "WeaponStorage")]
    WeaponStorage,
    #[serde(rename = "cargoiron", alias = "CargoIron")]
    CargoIron,
    #[serde(rename = "cargocarbon", alias = "CargoCarbon")]
    CargoCarbon,
    #[serde(rename = "cargosilicon", alias = "CargoSilicon")]
    CargoSilicon,
    #[serde(rename = "cargoplatinum", alias = "CargoPlatinum")]
    CargoPlatinum,
    #[serde(rename = "cargogold", alias = "CargoGold")]
    CargoGold,
    #[serde(rename = "cargospecial", alias = "CargoSpecial")]
    CargoSpecial,
    #[serde(rename = "extractoriron", alias = "ExtractorIron")]
    ExtractorIron,
    #[serde(rename = "extractorcarbon", alias = "ExtractorCarbon")]
    ExtractorCarbon,
    #[serde(rename = "extractorsilicon", alias = "ExtractorSilicon")]
    ExtractorSilicon,
    #[serde(rename = "extractorplatinum", alias = "ExtractorPlatinum")]
    ExtractorPlatinum,
    #[serde(rename = "extractorgold", alias = "ExtractorGold")]
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
