use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Cargo subsystem of a controllable.
#[derive(Debug)]
pub struct CargoSubsystem {
    base: SubsystemBase,

    maximum_metal: Atomic<f32>,
    maximum_carbon: Atomic<f32>,
    maximum_hydrogen: Atomic<f32>,
    maximum_silicon: Atomic<f32>,

    maximum_nebula: Atomic<f32>,

    current_metal: Atomic<f32>,
    current_carbon: Atomic<f32>,
    current_hydrogen: Atomic<f32>,
    current_silicon: Atomic<f32>,
    current_nebula: Atomic<f32>,
    nebula_hue: Atomic<f32>,
}

impl CargoSubsystem {
    const CLASSIC_SHIP_MAXIMUM_METAL: f32 = 250.0;
    const CLASSIC_SHIP_MAXIMUM_CARBON: f32 = 12.0;
    const CLASSIC_SHIP_MAXIMUM_HYDROGEN: f32 = 12.0;
    const CLASSIC_SHIP_MAXIMUM_SILICON: f32 = 12.0;
    const CLASSIC_SHIP_MAXIMUM_NEBULA: f32 = 16.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        exists: bool,
        maximum_metal: f32,
        maximum_carbon: f32,
        maximum_hydrogen: f32,
        maximum_silicon: f32,
        maximum_nebula: f32,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, "Cargo".to_string(), exists, slot),
            maximum_metal: Atomic::from(if exists { maximum_metal } else { 0.0 }),
            maximum_carbon: Atomic::from(if exists { maximum_carbon } else { 0.0 }),
            maximum_hydrogen: Atomic::from(if exists { maximum_hydrogen } else { 0.0 }),
            maximum_silicon: Atomic::from(if exists { maximum_silicon } else { 0.0 }),
            maximum_nebula: Atomic::from(if exists { maximum_nebula } else { 0.0 }),
            current_metal: Atomic::default(),
            current_carbon: Atomic::default(),
            current_hydrogen: Atomic::default(),
            current_silicon: Atomic::default(),
            current_nebula: Atomic::default(),
            nebula_hue: Atomic::default(),
        }
    }

    #[inline]
    pub(crate) fn create_classic_ship_cargo(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            true,
            Self::CLASSIC_SHIP_MAXIMUM_METAL,
            Self::CLASSIC_SHIP_MAXIMUM_CARBON,
            Self::CLASSIC_SHIP_MAXIMUM_HYDROGEN,
            Self::CLASSIC_SHIP_MAXIMUM_SILICON,
            Self::CLASSIC_SHIP_MAXIMUM_NEBULA,
            SubsystemSlot::Cargo,
        )
    }

    /// Maximum metal capacity.
    #[inline]
    pub fn maximum_metal(&self) -> f32 {
        self.maximum_metal.load()
    }

    /// Maximum carbon capacity.
    #[inline]
    pub fn maximum_carbon(&self) -> f32 {
        self.maximum_carbon.load()
    }

    /// Maximum hydrogen capacity.
    #[inline]
    pub fn maximum_hydrogen(&self) -> f32 {
        self.maximum_hydrogen.load()
    }

    /// Maximum silicon capacity.
    #[inline]
    pub fn maximum_silicon(&self) -> f32 {
        self.maximum_silicon.load()
    }

    pub(crate) fn set_maximums(
        &self,
        maximum_metal: f32,
        maximum_carbon: f32,
        maximum_hydrogen: f32,
        maximum_silicon: f32,
        maximum_nebula: f32,
    ) {
        if self.exists() {
            self.maximum_metal.store(maximum_metal);
            self.maximum_carbon.store(maximum_carbon);
            self.maximum_hydrogen.store(maximum_hydrogen);
            self.maximum_silicon.store(maximum_silicon);
            self.maximum_nebula.store(maximum_nebula);
        } else {
            self.maximum_metal.store(0.0);
            self.maximum_carbon.store(0.0);
            self.maximum_hydrogen.store(0.0);
            self.maximum_silicon.store(0.0);
            self.maximum_nebula.store(0.0);
        }

        // TODO self.refresh_tier();

        if self.current_metal() > maximum_metal {
            self.current_metal.store(maximum_metal);
        }

        if self.current_carbon() > maximum_carbon {
            self.current_carbon.store(maximum_carbon);
        }

        if self.current_hydrogen() > maximum_hydrogen {
            self.current_hydrogen.store(maximum_hydrogen);
        }

        if self.current_silicon() > maximum_silicon {
            self.current_silicon.store(maximum_silicon);
        }

        if self.current_nebula() > maximum_nebula {
            self.current_nebula.store(maximum_nebula);
        }
    }

    /// Maximum nebula capacity.
    #[inline]
    pub fn maximum_nebula(&self) -> f32 {
        self.maximum_nebula.load()
    }

    /// Current stored metal.
    #[inline]
    pub fn current_metal(&self) -> f32 {
        self.current_metal.load()
    }

    /// Current stored carbon.
    #[inline]
    pub fn current_carbon(&self) -> f32 {
        self.current_carbon.load()
    }

    /// Current stored hydrogen.
    #[inline]
    pub fn current_hydrogen(&self) -> f32 {
        self.current_hydrogen.load()
    }

    /// Current stored silicon.
    #[inline]
    pub fn current_silicon(&self) -> f32 {
        self.current_silicon.load()
    }

    /// Current stored nebula.
    #[inline]
    pub fn current_nebula(&self) -> f32 {
        self.current_nebula.load()
    }

    /// Average hue of the stored nebula.
    #[inline]
    pub fn nebula_hue(&self) -> f32 {
        self.nebula_hue.load()
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn reset_runtime(&self) {
        self.current_metal.store_default();
        self.current_carbon.store_default();
        self.current_hydrogen.store_default();
        self.current_silicon.store_default();
        self.current_nebula.store_default();
        self.nebula_hue.store_default();
        self.base.reset_runtime_status();
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_runtime(
        &self,
        current_metal: f32,
        current_carbon: f32,
        current_hydrogen: f32,
        current_silicon: f32,
        current_nebula: f32,
        nebula_hue: f32,
        status: SubsystemStatus,
    ) {
        self.current_metal.store(current_metal);
        self.current_carbon.store(current_carbon);
        self.current_hydrogen.store(current_hydrogen);
        self.current_silicon.store(current_silicon);
        self.current_nebula.store(current_nebula);
        self.nebula_hue.store(nebula_hue);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::CargoSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    current_metal: self.current_metal(),
                    current_carbon: self.current_carbon(),
                    current_hydrogen: self.current_hydrogen(),
                    current_silicon: self.current_silicon(),
                    current_nebula: self.current_nebula(),
                    nebula_hue: self.nebula_hue(),
                }
                .into(),
            )
        }
    }

    // TODO pub fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for CargoSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
