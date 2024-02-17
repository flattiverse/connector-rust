use crate::hierarchy::ClusterId;
use crate::network::PacketReader;
use crate::unit::{CelestialBody, Unit, UnitKind};
use crate::{NamedUnit, Vector};

#[derive(Debug)]
pub struct Moon {
    body: CelestialBody,
}

impl Moon {
    pub fn new(cluster: ClusterId, reader: &mut dyn PacketReader) -> Self {
        Self {
            body: CelestialBody::new(cluster, reader),
        }
    }

    // TODO pub async fn configure
    // TODO pub async fn remove
}

impl NamedUnit for Moon {
    #[inline]
    fn name(&self) -> &str {
        &self.body.name
    }
}

impl Unit for Moon {
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
        UnitKind::Moon
    }
}
