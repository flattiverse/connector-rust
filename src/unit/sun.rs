use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// A sun.
#[derive(Debug, Clone)]
pub struct Sun {
    parent: AbstractSteadyUnit,
    energy: Atomic<f32>,
    ions: Atomic<f32>,
    neutrinos: Atomic<f32>,
    heat: Atomic<f32>,
    drain: Atomic<f32>,
}

impl Sun {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            energy: Atomic::default(),
            ions: Atomic::default(),
            neutrinos: Atomic::default(),
            heat: Atomic::default(),
            drain: Atomic::default(),
        }))
    }

    /// Photon flux emitted by this sun.
    #[inline]
    pub fn energy(&self) -> f32 {
        self.energy.load()
    }

    /// Plasma wind emitted by this sun.
    #[inline]
    pub fn ions(&self) -> f32 {
        self.ions.load()
    }

    /// Neutrino radiation emitted by this sun. Neutrinos are not blocked by other celestial bodies.
    #[inline]
    pub fn neutrinos(&self) -> f32 {
        self.neutrinos.load()
    }

    /// Thermal radiation. Heat raises energy costs.
    #[inline]
    pub fn heat(&self) -> f32 {
        self.heat.load()
    }

    /// Shield-drain radiation. Drain loads and slowly discharges shields.
    #[inline]
    pub fn drain(&self) -> f32 {
        self.drain.load()
    }
}

impl UnitInternal for Sun {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.energy.read(reader);
        self.ions.read(reader);
        self.neutrinos.read(reader);
        self.heat.read(reader);
        self.drain.read(reader);
    }
}

impl UnitHierarchy for Sun {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_sun(&self) -> Option<&Sun> {
        Some(self)
    }
}

impl Unit for Sun {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Sun
    }
}

impl SteadyUnitInternal for Sun {}
impl SteadyUnit for Sun {}
