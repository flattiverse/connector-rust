use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::runtime::Atomic;
use crate::Vector;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct UnitBase {
    pub name: String,
    radius: Atomic<f32>,
    position: Atomic<Vector>,
    cluster: Weak<Cluster>,
}

impl UnitBase {
    pub(crate) fn from_packet(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            cluster,
            name,
            radius: Atomic::from_reader(reader),
            position: Atomic::from_reader(reader),
        }
    }

    /// The name of the unit. A unit can't change her name after it has been set up.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The radius of the unit.
    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius.load()
    }

    /// The position of the unit.
    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }

    /// The cluster the unit is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.upgrade().unwrap()
    }
}
