use crate::hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::configurations::PlanetConfiguration;
use crate::unit::sub_components::HarvestableSection;
use crate::unit::{CelestialBody, Harvestable, Unit, UnitKind};
use crate::{GameError, Vector};
use std::any::Any;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct Planet {
    cluster: Arc<Cluster>,
    body: CelestialBody,
    harvestable: Harvestable,
}

impl Planet {
    pub fn new(cluster: Arc<Cluster>, reader: &mut dyn PacketReader) -> Self {
        Self {
            cluster,
            body: CelestialBody::new(reader),
            harvestable: Harvestable::new(reader),
        }
    }

    /// Requests the current configuration of this unit from the server.
    /// See also [`ConnectionHandle::retrieve_unit_configuration`].
    pub async fn retrieve_configuration(&self) -> Result<PlanetConfiguration, GameError> {
        self.cluster
            .connection()?
            .retrieve_unit_configuration(self.cluster.id(), self.name(), self.kind())
            .await
    }

    /// Requests the server to apply the given configuration onto this unit.
    /// See also [`ConnectionHandle::configure_unit`].
    pub async fn configure(&self, configuration: &PlanetConfiguration) -> Result<(), GameError> {
        self.cluster
            .connection()?
            .configure_unit(self.cluster.id(), self.name(), configuration)
            .await
    }

    /// Removes this unit.
    /// See also [`ConnectionHandle::remove_unit`].
    pub async fn remove(&self) -> Result<(), GameError> {
        self.cluster
            .connection()?
            .remove_unit(self.cluster.id(), self.name(), self.kind())
            .await
    }

    #[inline]
    pub fn harvestable_sections(&self) -> impl Deref<Target = Arc<Vec<HarvestableSection>>> {
        self.harvestable.sections.load()
    }
}

impl Unit for Planet {
    #[inline]
    fn active(&self) -> bool {
        true
    }

    #[inline]
    fn name(&self) -> &str {
        &self.body.name
    }

    #[inline]
    fn cluster(&self) -> &Arc<Cluster> {
        &self.cluster
    }

    #[inline]
    fn position(&self) -> Vector {
        self.body.position.load()
    }

    #[inline]
    fn gravity(&self) -> f64 {
        self.body.gravity.load()
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.body.radius.load()
    }

    fn update(&self, reader: &mut dyn PacketReader) {
        self.body.update(reader);
        self.harvestable.update(reader);
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Planet
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        &*self
    }
}
