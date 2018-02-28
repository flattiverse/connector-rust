
use std::fmt;
use std::io::Result;

use net::BinaryReader;

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Corona {
    radius: f32,
    energy: f32,
    particles: f32,
    ions: f32
}

impl Corona {
    pub fn from_reader(reader: &mut BinaryReader) -> Result<Corona> {
        Ok(Corona {
            radius:     reader.read_single()?,
            energy:     reader.read_single()?,
            particles:  reader.read_single()?,
            ions:       reader.read_single()?
        })
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn particles(&self) -> f32 {
        self.particles
    }

    pub fn ions(&self) -> f32 {
        self.ions
    }
}

impl fmt::Display for Corona {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [E:{}, P:{}, I:{}]", self.radius, self.energy, self.particles, self.ions)
    }
}