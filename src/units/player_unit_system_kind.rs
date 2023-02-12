use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerUnitSystemKind {
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
