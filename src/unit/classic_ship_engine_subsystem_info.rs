use crate::utils::Atomic;
use crate::{SubsystemStatus, Vector};

/// Visible snapshot of a classic-ship engine subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ClassicShipEngineSubsystemInfo {
    exists: Atomic<bool>,
    maximum: Atomic<f32>,
    current: Atomic<Vector>,
    target: Atomic<Vector>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ClassicShipEngineSubsystemInfo {
    /// Indicates whether the scanned unit actually has this engine subsystem installed.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Maximum configurable impulse length of the engine command.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// Current engine impulse applied by the server.
    /// This is the thrust vector, not the ship's world-space movement vector.
    #[inline]
    pub fn current(&self) -> Vector {
        self.current.load()
    }

    /// Target engine impulse currently configured on the server.
    #[inline]
    pub fn target(&self) -> Vector {
        self.target.load()
    }

    /// The status reported for the current server tick.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// The energy consumed during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum: f32,
        current: Vector,
        target: Vector,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.maximum.store(maximum);
            self.current.store(current);
            self.target.store(target);
            self.status.store(status);
            self.consumed_energy_this_tick
                .store(consumed_energy_this_tick);
            self.consumed_ions_this_tick.store(consumed_ions_this_tick);
            self.consumed_neutrinos_this_tick
                .store(consumed_neutrinos_this_tick);
        } else {
            self.maximum.store_default();
            self.current.store_default();
            self.target.store_default();
            self.status.store_default();
            self.consumed_energy_this_tick.store_default();
            self.consumed_ions_this_tick.store_default();
            self.consumed_neutrinos_this_tick.store_default();
        }
    }
}
