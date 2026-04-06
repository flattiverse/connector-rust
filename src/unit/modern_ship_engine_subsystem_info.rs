use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a modern-ship engine subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ModernShipSubsystemInfo {
    exists: Atomic<bool>,
    maximum_forward_thrust: Atomic<f32>,
    maximum_reverse_thrust: Atomic<f32>,
    maximum_thrust_change_per_tick: Atomic<f32>,
    current_thrust: Atomic<f32>,
    target_thrust: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ModernShipSubsystemInfo {
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    #[inline]
    pub fn maximum_forward_thrust(&self) -> f32 {
        self.maximum_forward_thrust.load()
    }

    #[inline]
    pub fn maximum_reverse_thrust(&self) -> f32 {
        self.maximum_reverse_thrust.load()
    }

    #[inline]
    pub fn maximum_thrust_change_per_tick(&self) -> f32 {
        self.maximum_thrust_change_per_tick.load()
    }

    #[inline]
    pub fn current_thrust(&self) -> f32 {
        self.current_thrust.load()
    }

    #[inline]
    pub fn target_thrust(&self) -> f32 {
        self.target_thrust.load()
    }

    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum_forward_thrust: f32,
        maximum_reverse_thrust: f32,
        maximum_thrust_change_per_tick: f32,
        current_thrust: f32,
        target_thrust: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.maximum_forward_thrust.store(maximum_forward_thrust);
            self.maximum_reverse_thrust.store(maximum_reverse_thrust);
            self.maximum_thrust_change_per_tick
                .store(maximum_thrust_change_per_tick);
            self.current_thrust.store(current_thrust);
            self.target_thrust.store(target_thrust);
            self.status.store(status);
            self.consumed_energy_this_tick
                .store(consumed_energy_this_tick);
            self.consumed_ions_this_tick.store(consumed_ions_this_tick);
            self.consumed_neutrinos_this_tick
                .store(consumed_neutrinos_this_tick);
        } else {
            self.maximum_forward_thrust.store(0.0);
            self.maximum_reverse_thrust.store(0.0);
            self.maximum_thrust_change_per_tick.store(0.0);
            self.current_thrust.store(0.0);
            self.target_thrust.store(0.0);
            self.status.store(SubsystemStatus::Off);
            self.consumed_energy_this_tick.store(0.0);
            self.consumed_ions_this_tick.store(0.0);
            self.consumed_neutrinos_this_tick.store(0.0);
        }
    }
}
