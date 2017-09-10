
use std::sync::Arc;

use Error;
use Connector;
use Polynomial;

use controllable::Controllable;

use net::BinaryReader;

#[derive(Clone)]
pub struct EnergyCost {
    cost_energy:            Polynomial,
    cost_particles:         Polynomial,
    cost_ions:              Polynomial,
    extended_cost_energy:   Polynomial,
    extended_cost_particles:Polynomial,
    extended_cost_ions:     Polynomial,

    limit:          f32,
    extended_limit: f32,
}

impl EnergyCost {
    pub fn new(limit: f32, extended_limit: f32, cost_energy: Polynomial, cost_particles: Polynomial, cost_ions: Polynomial,
               extended_cost_energy: Polynomial, extended_cost_particles: Polynomial, extended_cost_ions: Polynomial) -> EnergyCost {
        EnergyCost {
            limit,
            extended_limit,
            cost_energy,
            cost_particles,
            cost_ions,
            extended_cost_energy,
            extended_cost_particles,
            extended_cost_ions,
        }
    }

    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader) -> Result<EnergyCost, Error> {
        Ok(EnergyCost {
            limit:          reader.read_single()?,
            extended_limit: reader.read_single()?,

            cost_energy:    Polynomial::from_reader(connector, reader)?,
            cost_particles: Polynomial::from_reader(connector, reader)?,
            cost_ions:      Polynomial::from_reader(connector, reader)?,

            extended_cost_energy:   Polynomial::from_reader(connector, reader)?,
            extended_cost_particles:Polynomial::from_reader(connector, reader)?,
            extended_cost_ions:     Polynomial::from_reader(connector, reader)?,
        })
    }

    pub fn calculate_energy_usage(&self, value: f32, energy: &mut f32, particles: &mut f32, ions: &mut f32, controllable: &Controllable) -> Result<bool, Error> {
        if !value.is_finite() {
            return Err(Error::InvalidValue(value))
        }

        if value < 0f32 || value > self.extended_limit * 1.000_000_06_f32 {
            return Err(Error::InvalidValue(value));
        }

        if value <= self.limit {
            *energy     = self.cost_energy   .value(value);
            *particles  = self.cost_particles.value(value);
            *ions       = self.cost_ions     .value(value);

        } else {
            *energy     = self.extended_cost_energy     .value(value);
            *particles  = self.extended_cost_particles  .value(value);
            *ions       = self.extended_cost_ions       .value(value);
        }

        Ok(
            *energy     <= controllable.energy() &&
            *particles  <= controllable.particles() &&
            *ions       <= controllable.ions()
        )
    }

    pub fn cost_energy(&self, value: f32) -> f32 {
        self.cost_energy.value(value)
    }

    pub fn cost_energy_polynomial(&self) -> &Polynomial {
        &self.cost_energy
    }

    pub fn cost_particles(&self, value: f32) -> f32 {
        self.cost_particles.value(value)
    }

    pub fn cost_particles_polynomial(&self) -> &Polynomial {
        &self.cost_particles
    }

    pub fn cost_ions(&self, value: f32) -> f32 {
        self.cost_ions.value(value)
    }

    pub fn cost_ions_polynomial(&self) -> &Polynomial {
        &self.cost_ions
    }

    pub fn extended_cost_energy(&self, value: f32) -> f32 {
        self.extended_cost_energy.value(value)
    }

    pub fn extended_cost_energy_polynomial(&self) -> &Polynomial {
        &self.extended_cost_energy
    }

    pub fn extended_cost_particles(&self, value: f32) -> f32 {
        self.extended_cost_particles.value(value)
    }

    pub fn extended_cost_particles_polynomial(&self) -> &Polynomial {
        &self.extended_cost_particles
    }

    pub fn extended_cost_ions(&self, value: f32) -> f32 {
        self.extended_cost_ions.value(value)
    }

    pub fn extended_cost_ions_polynomial(&self) -> &Polynomial {
        &self.extended_cost_ions
    }

    pub fn limit(&self) -> f32 {
        self.limit
    }

    pub fn extended_limit(&self) -> f32 {
        self.extended_limit
    }
}