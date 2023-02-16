use crate::controllable::ControllableId;
use crate::events::Completable;
use crate::players::PlayerId;
use crate::units::player_unit_system::PlayerUnitSystem;
use crate::units::player_unit_system_kind::{
    ArmorSystem, EnergyConsumingSystem, PlayerUnitSystemKind, RegularSystem, ScannerSystem,
};
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerUnit {
    pub player: PlayerId,
    pub controllable: ControllableId,
    #[serde(rename = "turnRate")]
    pub turn_rate: Option<f64>,
    #[serde(rename = "scanDirection")]
    pub scan_direction: Option<f64>,
    #[serde(rename = "scanWidth")]
    pub scan_width: Option<f64>,
    #[serde(rename = "scanRange")]
    pub scan_range: Option<f64>,
    #[serde(rename = "scanActivated")]
    pub scan_activated: Option<bool>,
    pub hull: Option<f64>,
    #[serde(rename = "hullMax")]
    pub hull_max: Option<f64>,
    pub nozzle: Option<f64>,
    #[serde(rename = "nozzleMax")]
    pub nozzle_max: Option<f64>,
    pub thruster: Option<f64>,
    #[serde(rename = "thrusterMax")]
    pub thruster_max: f64,
    pub armor: Option<f64>,
    #[serde(rename = "armorMax")]
    pub armor_max: Option<f64>,
    pub shield: Option<f64>,
    #[serde(rename = "shieldMax")]
    pub shield_max: Option<f64>,
    pub energy: Option<f64>,
    #[serde(rename = "energyMax")]
    pub energy_max: Option<f64>,
    pub particles: Option<f64>,
    #[serde(rename = "particlesMax")]
    pub particles_max: Option<f64>,
    #[serde(rename = "cargoLoad")]
    pub cargo_load: Option<f64>,
    #[serde(rename = "systemLevels")]
    pub system_levels: HashMap<PlayerUnitSystemKind, u32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerUnitSystems {
    /// The hull of the [`Unit`] or your [`crate::controllable::Controllable`], keeping you away
    /// from the cold void of space.
    pub hull: PlayerUnitSystem<RegularSystem>,
    /// The energy cell of the [`Unit`] or your [`crate::controllable::Controllable`], used for
    /// recharging your energy levels.
    #[serde(rename = "cellsenergy")]
    pub cells_energy: PlayerUnitSystem<RegularSystem>,
    /// The energy battery of the [`Unit`] or your [`crate::controllable::Controllable`], used for
    /// use for storing energy.
    #[serde(rename = "batteryenergy")]
    pub battery_energy: PlayerUnitSystem<RegularSystem>,
    /// The thruster of the [`Unit`] or your [`crate::controllable::Controllable`], used to propel
    /// it through the universe.
    pub thruster: PlayerUnitSystem<EnergyConsumingSystem>,
    /// The nozzle of the [`Unit`] or your [`crate::controllable::Controllable`], used to make it
    /// spin all around.
    pub nozzle: PlayerUnitSystem<EnergyConsumingSystem>,
    /// The scanner of your the [`Unit`] or your [`crate::controllable::Controllable`], used to
    /// detect objects in the vicinity.
    pub scanner: PlayerUnitSystem<ScannerSystem>,

    /// The amor of the [`Unit`] or your [`crate::controllable::Controllable`], used to reduce
    /// damage from malicious influences.
    pub armor: Option<PlayerUnitSystem<ArmorSystem>>,
    /// The shield of the [`Unit`] or your [`crate::controllable::Controllable`], used to avoid
    /// damage altogether.
    pub shield: Option<PlayerUnitSystem<RegularSystem>>,
    /// The analyzer of the [`Unit`] or your [`crate::controllable::Controllable`], used to identify
    /// objects.
    pub analyzer: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    /// The particle cells of the [`Unit`] or your [`crate::controllable::Controllable`], used for
    /// recharging your particle levels.
    #[serde(rename = "cellsparticles")]
    pub cells_particles: Option<PlayerUnitSystem<RegularSystem>>,
    /// The particle battery of the [`Unit`] or your [`crate::controllable::Controllable`], used for
    /// storing particles.
    #[serde(rename = "batteryparticles")]
    pub battery_particles: Option<PlayerUnitSystem<RegularSystem>>,
    /// The shot lifetime of your [`crate::controllable::Controllable`]'s weapons.
    #[serde(rename = "weaponammunition")]
    pub weapon_ammunition: Option<PlayerUnitSystem<RegularSystem>>,
    /// The weapon launcher of the [`Unit`] or your [`crate::controllable::Controllable`], used to
    /// do the pew pew.
    #[serde(rename = "weaponlauncher")]
    pub weapon_launcher: Option<PlayerUnitSystem<RegularSystem>>,
    /// The damage of the [`Unit`]'s or your [`crate::controllable::Controllable`]'s weapons.
    #[serde(rename = "weaponpayloaddamage")]
    pub weapon_payload_damage: Option<PlayerUnitSystem<RegularSystem>>,
    /// The radius of the [`Unit`]'s or your [`crate::controllable::Controllable`]'s weapons'
    /// explosion.
    #[serde(rename = "weaponpayloadradius")]
    pub weapon_payload_radius: Option<PlayerUnitSystem<RegularSystem>>,
    /// The weapon factory of the [`Unit`] or your [`crate::controllable::Controllable`].
    #[serde(rename = "weaponfactory")]
    pub weapon_factory: Option<PlayerUnitSystem<RegularSystem>>,
    /// The storage capacity of your controllable for weapons
    #[serde(rename = "weaponstorage")]
    pub weapon_storage: Option<PlayerUnitSystem<RegularSystem>>,
    /// The storage capacity of iron in the [`Unit`] or your [`crate::controllable::Controllable`].
    #[serde(rename = "cargoiron")]
    pub cargo_iron: Option<PlayerUnitSystem<RegularSystem>>,
    /// The storage capacity of carbon in the [`Unit`] or your [`crate::controllable::Controllable`].
    #[serde(rename = "cargocarbon")]
    pub cargo_carbon: Option<PlayerUnitSystem<RegularSystem>>,
    /// The storage capacity of silicon in the [`Unit`] or your
    /// [`crate::controllable::Controllable`].
    #[serde(rename = "cargosilicon")]
    pub cargo_silicon: Option<PlayerUnitSystem<RegularSystem>>,
    /// The storage capacity of platinum in the [`Unit`] or your
    /// [`crate::controllable::Controllable`].
    #[serde(rename = "cargoplatinum")]
    pub cargo_platinum: Option<PlayerUnitSystem<RegularSystem>>,
    /// The storage capacity of gold in the [`Unit`] or your [`crate::controllable::Controllable`].
    #[serde(rename = "cargogold")]
    pub cargo_gold: Option<PlayerUnitSystem<RegularSystem>>,
    /// The special storage capacity in the [`Unit`] or your [`crate::controllable::Controllable`].
    #[serde(rename = "cargospecial")]
    pub cargo_special: Option<PlayerUnitSystem<RegularSystem>>,
    /// The extraction capabilities of the [`Unit`] or your [`crate::controllable::Controllable`]
    /// for iron.
    #[serde(rename = "extractoriron")]
    pub extractor_iron: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    /// The extraction capabilities of the [`Unit`] or your [`crate::controllable::Controllable`]
    /// for carbon.
    #[serde(rename = "extractorcarbon")]
    pub extractor_carbon: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    /// The extraction capabilities of the [`Unit`] or your [`crate::controllable::Controllable`]
    /// for silicon.
    #[serde(rename = "extractorsilicon")]
    pub extractor_silicon: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    /// The extraction capabilities of the [`Unit`] or your [`crate::controllable::Controllable`]
    /// for platinum.
    #[serde(rename = "extractorplatinum")]
    pub extractor_platinum: Option<PlayerUnitSystem<EnergyConsumingSystem>>,
    /// The extraction capabilities of the [`Unit`] or your [`crate::controllable::Controllable`]
    /// for gold
    #[serde(rename = "extractorgold")]
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
