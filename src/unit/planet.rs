use crate::hierarchy::ClusterId;
use crate::network::PacketReader;
use crate::unit::sub_components::HarvestableSection;
use crate::unit::{CelestialBody, Harvestable, Unit, UnitKind};
use crate::{NamedUnit, Vector};

#[derive(Debug)]
pub struct Planet {
    body: CelestialBody,
    harvestable: Harvestable,
}

impl Planet {
    pub fn new(cluster: ClusterId, reader: &mut dyn PacketReader) -> Self {
        Self {
            body: CelestialBody::new(cluster, reader),
            harvestable: Harvestable::new(reader),
        }
    }

    // TODO pub async fn configure
    // TODO pub async fn remove

    #[inline]
    pub fn harvestable_sections(&self) -> &[HarvestableSection] {
        &self.harvestable.sections
    }
}

impl NamedUnit for Planet {
    #[inline]
    fn name(&self) -> &str {
        &self.body.name
    }
}

impl Unit for Planet {
    #[inline]
    fn cluster(&self) -> ClusterId {
        self.body.cluster
    }

    #[inline]
    fn position(&self) -> Vector {
        self.body.position
    }

    #[inline]
    fn gravity(&self) -> f64 {
        self.body.gravity
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.body.radius
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Planet
    }
}
