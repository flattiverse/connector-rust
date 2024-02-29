use crate::hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::configurations::{BuoyConfiguration, SunConfiguration};
use crate::unit::{CelestialBody, Unit, UnitKind};
use crate::{GameError, Vector};
use arc_swap::ArcSwap;
use std::any::Any;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct Buoy {
    cluster: Arc<Cluster>,
    body: CelestialBody,
    message: ArcSwap<String>,
    beacons: ArcSwap<Vec<Vector>>,
}

impl Buoy {
    pub fn new(cluster: Arc<Cluster>, reader: &mut dyn PacketReader) -> Self {
        Self {
            cluster,
            body: CelestialBody::new(reader),
            message: ArcSwap::new(Arc::new(reader.read_string())),
            beacons: ArcSwap::new(Arc::new(
                (0..reader.read_byte())
                    .map(|_| Vector::default().with_read(reader))
                    .collect(),
            )),
        }
    }

    /// Requests the current configuration of this unit from the server.
    /// See also [`ConnectionHandle::retrieve_unit_configuration`].
    pub async fn retrieve_configuration(&self) -> Result<BuoyConfiguration, GameError> {
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

    /// Removes this unit.
    /// See also [`ConnectionHandle::remove_unit`].
    pub async fn remove(&self) -> Result<(), GameError> {
        self.cluster
            .connection()?
            .remove_unit(self.cluster.id(), self.name(), self.kind())
            .await
    }

    /// The message of this [`Buoy`].
    #[inline]
    pub fn message(&self) -> impl Deref<Target = Arc<String>> {
        self.message.load()
    }

    /// Beacons of this [`Buoy`]. Beacons are locations relative to a [`Buoy`] for which the space
    /// in between might be of interest.
    #[inline]
    pub fn beacons(&self) -> impl Deref<Target = Arc<Vec<Vector>>> {
        self.beacons.load()
    }
}

impl Unit for Buoy {
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

    #[inline]
    fn update(&self, reader: &mut dyn PacketReader) {
        self.body.update(reader);

        self.message.store(Arc::new(reader.read_string()));
        self.beacons.store(Arc::new(
            (0..reader.read_byte())
                .map(|_| Vector::default().with_read(reader))
                .collect(),
        ));
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Buoy
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        &*self
    }
}
