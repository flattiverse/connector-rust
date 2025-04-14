use crate::galaxy_hierarchy::{Cluster, Indexer};
use crate::network::PacketReader;
use crate::utils::Atomic;
use crate::Vector;
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ControllableId(pub(crate) u8);

impl Indexer for ControllableId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct ControllableBase {
    name: String,
    id: ControllableId,
    cluster: Weak<Cluster>,
    active: Atomic<bool>,
    alive: Atomic<bool>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
}

impl ControllableBase {
    pub(crate) fn new(
        id: ControllableId,
        name: String,
        cluster: Weak<Cluster>,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id,
            name,
            cluster,
            active: Atomic::from(true),
            alive: Atomic::from(false),
            position: Atomic::from_reader(reader),
            movement: Atomic::from_reader(reader),
        }
    }

    /// The id of the controllable.
    #[inline]
    pub fn id(&self) -> ControllableId {
        self.id
    }

    /// The name of the controllable.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The cluster this unit currently is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.upgrade().unwrap()
    }

    /// The position of the unit.
    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }

    /// The movement of the unit.
    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement.load()
    }

    /// true, if the unit is alive.
    #[inline]
    pub fn alive(&self) -> bool {
        self.alive.load()
    }

    /// true, if this objet still can be used. If the unit has been disposed this is false.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    pub(crate) fn deceased(&self) {
        self.alive.store(false);
        self.position.store(Default::default());
        self.movement.store(Default::default());
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);
        self.alive.store(true);
    }
}
