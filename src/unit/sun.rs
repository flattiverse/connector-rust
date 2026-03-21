use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Atomic, Readable};
use std::sync::Weak;

/// A sun.
#[derive(Debug, Clone)]
pub struct Sun {
    base: UnitBase,
    steady: SteadyUnit,
    energy: Atomic<f32>,
    ions: Atomic<f32>,
    neutrinos: Atomic<f32>,
    heat: Atomic<f32>,
    drain: Atomic<f32>,
}

impl Sun {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster, name),
            steady: SteadyUnit::read(reader),
            energy: Atomic::default(),
            ions: Atomic::default(),
            neutrinos: Atomic::default(),
            heat: Atomic::default(),
            drain: Atomic::default(),
        }
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

impl AsRef<UnitBase> for Sun {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for Sun {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}

impl<'a> UnitExtSealed<'a> for &'a Sun {
    type Parent = (&'a UnitBase, &'a SteadyUnit);

    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady)
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.energy.read(reader);
        self.ions.read(reader);
        self.neutrinos.read(reader);
        self.heat.read(reader);
        self.drain.read(reader);
    }
}

impl<'a> UnitExt<'a> for &'a Sun {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Sun
    }
}
