use crate::network::PacketReader;
use crate::utils::{Atomic, Readable};
use crate::SubsystemStatus;

/// Visible snapshot of a hull subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct HullSubsystemInfo {
    exists: Atomic<bool>,
    maximum: Atomic<f32>,
    current: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
}

impl HullSubsystemInfo {
    /// Whether the subsystem exists.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The maximum hull integrity.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// The current hull integrity.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// The status reported for the current server tick.
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
                SubsystemStatus::read(reader),
            );
        } else {
            self.update(false, 0.0, 0.0, SubsystemStatus::Off)
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update(&self, exists: bool, maximum: f32, current: f32, status: SubsystemStatus) {
        self.exists.store(exists);
        self.maximum.store(maximum);
        self.current.store(current);
        self.status.store(status);
    }
}
