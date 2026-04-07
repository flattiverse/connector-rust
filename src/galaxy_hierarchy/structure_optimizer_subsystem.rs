use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::SubsystemSlot;
use std::sync::Weak;

/// Passive structure-optimizer subsystem of a controllable.
#[derive(Debug)]
pub struct StructureOptimizerSubsystem {
    base: SubsystemBase,
    reduction_percentage: Atomic<f32>,
}

impl StructureOptimizerSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        exists: bool,
        reduction_percentage: f32,
    ) -> Self {
        Self {
            base: SubsystemBase::new(
                controllable,
                "StructureOptimizer".to_string(),
                exists,
                SubsystemSlot::StructureOptimizer,
            ),
            reduction_percentage: Atomic::from(reduction_percentage),
        }
    }

    /// Percentage of raw structure load reduced by this subsystem.
    #[inline]
    pub fn reduction_percentage(&self) -> f32 {
        self.reduction_percentage.load()
    }

    pub(crate) fn set_reduction_percentage(&self, reduction_percentage: f32) {
        self.reduction_percentage.store(if self.exists() {
            reduction_percentage
        } else {
            0.0
        });

        // TODO self.refresh_tier();
    }

    // TODO pub(crate) fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for StructureOptimizerSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
