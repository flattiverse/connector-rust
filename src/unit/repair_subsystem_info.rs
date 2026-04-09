use crate::network::PacketReader;
use crate::utils::{Atomic, Readable};
use crate::SubsystemStatus;

/// Visible snapshot of a repair subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct RepairSubsystemInfo {
    exists: Atomic<bool>,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
    rate: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
    repaired_hull_this_tick: Atomic<f32>,
}

impl RepairSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Minimum configurable repair rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// Maximum configurable repair rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    /// Configured hull-repair rate for the reported tick.
    /// A rate of `0` means the repair subsystem is effectively off.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Tick-local runtime status reported for the repair subsystem.
    /// The repair subsystem only restores hull and can fail while the unit is moving too fast.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by the repair subsystem during the reported tick.
    /// The current server model uses a quadratic cost curve based on the configured rate.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by the repair subsystem during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by the repair subsystem during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Hull integrity restored during the reported tick.
    #[inline]
    pub fn repaired_hull_this_tick(&self) -> f32 {
        self.repaired_hull_this_tick.load()
    }

    pub(crate) fn update_from_reader(&self, reader: &mut dyn PacketReader) {
        if reader.read_byte() != 0x00 {
            self.update(
                true,
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
        } else {
            self.update(
                false,
                0.0,
                0.0,
                0.0,
                SubsystemStatus::Off,
                0.0,
                0.0,
                0.0,
                0.0,
            );
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update(
        &self,
        exists: bool,
        minimum_rate: f32,
        maximum_rate: f32,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
        repaired_hull_this_tick: f32,
    ) {
        self.exists.store(exists);
        self.minimum_rate.store(minimum_rate);
        self.maximum_rate.store(maximum_rate);
        self.rate.store(rate);
        self.status.store(status);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.repaired_hull_this_tick.store(repaired_hull_this_tick);
    }
}
