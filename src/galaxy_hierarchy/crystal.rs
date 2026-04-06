use crate::galaxy_hierarchy::CrystalGrade;

/// One account-wide crystal.
#[derive(Debug, Clone)]
pub struct Crystal {
    pub(crate) name: String,
    pub(crate) hue: f32,
    pub(crate) grade: CrystalGrade,
    pub(crate) energy_battery_multiplier: f32,
    pub(crate) ions_battery_multiplier: f32,
    pub(crate) neutrinos_battery_multiplier: f32,
    pub(crate) hull_multiplier: f32,
    pub(crate) shield_multiplier: f32,
    pub(crate) armor_multiplier: f32,
    pub(crate) energy_cell_multiplier: f32,
    pub(crate) ions_cell_multiplier: f32,
    pub(crate) neutrinos_cell_multiplier: f32,
    pub(crate) shot_weapon_production_multiplier: f32,
    pub(crate) interceptor_weapon_production_multiplier: f32,
    pub(crate) crystal_cargo_limit_multiplier: f32,
    pub(crate) locked: bool,
}

impl Crystal {
    /// Crystal name within the owning account inventory.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Hue value carried by the crystal, currently derived from harvested nebula material.
    #[inline]
    pub fn hue(&self) -> f32 {
        self.hue
    }

    /// Quality grade of the crystal.
    #[inline]
    pub fn grade(&self) -> CrystalGrade {
        self.grade
    }

    /// Effect-axis multiplier for the energy-battery stat.
    #[inline]
    pub fn energy_battery_multiplier(&self) -> f32 {
        self.energy_battery_multiplier
    }

    /// Effect-axis multiplier for the ion-battery stat.
    #[inline]
    pub fn ions_battery_multiplier(&self) -> f32 {
        self.ions_battery_multiplier
    }

    /// Effect-axis multiplier for the neutrino-battery stat.
    #[inline]
    pub fn neutrinos_battery_multiplier(&self) -> f32 {
        self.neutrinos_battery_multiplier
    }

    /// Effect-axis multiplier for hull-related stats.
    #[inline]
    pub fn hull_multiplier(&self) -> f32 {
        self.hull_multiplier
    }

    /// Effect-axis multiplier for shield-related stats.
    #[inline]
    pub fn shield_multiplier(&self) -> f32 {
        self.shield_multiplier
    }

    /// Effect-axis multiplier for armor-related stats.
    #[inline]
    pub fn armor_multiplier(&self) -> f32 {
        self.armor_multiplier
    }

    /// Effect-axis multiplier for the energy-cell stat.
    #[inline]
    pub fn energy_cell_multiplier(&self) -> f32 {
        self.energy_cell_multiplier
    }

    /// Effect-axis multiplier for the ion-cell stat.
    #[inline]
    pub fn ions_cell_multiplier(&self) -> f32 {
        self.ions_cell_multiplier
    }

    /// Effect-axis multiplier for the neutrino-cell stat.
    #[inline]
    pub fn neutrinos_cell_multiplier(&self) -> f32 {
        self.neutrinos_cell_multiplier
    }

    /// Effect-axis multiplier for shot-weapon production.
    #[inline]
    pub fn shot_weapon_production_multiplier(&self) -> f32 {
        self.shot_weapon_production_multiplier
    }

    /// Effect-axis multiplier for interceptor-weapon production.
    #[inline]
    pub fn interceptor_weapon_production_multiplier(&self) -> f32 {
        self.interceptor_weapon_production_multiplier
    }

    /// Effect-axis multiplier for crystal-cargo capacity or efficiency.
    #[inline]
    pub fn crystal_cargo_limit_multiplier(&self) -> f32 {
        self.crystal_cargo_limit_multiplier
    }

    /// Whether rename and destroy operations are currently forbidden for this crystal.
    #[inline]
    pub fn locked(&self) -> bool {
        self.locked
    }
}
