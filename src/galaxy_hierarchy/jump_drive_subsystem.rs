use crate::galaxy_hierarchy::{Controllable, Cost, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{GameError, GameErrorKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Jump-drive subsystem of a controllable.
#[derive(Debug)]
pub struct JumpDriveSubsystem {
    base: SubsystemBase,
    energy_cost: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl JumpDriveSubsystem {
    pub(crate) fn new(controllable: Weak<Controllable>, exists: bool) -> Self {
        Self {
            base: SubsystemBase::new(
                controllable,
                "JumpDrive".to_string(),
                exists,
                SubsystemSlot::JumpDrive,
            ),
            energy_cost: Atomic::from(if exists { 6_000.0 } else { 0.0 }),
            consumed_energy_this_tick: Atomic::from(0.0),
            consumed_ions_this_tick: Atomic::from(0.0),
            consumed_neutrinos_this_tick: Atomic::from(0.0),
        }
    }

    /// Energy required for one jump.
    #[inline]
    pub fn energy_cost(&self) -> f32 {
        self.energy_cost.load()
    }

    /// Standard energy consumed by the jump drive during the current tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by the jump drive during the current tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by the jump drive during the current tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the current fixed jump cost.
    pub fn calculate_cost(&self) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            Some(Cost {
                energy: self.energy_cost(),
                ions: 0.0,
                neutrinos: 0.0,
            })
        }
    }

    /// Requests a worm-hole jump on the server.
    pub async fn jump(&self) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            controllable
                .cluster()
                .galaxy()
                .connection()
                .jump_drive_subsystem_jump(controllable.id())
                .await
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.consumed_energy_this_tick.store(0.0);
        self.consumed_ions_this_tick.store(0.0);
        self.consumed_neutrinos_this_tick.store(0.0);
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn set_energy_cost(&self, energy_cost: f32) {
        if self.exists() {
            self.energy_cost.store(energy_cost);
        } else {
            self.energy_cost.store(0.0)
        }

        // TODO self.refresh_tier();
    }

    // TODO pub fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for JumpDriveSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
