use crate::network::PacketReader;
use crate::utils::Atomic;

/// Visible snapshot of a jump-drive subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct JumpDriveSubsystemInfo {
    exists: Atomic<bool>,
    energy_cost: Atomic<f32>,
}

impl JumpDriveSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Energy required for a single jump activation.
    /// The actual destination depends on the worm hole being used, not on the subsystem itself.
    #[inline]
    pub fn energy_cost(&self) -> f32 {
        self.energy_cost.load()
    }

    pub(crate) fn update_from_reader(&self, reader: &mut dyn PacketReader) {
        if reader.read_byte() != 0x00 {
            self.update(true, reader.read_f32());
        } else {
            self.update(false, 0.0);
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update(&self, exists: bool, energy_cost: f32) {
        self.exists.store(exists);
        self.energy_cost.store(energy_cost);
    }
}
