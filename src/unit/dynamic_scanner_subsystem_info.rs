use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a dynamic scanner subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct DynamicScannerSubsystemInfo {
    exists: Atomic<bool>,
    maximum_width: Atomic<f32>,
    maximum_length: Atomic<f32>,
    width_speed: Atomic<f32>,
    length_speed: Atomic<f32>,
    angle_speed: Atomic<f32>,
    active: Atomic<bool>,
    current_width: Atomic<f32>,
    current_length: Atomic<f32>,
    current_angle: Atomic<f32>,
    target_width: Atomic<f32>,
    target_length: Atomic<f32>,
    target_angle: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicScannerSubsystemInfo {
    /// Whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Maximum configured scan width reported for this subsystem.
    #[inline]
    pub fn maximum_width(&self) -> f32 {
        self.maximum_width.load()
    }

    /// Maximum configured scan length reported for this subsystem.
    #[inline]
    pub fn maximum_length(&self) -> f32 {
        self.maximum_length.load()
    }

    /// Width speed capability of the reported scanner.
    #[inline]
    pub fn width_speed(&self) -> f32 {
        self.width_speed.load()
    }

    /// Length speed capability of the reported scanner.
    #[inline]
    pub fn length_speed(&self) -> f32 {
        self.length_speed.load()
    }

    /// Angle speed capability of the reported scanner.
    #[inline]
    pub fn angle_speed(&self) -> f32 {
        self.angle_speed.load()
    }

    /// Whether the scanner was active during the reported tick.
    /// After switching the scanner off, the current geometry typically drops back to zero until it
    /// is activated again.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// Current scan width reported by the server for this tick.
    /// This is the live runtime value, not necessarily the requested target width.
    #[inline]
    pub fn current_width(&self) -> f32 {
        self.current_width.load()
    }

    /// Current scan length reported by the server for this tick.
    #[inline]
    pub fn current_length(&self) -> f32 {
        self.current_length.load()
    }

    /// Current absolute world-space scan center angle reported for this tick.
    #[inline]
    pub fn current_angle(&self) -> f32 {
        self.current_angle.load()
    }

    /// Target scan width currently requested on the server.
    #[inline]
    pub fn target_width(&self) -> f32 {
        self.target_width.load()
    }

    /// Target scan length currently requested on the server.
    #[inline]
    pub fn target_length(&self) -> f32 {
        self.target_length.load()
    }

    /// Target absolute world-space scan center angle currently requested on the server.
    #[inline]
    pub fn target_angle(&self) -> f32 {
        self.target_angle.load()
    }

    /// Tick-local runtime status reported for the scanner subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by scanning during the reported tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by scanning during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by scanning during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum_width: f32,
        maximum_length: f32,
        width_speed: f32,
        length_speed: f32,
        angle_speed: f32,
        active: bool,
        current_width: f32,
        current_length: f32,
        current_angle: f32,
        target_width: f32,
        target_length: f32,
        target_angle: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.maximum_width.store(maximum_width);
            self.maximum_length.store(maximum_length);
            self.width_speed.store(width_speed);
            self.length_speed.store(length_speed);
            self.angle_speed.store(angle_speed);
            self.active.store(active);
            self.current_width.store(current_width);
            self.current_length.store(current_length);
            self.current_angle.store(current_angle);
            self.target_width.store(target_width);
            self.target_length.store(target_length);
            self.target_angle.store(target_angle);
            self.status.store(status);
            self.consumed_energy_this_tick
                .store(consumed_energy_this_tick);
            self.consumed_ions_this_tick.store(consumed_ions_this_tick);
            self.consumed_neutrinos_this_tick
                .store(consumed_neutrinos_this_tick);
        } else {
            self.maximum_width.store_default();
            self.maximum_length.store_default();
            self.width_speed.store_default();
            self.length_speed.store_default();
            self.angle_speed.store_default();
            self.active.store_default();
            self.current_width.store_default();
            self.current_length.store_default();
            self.current_angle.store_default();
            self.target_width.store_default();
            self.target_length.store_default();
            self.target_angle.store_default();
            self.status.store_default();
            self.consumed_energy_this_tick.store_default();
            self.consumed_ions_this_tick.store_default();
            self.consumed_neutrinos_this_tick.store_default();
        }
    }
}
