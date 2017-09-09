
use std::sync::Arc;
use std::sync::Weak;

use Error;
use Connector;
use Polynomial;

use net::Packet;
use net::BinaryReader;
use net::BinaryWriter;

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
    connector:      Weak<Connector>,
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
            connector: Weak::new(),
        }
    }
}