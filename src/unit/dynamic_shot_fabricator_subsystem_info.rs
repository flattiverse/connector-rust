use crate::network::PacketReader;
use crate::utils::{Atomic, Readable};
use crate::SubsystemStatus;

/// Visible snapshot of a dynamic shot fabricator subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct DynamicShotFabricatorSubsystemInfo {
    exists: Atomic<bool>,
    maximum_rate: Atomic<f32>,
    active: Atomic<bool>,
    rate: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicShotFabricatorSubsystemInfo {
    const MINIMUM_RATE_VALUE: f32 = 0.0;

    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The minimum configurable shot fabrication rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        Self::MINIMUM_RATE_VALUE
    }

    /// The maximum configurable shot fabrication rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    /// Whether the fabricator was active during the reported tick.
    /// This is separate from [`Self::rate()`] because a non-zero configured rate can still be
    /// inactive.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// Configured shot fabrication rate.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Tick-local runtime status reported for the shot fabricator subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by fabrication during the reported tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by fabrication during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by fabrication during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update_from_reader(&self, reader: &mut dyn PacketReader) {
        if reader.read_byte() != 0x00 {
            self.update(
                true,
                reader.read_f32(),
                reader.read_byte() != 0x00,
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
        } else {
            self.update(false, 0.0, false, 0.0, SubsystemStatus::Off, 0.0, 0.0, 0.0);
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update(
        &self,
        exists: bool,
        maximum_rate: f32,
        active: bool,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        self.maximum_rate.store(maximum_rate);
        self.active.store(active);
        self.rate.store(rate);
        self.status.store(status);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
    }
}
