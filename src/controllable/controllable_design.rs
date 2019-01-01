

use crate::Error;
use crate::unit::UnitKind;
use crate::net::BinaryReader;


pub struct ControllableDesign {
    name:           String,
    level:          u8,
    revision:       i64,
    kind:           UnitKind,
    radius:         f32,
    gravity:        f32,
    energy_max:     f32,
    particles_max:  f32,
    ions_max:       f32,
    hull_max:       f32,
    hull_armor:     f32,
    shield_max:     f32,
    shield_armor:   f32,
    weapon_damage_hull:     f32,
    weapon_damage_shield:   f32,
    weapon_damage_energy:   f32,
    weapon_production:      f32,
    weapon_production_load: f32,
    scanner_count:          u8,
    scanner_degree_per_scan:f32,
    scanner_range:          f32,
    tractorbeam_range:      f32,
    energy_cells:           f32,
    particles_cells:        f32,
    ion_cells:              f32,
    energy_reactor:         f32,
    particles_reactor:      f32,
    ion_reactor:            f32,
    cargo_slots:            u8,
    crystal_slots:          u8,
    energy_transfer:        f32,
    build_capabilities:     Vec<UnitKind>,
}

impl ControllableDesign {

    pub fn from_reader(reader: &mut BinaryReader) -> Result<ControllableDesign, Error> {
        Ok(ControllableDesign {
            name:                   reader.read_string()?,
            revision:               reader.read_i64()?,
            kind:                   UnitKind::from_id(reader.read_byte()?),
            level:                  reader.read_unsigned_byte()?,
            radius:                 reader.read_single()?,
            gravity:                reader.read_single()?,
            energy_max:             reader.read_single()?,
            particles_max:          reader.read_single()?,
            ions_max:               reader.read_single()?,
            hull_max:               reader.read_single()?,
            hull_armor:             reader.read_single()?,
            shield_max:             reader.read_single()?,
            shield_armor:           reader.read_single()?,
            weapon_damage_hull:     reader.read_single()?,
            weapon_damage_shield:   reader.read_single()?,
            weapon_damage_energy:   reader.read_single()?,

            weapon_production:      reader.read_single()?,
            weapon_production_load: reader.read_single()?,
            scanner_count:          reader.read_byte()?,
            scanner_degree_per_scan:reader.read_single()?,
            scanner_range:          reader.read_single()?,
            tractorbeam_range:      reader.read_single()?,
            energy_cells:           reader.read_single()?,
            particles_cells:        reader.read_single()?,
            ion_cells:              reader.read_single()?,
            energy_reactor:         reader.read_single()?,
            particles_reactor:      reader.read_single()?,
            ion_reactor:            reader.read_single()?,
            cargo_slots:            reader.read_byte()?,
            crystal_slots:          reader.read_byte()?,
            energy_transfer:        reader.read_single()?,
            build_capabilities: {
                let count = reader.read_byte()?;
                let mut vec = Vec::new();
                for _ in 0..count {
                    vec.push(UnitKind::from_id(reader.read_byte()?));
                }
                vec
            }
        })
    }

    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn level(&self) -> u8 {
        self.level
    }
    
    
    pub fn revision(&self) -> i64 {
        self.revision
    }
    
    pub fn kind(&self) -> UnitKind {
        self.kind
    }
    
    pub fn radius(&self) -> f32 {
        self.radius
    }
    
    pub fn gravity(&self) -> f32 {
        self.gravity
    }
    
    pub fn energy_max(&self) -> f32 {
        self.energy_max
    }
    
    pub fn particles_max(&self) -> f32 {
        self.particles_max
    }
    
    pub fn ions_max(&self) -> f32 {
        self.ions_max
    }
    
    pub fn hull_max(&self) -> f32 {
        self.hull_max
    }
    
    pub fn hull_armor(&self) -> f32 {
        self.hull_armor
    }
    
    pub fn shield_max(&self) -> f32 {
        self.shield_max
    }
    
    pub fn shield_armor(&self) -> f32 {
        self.shield_armor
    }
    
    pub fn weapon_damage_hull(&self) -> f32 {
        self.weapon_damage_hull
    }
    
    pub fn weapon_damage_shield(&self) -> f32 {
        self.weapon_damage_shield
    }
    
    pub fn weapon_damage_energy(&self) -> f32 {
        self.weapon_damage_energy
    }
    
    pub fn weapon_production(&self) -> f32 {
        self.weapon_production
    }
    
    pub fn weapon_production_load(&self) -> f32 {
        self.weapon_production_load
    }
    
    pub fn scanner_count(&self) -> u8 {
        self.scanner_count
    }
    
    pub fn scanner_degree_per_scan(&self) -> f32 {
        self.scanner_degree_per_scan
    }
    
    pub fn scanner_range(&self) -> f32 {
        self.scanner_range
    }
    
    pub fn tractorbeam_range(&self) -> f32 {
        self.tractorbeam_range
    }
    
    pub fn energy_cells(&self) -> f32 {
        self.energy_cells
    }
    
    pub fn particles_cells(&self) -> f32 {
        self.particles_cells
    }
    
    pub fn ion_cells(&self) -> f32 {
        self.ion_cells
    }
    
    pub fn energy_produced_by_reactor(&self) -> f32 {
        self.energy_reactor
    }
    
    pub fn particles_produced_by_reactor(&self) -> f32 {
        self.particles_reactor
    }
    
    pub fn ions_produced_by_reactor(&self) -> f32 {
        self.ion_reactor
    }
    
    pub fn cargo_slots(&self) -> u8 {
        self.cargo_slots
    }
    
    pub fn crystal_slots(&self) -> u8 {
        self.crystal_slots
    }
    
    pub fn energy_transfer(&self) -> f32 {
        self.energy_transfer
    }
    
    pub fn build_capabilities(&self) -> &Vec<UnitKind> {
        &self.build_capabilities
    }

}