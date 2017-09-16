
use std::sync::Arc;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::Controllable;
use controllable::ControllableData;

use controllable::any_controllable::prelude::*;

pub struct Platform {
    controllable: ControllableData,
}

impl Platform {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Platform, Error>  {
        Ok(Platform {
            controllable: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

impl AsRef<ControllableData> for Platform {
    fn as_ref(&self) -> &ControllableData {
        &self.controllable
    }
}


// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Controllable for Platform {
    fn id(&self) -> u8 {
        self.as_ref().id()
    }

    fn revision(&self) -> i64 {
        self.as_ref().revision()
    }

    fn class(&self) -> &str {
        self.as_ref().class()
    }

    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn level(&self) -> u8 {
        self.as_ref().level()
    }

    fn radius(&self) -> f32 {
        self.as_ref().radius()
    }

    fn gravity(&self) -> f32 {
        self.as_ref().gravity()
    }

    fn efficiency_tactical(&self) -> f32 {
        self.as_ref().efficiency_tactical()
    }

    fn efficiency_economical(&self) -> f32 {
        self.as_ref().efficiency_economical()
    }

    fn visible_range_multiplier(&self) -> f32 {
        self.as_ref().visible_range_multiplier()
    }

    fn energy_max(&self) -> f32 {
        self.as_ref().energy_max()
    }

    fn particles_max(&self) -> f32 {
        self.as_ref().particles_max()
    }

    fn ions_max(&self) -> f32 {
        self.as_ref().ions_max()
    }

    fn energy_cells(&self) -> f32 {
        self.as_ref().energy_cells()
    }

    fn particles_cells(&self) -> f32 {
        self.as_ref().particles_cells()
    }

    fn ions_cells(&self) -> f32 {
        self.as_ref().ions_cells()
    }

    fn energy_reactor(&self) -> f32 {
        self.as_ref().energy_reactor()
    }

    fn particles_reactor(&self) -> f32 {
        self.as_ref().particles_reactor()
    }

    fn ions_reactor(&self) -> f32 {
        self.as_ref().ions_reactor()
    }

    fn hull_max(&self) -> f32 {
        self.as_ref().hull_max()
    }

    fn hull_armor(&self) -> f32 {
        self.as_ref().hull_armor()
    }

    fn hull_repair(&self) -> &EnergyCost {
        self.as_ref().hull_repair()
    }

    fn shield_max(&self) -> f32 {
        self.as_ref().shield_max()
    }

    fn shield_armor(&self) -> f32 {
        self.as_ref().shield_armor()
    }

    fn shield_load(&self) -> &EnergyCost {
        self.as_ref().shield_load()
    }

    fn engine_speed(&self) -> f32 {
        self.as_ref().engine_speed()
    }

    fn engine_acceleration(&self) -> &EnergyCost {
        self.as_ref().engine_acceleration()
    }

    fn scanner_degree_per_scan(&self) -> f32 {
        self.as_ref().scanner_degree_per_scan()
    }

    fn scanner_count(&self) -> u8 {
        self.as_ref().scanner_count()
    }

    fn scanner_area(&self) -> &ScanEnergyCost {
        self.as_ref().scanner_area()
    }

    fn weapon_shot(&self) -> &WeaponEnergyCost {
        self.as_ref().weapon_shot()
    }

    fn weapon_hull(&self) -> f32 {
        self.as_ref().weapon_hull()
    }

    fn weapon_hull_armor(&self) -> f32 {
        self.as_ref().weapon_hull_armor()
    }

    fn weapon_shield(&self) -> f32 {
        self.as_ref().weapon_shield()
    }

    fn weapon_shield_armor(&self) -> f32 {
        self.as_ref().weapon_shield_armor()
    }

    fn weapon_visible_range_multiplier(&self) -> f32 {
        self.as_ref().weapon_visible_range_multiplier()
    }

    fn weapon_gravity(&self) -> f32 {
        self.as_ref().weapon_gravity()
    }

    fn weapon_size(&self) -> f32 {
        self.as_ref().weapon_size()
    }

    fn weapon_production(&self) -> f32 {
        self.as_ref().weapon_production()
    }

    fn weapon_production_load(&self) -> f32 {
        self.as_ref().weapon_production_load()
    }

    fn weapon_sub_directions(&self) -> u8 {
        self.as_ref().weapon_sub_directions()
    }

    fn weapon_sub_directions_length(&self) -> f32 {
        self.as_ref().weapon_sub_directions_length()
    }

    fn builder_time(&self) -> f32 {
        self.as_ref().builder_time()
    }

    fn builder_time_factory_energy(&self) -> f32 {
        self.as_ref().builder_time_factory_energy()
    }

    fn builder_time_factory_particles(&self) -> f32 {
        self.as_ref().builder_time_factory_particles()
    }

    fn builder_time_factory_ions(&self) -> f32 {
        self.as_ref().builder_time_factory_ions()
    }

    fn builder_capabilities(&self) -> &Vec<UnitKind> {
        self.as_ref().builder_capabilities()
    }

    fn energy_transfer_energy(&self) -> &EnergyCost {
        self.as_ref().energy_transfer_energy()
    }

    fn energy_transfer_particles(&self) -> &EnergyCost {
        self.as_ref().energy_transfer_particles()
    }

    fn energy_transfer_ions(&self) -> &EnergyCost {
        self.as_ref().energy_transfer_ions()
    }

    fn cargo_slots(&self) -> u8 {
        self.as_ref().cargo_slots()
    }

    fn cargo_amount(&self) -> f32 {
        self.as_ref().cargo_amount()
    }

    fn crystal_converter(&self) -> &EnergyCost {
        self.as_ref().crystal_converter()
    }

    fn crystal_slots(&self) -> u8 {
        self.as_ref().crystal_slots()
    }

    fn tractor_beam(&self) -> &EnergyCost {
        self.as_ref().tractor_beam()
    }

    fn tractor_beam_range(&self) -> f32 {
        self.as_ref().tractor_beam_range()
    }

    fn scores(&self) -> &Arc<Scores> {
        self.as_ref().scores()
    }

    fn energy(&self) -> f32 {
        self.as_ref().energy()
    }

    fn particles(&self) -> f32 {
        self.as_ref().particles()
    }

    fn ions(&self) -> f32 {
        self.as_ref().ions()
    }

    fn hull(&self) -> f32 {
        self.as_ref().hull()
    }

    fn shield(&self) -> f32 {
        self.as_ref().shield()
    }

    fn build_position(&self) -> Option<Vector> {
        self.as_ref().build_position()
    }

    fn build_progress(&self) -> f32 {
        self.as_ref().build_progress()
    }

    fn is_building(&self) -> Option<AnyControllable> {
        self.as_ref().is_building()
    }

    fn is_built_by(&self) -> Option<AnyControllable> {
        self.as_ref().is_built_by()
    }

    fn weapon_production_status(&self) -> f32 {
        self.as_ref().weapon_production_status()
    }

    fn crystals(&self) -> RwLockReadGuard<Vec<Arc<CrystalCargoItem>>> {
        self.as_ref().crystals()
    }

    fn cargo_items(&self) -> RwLockReadGuard<Vec<AnyCargoItem>> {
        self.as_ref().cargo_items()
    }

    fn universe(&self) -> &Weak<Universe> {
        self.as_ref().universe()
    }

    fn haste_time(&self) -> u16 {
        self.as_ref().haste_time()
    }

    fn double_damage_time(&self) -> u16 {
        self.as_ref().double_damage_time()
    }

    fn quad_damage_time(&self) -> u16 {
        self.as_ref().quad_damage_time()
    }

    fn cloak_time(&self) -> u16 {
        self.as_ref().cloak_time()
    }

    fn connector(&self) -> &Weak<Connector> {
        self.as_ref().connector()
    }

    fn is_active(&self) -> bool {
        self.as_ref().is_active()
    }

    fn is_pending_shutdown(&self) -> bool {
        self.as_ref().is_pending_shutdown()
    }

    fn scan_list(&self) -> &RwLock<Vec<Arc<Unit>>> {
        self.as_ref().scan_list()
    }

    fn update(&self, packet: &Packet) -> Result<(), Error> {
        self.as_ref().update(packet)
    }

    fn update_extended(&self, packet: &Packet) -> Result<(), Error> {
        self.as_ref().update_extended(packet)
    }

    fn set_crystals(&self, crystals: Vec<Arc<CrystalCargoItem>>) -> Result<(), Error> {
        self.as_ref().set_crystals(crystals)
    }

    fn set_cargo_items(&self, items: Vec<AnyCargoItem>) -> Result<(), Error> {
        self.as_ref().set_cargo_items(items)
    }

    fn set_scan_list(&self, list: Vec<Arc<Unit>>) -> Result<(), Error> {
        self.as_ref().set_scan_list(list)
    }

    fn set_active(&self, active: bool) -> Result<(), Error> {
        self.as_ref().set_active(active)
    }
}