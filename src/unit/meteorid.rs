use crate::hierarchy::ClusterId;
use crate::network::PacketReader;
use crate::unit::sub_components::HarvestableSection;
use crate::unit::{CelestialBody, Harvestable, Unit, UnitKind};
use crate::Vector;

#[derive(Debug)]
pub struct Meteoroid {
    body: CelestialBody,
    harvestable: Harvestable,
}

impl Meteoroid {
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

impl Unit for Meteoroid {
    #[inline]
    fn name(&self) -> &str {
        &self.body.name
    }

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

    fn update(&mut self, reader: &mut dyn PacketReader) {
        self.body.update(reader);
        self.harvestable.update(reader);
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Meteoroid
    }
}
