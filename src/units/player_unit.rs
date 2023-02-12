use crate::controllable::ControllableId;
use crate::events::Completable;
use crate::players::PlayerId;
use crate::units::player_unit_system::PlayerUnitSystem;
use crate::units::player_unit_system_kind::{
    ArmorSystem, EnergyConsumingSystem, PlayerUnitSystemKind, RegularSystem, ScannerSystem,
};
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerUnit {
    pub player: PlayerId,
    pub controllable: ControllableId,
    #[serde(rename = "turnRate")]
    pub turn_rate: f64,
    #[serde(rename = "requestedScanDirection")]
    pub requested_scan_direction: f64,
    #[serde(rename = "requestedScanWidth")]
    pub requested_scan_width: f64,
    #[serde(rename = "requestedScanRange")]
    pub requested_scan_range: f64,
    #[serde(rename = "scanDirection")]
    pub scan_direction: f64,
    #[serde(rename = "scanWidth")]
    pub scan_width: f64,
    #[serde(rename = "scanRange")]
    pub scan_range: f64,
    pub systems: PlayerUnitSystems,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerUnitSystems {
    #[serde(rename = "Hull")]
    pub hull: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "Shield")]
    pub shield: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "Armor")]
    pub armor: PlayerUnitSystem<ArmorSystem>,
    #[serde(rename = "Thruster")]
    pub thruster: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "Nozzle")]
    pub nozzle: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "Scanner")]
    pub scanner: PlayerUnitSystem<ScannerSystem>,
    #[serde(rename = "Analyzer")]
    pub analyzer: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "CellsEnergy")]
    pub cells_energy: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CellsParticles")]
    pub cells_particles: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "BatteryParticles")]
    pub battery_energy: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "WeaponLauncher")]
    pub weapon_launcher: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "WeaponPayloadDamage")]
    pub weapon_payload_damage: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "WeaponPayloadRadius")]
    pub weapon_payload_radius: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "WeaponFactory")]
    pub weapon_factory: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "WeaponStorage")]
    pub weapon_storage: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CargoIron")]
    pub cargo_iron: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CargoCarbon")]
    pub cargo_carbon: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CargoSilicon")]
    pub cargo_silicon: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CargoPlatinum")]
    pub cargo_platinum: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CargoGold")]
    pub cargo_gold: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "CargoSpecial")]
    pub cargo_special: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "ExtractorIron")]
    pub extractor_iron: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "ExtractorCarbon")]
    pub extractor_carbon: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "ExtractorSilicon")]
    pub extractor_silicon: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "ExtractorPlatinum")]
    pub extractor_platinum: PlayerUnitSystem<EnergyConsumingSystem>,
    #[serde(rename = "ExtractorGold")]
    pub extractor_gold: PlayerUnitSystem<EnergyConsumingSystem>,
}

impl Completable<UniverseGroup> for PlayerUnitSystems {
    fn complete(&mut self, group: &UniverseGroup) {
        self.hull.complete(&(PlayerUnitSystemKind::Hull, group));
        self.shield.complete(&(PlayerUnitSystemKind::Shield, group));
        self.armor.complete(&(PlayerUnitSystemKind::Armor, group));
        self.thruster
            .complete(&(PlayerUnitSystemKind::Thruster, group));
        self.nozzle.complete(&(PlayerUnitSystemKind::Nozzle, group));
        self.scanner
            .complete(&(PlayerUnitSystemKind::Scanner, group));
        self.analyzer
            .complete(&(PlayerUnitSystemKind::Analyzer, group));
        self.cells_energy
            .complete(&(PlayerUnitSystemKind::CellsEnergy, group));
        self.cells_particles
            .complete(&(PlayerUnitSystemKind::CellsParticles, group));
        self.battery_energy
            .complete(&(PlayerUnitSystemKind::BatteryEnergy, group));
        self.weapon_launcher
            .complete(&(PlayerUnitSystemKind::WeaponLauncher, group));
        self.weapon_payload_damage
            .complete(&(PlayerUnitSystemKind::WeaponPayloadDamage, group));
        self.weapon_payload_radius
            .complete(&(PlayerUnitSystemKind::WeaponPayloadRadius, group));
        self.weapon_factory
            .complete(&(PlayerUnitSystemKind::WeaponFactory, group));
        self.weapon_storage
            .complete(&(PlayerUnitSystemKind::WeaponStorage, group));
        self.cargo_iron
            .complete(&(PlayerUnitSystemKind::CargoIron, group));
        self.cargo_carbon
            .complete(&(PlayerUnitSystemKind::CargoCarbon, group));
        self.cargo_silicon
            .complete(&(PlayerUnitSystemKind::CargoSilicon, group));
        self.cargo_platinum
            .complete(&(PlayerUnitSystemKind::CargoPlatinum, group));
        self.cargo_gold
            .complete(&(PlayerUnitSystemKind::CargoGold, group));
        self.cargo_special
            .complete(&(PlayerUnitSystemKind::CargoSpecial, group));
        self.extractor_iron
            .complete(&(PlayerUnitSystemKind::ExtractorIron, group));
        self.extractor_carbon
            .complete(&(PlayerUnitSystemKind::ExtractorCarbon, group));
        self.extractor_silicon
            .complete(&(PlayerUnitSystemKind::ExtractorSilicon, group));
        self.extractor_platinum
            .complete(&(PlayerUnitSystemKind::ExtractorPlatinum, group));
        self.extractor_gold
            .complete(&(PlayerUnitSystemKind::ExtractorGold, group));
    }
}
