
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use Scores;
use Vector;
use Universe;
use Connector;

use unit::Unit;
use unit::UnitKind;

use item::CargoItem;
use item::CrystalCargoItem;

use controllable::Controllable;
use controllable::EnergyCost;
use controllable::ScanEnergyCost;
use controllable::WeaponEnergyCost;




pub struct Empty {

}


impl Controllable for Empty {
    fn id(&self) -> u8 {
        unimplemented!()
    }

    fn revision(&self) -> i64 {
        unimplemented!()
    }

    fn class(&self) -> &str {
        unimplemented!()
    }

    fn name(&self) -> &str {
        unimplemented!()
    }

    fn level(&self) -> u8 {
        unimplemented!()
    }

    fn radius(&self) -> f32 {
        unimplemented!()
    }

    fn gravity(&self) -> f32 {
        unimplemented!()
    }

    fn efficiency_tactical(&self) -> f32 {
        unimplemented!()
    }

    fn efficiency_economical(&self) -> f32 {
        unimplemented!()
    }

    fn visible_range_multiplier(&self) -> f32 {
        unimplemented!()
    }

    fn energy_max(&self) -> f32 {
        unimplemented!()
    }

    fn particles_max(&self) -> f32 {
        unimplemented!()
    }

    fn ions_max(&self) -> f32 {
        unimplemented!()
    }

    fn energy_cells(&self) -> f32 {
        unimplemented!()
    }

    fn particles_cells(&self) -> f32 {
        unimplemented!()
    }

    fn ions_cells(&self) -> f32 {
        unimplemented!()
    }

    fn energy_reactor(&self) -> f32 {
        unimplemented!()
    }

    fn particles_reactor(&self) -> f32 {
        unimplemented!()
    }

    fn ions_reactor(&self) -> f32 {
        unimplemented!()
    }

    fn hull_max(&self) -> f32 {
        unimplemented!()
    }

    fn hull_armor(&self) -> f32 {
        unimplemented!()
    }

    fn hull_repair(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn shield_max(&self) -> f32 {
        unimplemented!()
    }

    fn shield_armor(&self) -> f32 {
        unimplemented!()
    }

    fn shield_load(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn engine_speed(&self) -> f32 {
        unimplemented!()
    }

    fn engine_acceleration(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn scanner_degree_per_scan(&self) -> f32 {
        unimplemented!()
    }

    fn scanner_count(&self) -> u8 {
        unimplemented!()
    }

    fn scanner_area(&self) -> &ScanEnergyCost {
        unimplemented!()
    }

    fn weapon_shot(&self) -> &WeaponEnergyCost {
        unimplemented!()
    }

    fn weapon_hull(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_hull_armor(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_shield(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_shield_armor(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_visible_range_multiplier(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_gravity(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_size(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_production(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_production_load(&self) -> f32 {
        unimplemented!()
    }

    fn weapon_sub_directions(&self) -> u8 {
        unimplemented!()
    }

    fn weapon_sub_directions_length(&self) -> f32 {
        unimplemented!()
    }

    fn builder_time(&self) -> f32 {
        unimplemented!()
    }

    fn builder_time_factory_energy(&self) -> f32 {
        unimplemented!()
    }

    fn builder_time_factory_particles(&self) -> f32 {
        unimplemented!()
    }

    fn builder_time_factory_ions(&self) -> f32 {
        unimplemented!()
    }

    fn builder_capabilities(&self) -> &Vec<UnitKind> {
        unimplemented!()
    }

    fn energy_transfer_energy(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn energy_transfer_particles(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn energy_transfer_ions(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn cargo_slots(&self) -> u8 {
        unimplemented!()
    }

    fn cargo_amount(&self) -> f32 {
        unimplemented!()
    }

    fn crystal_converter(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn crystal_slots(&self) -> u8 {
        unimplemented!()
    }

    fn tractor_beam(&self) -> &EnergyCost {
        unimplemented!()
    }

    fn tractor_beam_range(&self) -> f32 {
        unimplemented!()
    }

    fn scores(&self) -> &Arc<RwLock<Scores>> {
        unimplemented!()
    }

    fn energy(&self) -> f32 {
        unimplemented!()
    }

    fn particles(&self) -> f32 {
        unimplemented!()
    }

    fn ions(&self) -> f32 {
        unimplemented!()
    }

    fn hull(&self) -> f32 {
        unimplemented!()
    }

    fn shield(&self) -> f32 {
        unimplemented!()
    }

    fn build_position(&self) -> &Option<Vector> {
        unimplemented!()
    }

    fn build_progress(&self) -> f32 {
        unimplemented!()
    }

    fn is_building(&self) -> &Option<Weak<RwLock<Controllable>>> {
        unimplemented!()
    }

    fn is_built_by(&self) -> &Option<Weak<RwLock<Controllable>>> {
        unimplemented!()
    }

    fn weapon_production_status(&self) -> f32 {
        unimplemented!()
    }

    fn crystals(&self) -> Arc<Vec<Box<CrystalCargoItem>>> {
        unimplemented!()
    }

    fn set_crystals(&self, _: Arc<Vec<Box<CrystalCargoItem>>>) {
        unimplemented!()
    }

    fn cargo_items(&self) -> Arc<Vec<Box<CargoItem>>> {
        unimplemented!()
    }

    fn set_cargo_items(&self, _: Arc<Vec<Box<CargoItem>>>) {
        unimplemented!()
    }

    fn universe(&self) -> &Weak<RwLock<Universe>> {
        unimplemented!()
    }

    fn haste_time(&self) -> u16 {
        unimplemented!()
    }

    fn double_damage_time(&self) -> u16 {
        unimplemented!()
    }

    fn quad_damage_time(&self) -> u16 {
        unimplemented!()
    }

    fn cloak_time(&self) -> u16 {
        unimplemented!()
    }

    fn connector(&self) -> &Weak<Connector> {
        unimplemented!()
    }

    fn active(&self) -> bool {
        unimplemented!()
    }

    fn pending_shutdown(&self) -> bool {
        unimplemented!()
    }

    fn scan_list(&self) -> &RwLock<Vec<Box<Unit>>> {
        unimplemented!()
    }
}