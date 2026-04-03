use crate::galaxy_hierarchy::{Cluster, ClusterId};
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use arc_swap::ArcSwapWeak;
use std::sync::{Arc, Weak};

/// A worm hole with a visible jump target after full disclosure.
#[derive(Debug)]
pub struct WormHole {
    parent: AbstractSteadyUnit,
    target_cluster: ArcSwapWeak<Cluster>,
    target_left: Atomic<f32>,
    target_top: Atomic<f32>,
    target_right: Atomic<f32>,
    target_bottom: Atomic<f32>,
}

impl Clone for WormHole {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            target_cluster: ArcSwapWeak::new(self.target_cluster.load_full()),
            target_left: self.target_left.clone(),
            target_top: self.target_top.clone(),
            target_right: self.target_right.clone(),
            target_bottom: self.target_bottom.clone(),
        }
    }
}

impl WormHole {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            target_cluster: ArcSwapWeak::default(),
            target_left: Atomic::from(0.0),
            target_top: Atomic::from(0.0),
            target_right: Atomic::from(0.0),
            target_bottom: Atomic::from(0.0),
        }))
    }

    /// The cluster a jump leads to, once the full state is known.
    #[inline]
    pub fn target_cluster(&self) -> Weak<Cluster> {
        self.target_cluster.load_full()
    }

    /// Left boundary of the target region inside [`WormHole::target_cluster`].
    #[inline]
    pub fn target_left(&self) -> f32 {
        self.target_left.load()
    }

    /// Top boundary of the target region inside [`WormHole::target_cluster`].
    #[inline]
    pub fn target_top(&self) -> f32 {
        self.target_top.load()
    }

    /// Right boundary of the target region inside [`WormHole::target_cluster`].
    #[inline]
    pub fn target_right(&self) -> f32 {
        self.target_right.load()
    }

    /// Bottom boundary of the target region inside [`WormHole::target_cluster`].
    #[inline]
    pub fn target_bottom(&self) -> f32 {
        self.target_bottom.load()
    }
}

impl UnitInternal for WormHole {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.target_cluster.store(Arc::downgrade(
            &self
                .cluster()
                .galaxy()
                .get_cluster(ClusterId(reader.read_byte())),
        ));

        self.target_left.read(reader);
        self.target_top.read(reader);
        self.target_right.read(reader);
        self.target_bottom.read(reader);
    }
}

impl UnitCastTable for WormHole {
    cast_fn!(steady_unit_cast_fn, WormHole, dyn SteadyUnit);
}

impl UnitHierarchy for WormHole {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_worm_hole(&self) -> Option<&WormHole> {
        Some(self)
    }
}

impl Unit for WormHole {
    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn is_solid(&self) -> bool {
        false
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::WormHole
    }
}

impl SteadyUnitInternal for WormHole {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for WormHole {}
