use crate::hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::configurations::SunConfiguration;
use crate::unit::sub_components::SunSection;
use crate::unit::{CelestialBody, Unit, UnitKind};
use crate::{GameError, Vector};
use arc_swap::ArcSwap;
use std::any::Any;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sun {
    cluster: Arc<Cluster>,
    body: CelestialBody,
    sections: ArcSwap<Vec<SunSection>>,
}

impl Sun {
    pub fn new(cluster: Arc<Cluster>, reader: &mut dyn PacketReader) -> Self {
        Self {
            cluster,
            body: CelestialBody::new(reader),
            sections: ArcSwap::new(Arc::new(
                (0..reader.read_byte())
                    .map(|_| SunSection::default().with_read(reader))
                    .collect(),
            )),
        }
    }

    /// Requests the current configuration of this unit from the server.
    /// See also [`ConnectionHandle::retrieve_unit_configuration`].
    pub async fn retrieve_configuration(&self) -> Result<SunConfiguration, GameError> {
        self.cluster
            .connection()?
            .retrieve_unit_configuration(self.cluster.id(), self.name(), self.kind())
            .await
    }

    /// Requests the server to apply the given configuration onto this unit.
    /// See also [`ConnectionHandle::configure_unit`].
    pub async fn configure(&self, configuration: &SunConfiguration) -> Result<(), GameError> {
        self.cluster
            .connection()?
            .configure_unit(self.cluster.id(), self.name(), configuration)
            .await
    }

    /// Requests  the server to remove this unit.
    pub async fn remove(&self) -> Result<(), GameError> {
        self.cluster
            .connection()?
            .remove_unit(self.cluster.id(), self.name(), self.kind())
            .await
    }

    #[inline]
    pub fn sections(&self) -> impl Deref<Target = Arc<Vec<SunSection>>> {
        self.sections.load()
    }
}

impl Unit for Sun {
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
        self.sections.store(Arc::new(
            (0..reader.read_byte())
                .map(|_| SunSection::default().with_read(reader))
                .collect(),
        ));
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Sun
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        &*self
    }
}
