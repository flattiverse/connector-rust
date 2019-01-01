
use std::sync::Arc;
use std::ops::Deref;

use crate::Error;
use crate::Connector;
use crate::Polynomial;
use crate::controllable::EnergyCost;
use crate::controllable::Controllable;

use crate::net::BinaryReader;

#[derive(Clone)]
pub struct ScanEnergyCost {
    energy_cost: EnergyCost
}

impl ScanEnergyCost {
    pub fn new(limit: f32, extended_limit: f32, cost_energy: Polynomial, cost_particles: Polynomial, cost_ions: Polynomial,
               extended_cost_energy: Polynomial, extended_cost_particles: Polynomial, extended_cost_ions: Polynomial) -> ScanEnergyCost {
        ScanEnergyCost {
            energy_cost: EnergyCost::new(
                limit,
                extended_limit,
                cost_energy,
                cost_particles,
                cost_ions,
                extended_cost_energy,
                extended_cost_particles,
                extended_cost_ions,
            )
        }
    }

    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader) -> Result<ScanEnergyCost, Error> {
        Ok(ScanEnergyCost {
            energy_cost: EnergyCost::from_reader(connector, reader)?
        })
    }

    pub fn calculate_energy_usage(&self, range: f32, degree: f32, energy: &mut f32, particles: &mut f32, ions: &mut f32, controllable: &Controllable) -> Result<bool, Error> {
        if !range.is_finite() {
            return Err(Error::InvalidValue(range))
        }

        if range < 0f32 || range > self.energy_cost.extended_limit() {
            return Err(Error::InvalidValue(range));
        }

        if degree < 2_f32 || degree > controllable.scanner_degree_per_scan() * 1.000_000_06_f32 {
            return Err(Error::InvalidValue(degree));
        }

        let surface = range * range * degree.to_radians();

        if range <= self.energy_cost.limit() {
            *energy     = self.energy_cost.cost_energy   (surface);
            *particles  = self.energy_cost.cost_particles(surface);
            *ions       = self.energy_cost.cost_ions     (surface);

        } else {
            *energy     = self.energy_cost.extended_cost_energy     (surface);
            *particles  = self.energy_cost.extended_cost_particles  (surface);
            *ions       = self.energy_cost.extended_cost_ions       (surface);
        }

        Ok(
            *energy     <= controllable.energy() &&
            *particles  <= controllable.particles() &&
            *ions       <= controllable.ions()
        )
    }
}

// ScanEnergyCost 'extends' EnergyCost - kinda
impl Deref for ScanEnergyCost {
    type Target = EnergyCost;

    fn deref(&self) -> &Self::Target {
        &self.energy_cost
    }
}