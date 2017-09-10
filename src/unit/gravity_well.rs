

use std::fmt;

use Error;
use net::BinaryReader;

#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub struct GravityWell {
    radius:   f32,
    movement: f32,
}

impl GravityWell {
    pub fn from_reader(reader: &mut BinaryReader) -> Result<GravityWell, Error> {
        GravityWell {
            radius:     reader.read_single()?,
            movement:   reader.read_single()?,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn gravity_movement(&self) -> f32 {
        self.movement
    }
}

impl fmt::Display for GravityWell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmg::Result {
        write!(f, "{} [G:{}]", self.radius, self.movement)
    }
}