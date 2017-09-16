
use std::sync::Arc;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::Controllable;
use controllable::ControllableData;

use controllable::any_controllable::prelude::*;

pub struct Drone {
    pub(crate) controllable: ControllableData,
}

impl Drone {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Drone, Error>  {
        Ok(Drone {
            controllable: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Controllable for Drone {
    fn id(&self) -> u8 {
        self.controllable.id()
    }

    fn revision(&self) -> i64 {
        self.controllable.revision()
    }

    fn class(&self) -> &str {
        self.controllable.class()
    }

    fn name(&self) -> &str {
        self.controllable.name()
    }

    fn level(&self) -> u8 {
        self.controllable.level()
    }

    fn radius(&self) -> f32 {
        self.controllable.radius()
    }

    fn gravity(&self) -> f32 {
        self.controllable.gravity()
    }

    fn efficiency_tactical(&self) -> f32 {
        self.controllable.efficiency_tactical()
    }

    fn efficiency_economical(&self) -> f32 {
        self.controllable.efficiency_economical()
    }

    fn visible_range_multiplier(&self) -> f32 {
        self.controllable.visible_range_multiplier()
    }

    fn energy_max(&self) -> f32 {
        self.controllable.energy_max()
    }

    fn particles_max(&self) -> f32 {
        self.controllable.particles_max()
    }

    fn ions_max(&self) -> f32 {
        self.controllable.ions_max()
    }

    fn energy_cells(&self) -> f32 {
        self.controllable.energy_cells()
    }

    fn particles_cells(&self) -> f32 {
        self.controllable.particles_cells()
    }

    fn ions_cells(&self) -> f32 {
        self.controllable.ions_cells()
    }

    fn energy_reactor(&self) -> f32 {
        self.controllable.energy_reactor()
    }

    fn particles_reactor(&self) -> f32 {
        self.controllable.particles_reactor()
    }

    fn ions_reactor(&self) -> f32 {
        self.controllable.ions_reactor()
    }

    fn hull_max(&self) -> f32 {
        self.controllable.hull_max()
    }

    fn hull_armor(&self) -> f32 {
        self.controllable.hull_armor()
    }

    fn hull_repair(&self) -> &EnergyCost {
        self.controllable.hull_repair()
    }

    fn shield_max(&self) -> f32 {
        self.controllable.shield_max()
    }

    fn shield_armor(&self) -> f32 {
        self.controllable.shield_armor()
    }

    fn shield_load(&self) -> &EnergyCost {
        self.controllable.shield_load()
    }

    fn engine_speed(&self) -> f32 {
        self.controllable.engine_speed()
    }

    fn engine_acceleration(&self) -> &EnergyCost {
        self.controllable.engine_acceleration()
    }

    fn scanner_degree_per_scan(&self) -> f32 {
        self.controllable.scanner_degree_per_scan()
    }

    fn scanner_count(&self) -> u8 {
        self.controllable.scanner_count()
    }

    fn scanner_area(&self) -> &ScanEnergyCost {
        self.controllable.scanner_area()
    }

    fn weapon_shot(&self) -> &WeaponEnergyCost {
        self.controllable.weapon_shot()
    }

    fn weapon_hull(&self) -> f32 {
        self.controllable.weapon_hull()
    }

    fn weapon_hull_armor(&self) -> f32 {
        self.controllable.weapon_hull_armor()
    }

    fn weapon_shield(&self) -> f32 {
        self.controllable.weapon_shield()
    }

    fn weapon_shield_armor(&self) -> f32 {
        self.controllable.weapon_shield_armor()
    }

    fn weapon_visible_range_multiplier(&self) -> f32 {
        self.controllable.weapon_visible_range_multiplier()
    }

    fn weapon_gravity(&self) -> f32 {
        self.controllable.weapon_gravity()
    }

    fn weapon_size(&self) -> f32 {
        self.controllable.weapon_size()
    }

    fn weapon_production(&self) -> f32 {
        self.controllable.weapon_production()
    }

    fn weapon_production_load(&self) -> f32 {
        self.controllable.weapon_production_load()
    }

    fn weapon_sub_directions(&self) -> u8 {
        self.controllable.weapon_sub_directions()
    }

    fn weapon_sub_directions_length(&self) -> f32 {
        self.controllable.weapon_sub_directions_length()
    }

    fn builder_time(&self) -> f32 {
        self.controllable.builder_time()
    }

    fn builder_time_factory_energy(&self) -> f32 {
        self.controllable.builder_time_factory_energy()
    }

    fn builder_time_factory_particles(&self) -> f32 {
        self.controllable.builder_time_factory_particles()
    }

    fn builder_time_factory_ions(&self) -> f32 {
        self.controllable.builder_time_factory_ions()
    }

    fn builder_capabilities(&self) -> &Vec<UnitKind> {
        self.controllable.builder_capabilities()
    }

    fn energy_transfer_energy(&self) -> &EnergyCost {
        self.controllable.energy_transfer_energy()
    }

    fn energy_transfer_particles(&self) -> &EnergyCost {
        self.controllable.energy_transfer_particles()
    }

    fn energy_transfer_ions(&self) -> &EnergyCost {
        self.controllable.energy_transfer_ions()
    }

    fn cargo_slots(&self) -> u8 {
        self.controllable.cargo_slots()
    }

    fn cargo_amount(&self) -> f32 {
        self.controllable.cargo_amount()
    }

    fn crystal_converter(&self) -> &EnergyCost {
        self.controllable.crystal_converter()
    }

    fn crystal_slots(&self) -> u8 {
        self.controllable.crystal_slots()
    }

    fn tractor_beam(&self) -> &EnergyCost {
        self.controllable.tractor_beam()
    }

    fn tractor_beam_range(&self) -> f32 {
        self.controllable.tractor_beam_range()
    }

    fn scores(&self) -> &Arc<Scores> {
        self.controllable.scores()
    }

    fn energy(&self) -> f32 {
        self.controllable.energy()
    }

    fn particles(&self) -> f32 {
        self.controllable.particles()
    }

    fn ions(&self) -> f32 {
        self.controllable.ions()
    }

    fn hull(&self) -> f32 {
        self.controllable.hull()
    }

    fn shield(&self) -> f32 {
        self.controllable.shield()
    }

    fn build_position(&self) -> Option<Vector> {
        self.controllable.build_position()
    }

    fn build_progress(&self) -> f32 {
        self.controllable.build_progress()
    }

    fn is_building(&self) -> Option<AnyControllable> {
        self.controllable.is_building()
    }

    fn is_built_by(&self) -> Option<AnyControllable> {
        self.controllable.is_built_by()
    }

    fn weapon_production_status(&self) -> f32 {
        self.controllable.weapon_production_status()
    }

    fn crystals(&self) -> RwLockReadGuard<Vec<Arc<CrystalCargoItem>>> {
        self.controllable.crystals()
    }

    fn cargo_items(&self) -> RwLockReadGuard<Vec<AnyCargoItem>> {
        self.controllable.cargo_items()
    }

    fn universe(&self) -> &Weak<Universe> {
        self.controllable.universe()
    }

    fn haste_time(&self) -> u16 {
        self.controllable.haste_time()
    }

    fn double_damage_time(&self) -> u16 {
        self.controllable.double_damage_time()
    }

    fn quad_damage_time(&self) -> u16 {
        self.controllable.quad_damage_time()
    }

    fn cloak_time(&self) -> u16 {
        self.controllable.cloak_time()
    }

    fn connector(&self) -> &Weak<Connector> {
        self.controllable.connector()
    }

    fn is_active(&self) -> bool {
        self.controllable.is_active()
    }

    fn is_pending_shutdown(&self) -> bool {
        self.controllable.is_pending_shutdown()
    }

    fn scan_list(&self) -> &RwLock<Vec<AnyUnit>> {
        self.controllable.scan_list()
    }

    fn update(&self, packet: &Packet) -> Result<(), Error> {
        self.controllable.update(packet)
    }

    fn update_extended(&self, packet: &Packet) -> Result<(), Error> {
        self.controllable.update_extended(packet)
    }

    fn set_crystals(&self, crystals: Vec<Arc<CrystalCargoItem>>) -> Result<(), Error> {
        self.controllable.set_crystals(crystals)
    }

    fn set_cargo_items(&self, items: Vec<AnyCargoItem>) -> Result<(), Error> {
        self.controllable.set_cargo_items(items)
    }

    fn set_scan_list(&self, list: Vec<AnyUnit>) -> Result<(), Error> {
        self.controllable.set_scan_list(list)
    }

    fn set_active(&self, active: bool) -> Result<(), Error> {
        self.controllable.set_active(active)
    }
}