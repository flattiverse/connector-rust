use crate::network::PacketReader;
use crate::utils::{Atomic, Readable};
use crate::SubsystemStatus;

/// Visible snapshot of a battery subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct BatterySubsystemInfo {
    exists: Atomic<bool>,
    maximum: Atomic<f32>,
    current: Atomic<f32>,
    consumed_this_tick: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
}

impl BatterySubsystemInfo {
    /// Indicates whether the scanned unit actually has this battery subsystem installed.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Maximum storable amount in this battery.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// Currently stored amount in this battery.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// Amount consumed during the current server tick.
    #[inline]
    pub fn consumed_this_tick(&self) -> f32 {
        self.consumed_this_tick.load()
    }

    /// Tick-local runtime status reported for this battery.
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
                SubsystemStatus::read(reader),
            );
        } else {
            self.update(false, 0.0, 0.0, 0.0, SubsystemStatus::Off);
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update(
        &self,
        exists: bool,
        maximum: f32,
        current: f32,
        consumed_this_tick: f32,
        status: SubsystemStatus,
    ) {
        self.exists.store(exists);
        self.maximum.store(maximum);
        self.current.store(current);
        self.consumed_this_tick.store(consumed_this_tick);
        self.status.store(status);
    }
}
