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
    pub hull: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "cellsEnergy")]
    pub cells_energy: PlayerUnitSystem<RegularSystem>,
    #[serde(rename = "batteryParticles")]
    pub battery_energy: PlayerUnitSystem<RegularSystem>,
    pub thruster: PlayerUnitSystem<EnergyConsumingSystem>,
    pub nozzle: PlayerUnitSystem<EnergyConsumingSystem>,
    pub scanner: PlayerUnitSystem<ScannerSystem>,

    pub armor: Option<PlayerUnitSystem<ArmorSystem>>,
    pub shield: Option<PlayerUnitSystem<RegularSystem>>,
    pub analyzer: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    #[serde(rename = "cellsParticles")]
    pub cells_particles: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "weaponLauncher")]
    pub weapon_launcher: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "weaponPayloadDamage")]
    pub weapon_payload_damage: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "weaponPayloadRadius")]
    pub weapon_payload_radius: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "weaponFactory")]
    pub weapon_factory: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "weaponStorage")]
    pub weapon_storage: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "cargoIron")]
    pub cargo_iron: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "cargoCarbon")]
    pub cargo_carbon: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "cargoSilicon")]
    pub cargo_silicon: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "CargoPlatinum")]
    pub cargo_platinum: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "cargoGold")]
    pub cargo_gold: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "cargoSpecial")]
    pub cargo_special: Option<PlayerUnitSystem<RegularSystem>>,
    #[serde(rename = "extractorIron")]
    pub extractor_iron: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    #[serde(rename = "extractorCarbon")]
    pub extractor_carbon: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    #[serde(rename = "extractorSilicon")]
    pub extractor_silicon: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    #[serde(rename = "extractorPlatinum")]
    pub extractor_platinum: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    #[serde(rename = "extractorGold")]
    pub extractor_gold: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
}

impl Completable<UniverseGroup> for PlayerUnitSystems {
    fn complete(&mut self, group: &UniverseGroup) {
        self.hull.complete(&(PlayerUnitSystemKind::Hull, group));
        self.cells_energy
            .complete(&(PlayerUnitSystemKind::CellsEnergy, group));
        self.battery_energy
            .complete(&(PlayerUnitSystemKind::BatteryEnergy, group));
        self.thruster
            .complete(&(PlayerUnitSystemKind::Thruster, group));
        self.nozzle.complete(&(PlayerUnitSystemKind::Nozzle, group));
        self.scanner
            .complete(&(PlayerUnitSystemKind::Scanner, group));

        self.armor.complete(&(PlayerUnitSystemKind::Armor, group));
        self.shield.complete(&(PlayerUnitSystemKind::Shield, group));
        self.analyzer
            .complete(&(PlayerUnitSystemKind::Analyzer, group));
        self.cells_particles
            .complete(&(PlayerUnitSystemKind::CellsParticles, group));
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
