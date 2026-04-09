use crate::network::PacketReader;
use crate::utils::{Atomic, Readable};
use crate::SubsystemStatus;

/// Visible snapshot of a cargo subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct CargoSubsystemInfo {
    exists: Atomic<bool>,
    maximum_metal: Atomic<f32>,
    maximum_carbon: Atomic<f32>,
    maximum_hydrogen: Atomic<f32>,
    maximum_silicon: Atomic<f32>,
    maximum_nebula: Atomic<f32>,
    current_metal: Atomic<f32>,
    current_carbon: Atomic<f32>,
    current_hydrogen: Atomic<f32>,
    current_silicon: Atomic<f32>,
    current_nebula: Atomic<f32>,
    nebula_hue: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
}

impl CargoSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Maximum metal capacity.
    #[inline]
    pub fn maximum_metal(&self) -> f32 {
        self.maximum_metal.load()
    }

    /// Maximum carbon capacity.
    #[inline]
    pub fn maximum_carbon(&self) -> f32 {
        self.maximum_carbon.load()
    }

    /// Maximum hydrogen capacity.
    #[inline]
    pub fn maximum_hydrogen(&self) -> f32 {
        self.maximum_hydrogen.load()
    }

    /// Maximum silicon capacity.
    #[inline]
    pub fn maximum_silicon(&self) -> f32 {
        self.maximum_silicon.load()
    }

    /// Maximum nebula capacity.
    #[inline]
    pub fn maximum_nebula(&self) -> f32 {
        self.maximum_nebula.load()
    }

    /// Current stored metal.
    #[inline]
    pub fn current_metal(&self) -> f32 {
        self.current_metal.load()
    }

    /// Current stored carbon.
    #[inline]
    pub fn current_carbon(&self) -> f32 {
        self.current_carbon.load()
    }

    /// Current stored hydrogen.
    #[inline]
    pub fn current_hydrogen(&self) -> f32 {
        self.current_hydrogen.load()
    }

    /// Current stored silicon.
    #[inline]
    pub fn current_silicon(&self) -> f32 {
        self.current_silicon.load()
    }

    /// Current stored nebula material.
    /// Nebula cargo is tracked separately from the normal metal, carbon, hydrogen, and silicon
    /// stores.
    #[inline]
    pub fn current_nebula(&self) -> f32 {
        self.current_nebula.load()
    }

    /// Average hue of the nebula material currently stored in cargo.
    #[inline]
    pub fn nebula_hue(&self) -> f32 {
        self.nebula_hue.load()
    }

    /// Tick-local runtime status reported for the cargo subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    pub(crate) fn update_from_reader(&self, reader: &mut dyn PacketReader) {
        if reader.read_byte() != 0x00 {
            self.update(
                true,
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
        } else {
            self.update(
                false,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                SubsystemStatus::Off,
            );
        }
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum_metal: f32,
        maximum_carbon: f32,
        maximum_hydrogen: f32,
        maximum_silicon: f32,
        maximum_nebula: f32,
        current_metal: f32,
        current_carbon: f32,
        current_hydrogen: f32,
        current_silicon: f32,
        current_nebula: f32,
        nebula_hue: f32,
        status: SubsystemStatus,
    ) {
        self.exists.store(exists);
        self.maximum_metal.store(maximum_metal);
        self.maximum_carbon.store(maximum_carbon);
        self.maximum_hydrogen.store(maximum_hydrogen);
        self.maximum_silicon.store(maximum_silicon);
        self.maximum_nebula.store(maximum_nebula);
        self.current_metal.store(current_metal);
        self.current_carbon.store(current_carbon);
        self.current_hydrogen.store(current_hydrogen);
        self.current_silicon.store(current_silicon);
        self.current_nebula.store(current_nebula);
        self.nebula_hue.store(nebula_hue);
        self.status.store(status);
    }
}
