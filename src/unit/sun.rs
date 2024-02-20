use crate::hierarchy::ClusterId;
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::sub_components::SunSection;
use crate::unit::{CelestialBody, Unit, UnitKind};
use crate::{GameError, Vector};
use std::future::Future;

#[derive(Debug)]
pub struct Sun {
    body: CelestialBody,
    sections: Vec<SunSection>,
    connection: ConnectionHandle,
}

impl Sun {
    pub fn new(
        cluster: ClusterId,
        reader: &mut dyn PacketReader,
        connection: ConnectionHandle,
    ) -> Self {
        Self {
            body: CelestialBody::new(cluster, reader),
            sections: (0..reader.read_byte())
                .map(|_| SunSection::default().with_read(reader))
                .collect(),
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

    #[inline]
    pub fn sections(&self) -> &[SunSection] {
        &self.sections
    }
}

impl Unit for Sun {
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
        self.sections = (0..reader.read_byte())
            .map(|_| SunSection::default().with_read(reader))
            .collect();
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Sun
    }
}
