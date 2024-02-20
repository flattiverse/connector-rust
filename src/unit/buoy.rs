use crate::hierarchy::ClusterId;
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::{CelestialBody, Unit, UnitKind};
use crate::{GameError, Vector};
use std::future::Future;

#[derive(Debug)]
pub struct Buoy {
    body: CelestialBody,
    connection: ConnectionHandle,
}

impl Buoy {
    pub fn new(
        cluster: ClusterId,
        reader: &mut dyn PacketReader,
        connection: ConnectionHandle,
    ) -> Self {
        Self {
            body: CelestialBody::new(cluster, reader),
            connection,
        }
    }

    // TODO pub async fn configure

    /// Removes this unit.
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .remove_unit_split(self.body.cluster, self.name().to_string(), self.kind())
            .await
    }
}

impl Unit for Buoy {
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

    #[inline]
    fn update(&mut self, reader: &mut dyn PacketReader) {
        self.body.update(reader);
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Buoy
    }
}
