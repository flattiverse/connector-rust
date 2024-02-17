use crate::hierarchy::ClusterId;
use crate::unit::configurations::SunConfiguration;
use crate::unit::sub_components::SunSection;
use crate::unit::{CelestialBody, Unit};
use crate::{NamedUnit, Vector};

#[derive(Debug)]
pub struct Sun {
    name: String,
    sections: Vec<SunSection>,
    position: Vector,
    radius: f64,
    graity: f64,
}

impl From<SunConfiguration> for Sun {
    fn from(configuration: SunConfiguration) -> Self {
        Self {
            name: configuration.base.base.name,
            sections: configuration.sections,
            position: configuration.base.position,
            radius: configuration.base.radius,
            graity: configuration.base.graity,
        }
    }
}

impl Sun {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn sections(&self) -> &[SunSection] {
        &self.sections
    }
}

impl NamedUnit for Sun {
    #[inline]
    fn name(&self) -> &str {
        Sun::name(self)
    }
}

impl Unit for Sun {
    #[inline]
    fn name(&self) -> &str {
        Sun::name(self)
    }

    #[inline]
    fn cluster(&self) -> ClusterId {
        todo!()
    }

    #[inline]
    fn position(&self) -> Vector {
        self.position
    }

    #[inline]
    fn gravity(&self) -> f64 {
        self.graity
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.radius
    }
}

impl CelestialBody for Sun {}
