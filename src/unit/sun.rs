use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase};
use crate::utils::Readable;
use std::sync::Weak;

/// A sun.
#[derive(Debug, Clone)]
pub struct Sun {
    base: UnitBase,
    steady: SteadyUnit,
    energy: f32,
    ions: f32,
    neutrinos: f32,
    heat: f32,
    drain: f32,
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
            energy: reader.read_f32(),
            ions: reader.read_f32(),
            neutrinos: reader.read_f32(),
            heat: reader.read_f32(),
            drain: reader.read_f32(),
        }
    }

    /// Photon flux emitted by this sun.
    #[inline]
    pub fn energy(&self) -> f32 {
        self.energy
    }

    /// Plasma wind emitted by this sun.
    #[inline]
    pub fn ions(&self) -> f32 {
        self.ions
    }

    /// Neutrino radiation emitted by this sun. Neutrinos are not blocked by other celestial bodies.
    #[inline]
    pub fn neutrinos(&self) -> f32 {
        self.neutrinos
    }

    /// Thermal radiation. Heat raises energy costs.
    #[inline]
    pub fn heat(&self) -> f32 {
        self.heat
    }

    /// Shield-drain radiation. Drain loads and slowly discharges shields.
    #[inline]
    pub fn drain(&self) -> f32 {
        self.drain
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
