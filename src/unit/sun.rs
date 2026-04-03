use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// Stellar map unit that acts as a major energy intake source and environmental hazard.
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
    /// Energy cells can harvest this field.
    #[inline]
    pub fn energy(&self) -> f32 {
        self.energy.load()
    }

    /// Plasma wind emitted by this sun.
    /// Ion cells can harvest this field.
    #[inline]
    pub fn ions(&self) -> f32 {
        self.ions.load()
    }

    /// Neutrino radiation emitted by this sun. Neutrinos are not blocked by other celestial bodies.
    #[inline]
    pub fn neutrinos(&self) -> f32 {
        self.neutrinos.load()
    }

    /// Thermal radiation. Each point drains 15 energy per tick before any remaining overflow turns
    /// into radiation damage.
    #[inline]
    pub fn heat(&self) -> f32 {
        self.heat.load()
    }

    /// Ionizing radiation component. Each point causes 0.125 hull damage per tick after armor
    /// reduction.
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

impl UnitCastTable for Sun {
    cast_fn!(steady_unit_cast_fn, Sun, dyn SteadyUnit);
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

impl SteadyUnitInternal for Sun {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Sun {}
