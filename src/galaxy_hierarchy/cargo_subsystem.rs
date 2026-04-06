use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Cargo subsystem of a controllable.
#[derive(Debug)]
pub struct CargoSubsystem {
    base: SubsystemBase,

    maximum_metal: f32,
    maximum_carbon: f32,
    maximum_hydrogen: f32,
    maximum_silicon: f32,

    maximum_nebula: Atomic<f32>,

    current_metal: Atomic<f32>,
    current_carbon: Atomic<f32>,
    current_hydrogen: Atomic<f32>,
    current_silicon: Atomic<f32>,
    current_nebula: Atomic<f32>,
    nebula_hue: Atomic<f32>,
}

impl CargoSubsystem {
    const CLASSIC_SHIP_MAXIMUM_METAL: f32 = 20.0;
    const CLASSIC_SHIP_MAXIMUM_CARBON: f32 = 20.0;
    const CLASSIC_SHIP_MAXIMUM_HYDROGEN: f32 = 20.0;
    const CLASSIC_SHIP_MAXIMUM_SILICON: f32 = 20.0;
    const CLASSIC_SHIP_MAXIMUM_NEBULA: f32 = 24.0;

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
            maximum_metal: if exists { maximum_metal } else { 0.0 },
            maximum_carbon: if exists { maximum_carbon } else { 0.0 },
            maximum_hydrogen: if exists { maximum_hydrogen } else { 0.0 },
            maximum_silicon: if exists { maximum_silicon } else { 0.0 },
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
        self.maximum_metal
    }

    /// Maximum carbon capacity.
    #[inline]
    pub fn maximum_carbon(&self) -> f32 {
        self.maximum_carbon
    }

    /// Maximum hydrogen capacity.
    #[inline]
    pub fn maximum_hydrogen(&self) -> f32 {
        self.maximum_hydrogen
    }

    /// Maximum silicon capacity.
    #[inline]
    pub fn maximum_silicon(&self) -> f32 {
        self.maximum_silicon
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

    pub(crate) fn set_maximum_nebula(&self, maximum_nebula: f32) {
        let maximum_nebula = if self.exists() {
            self.maximum_nebula.store(maximum_nebula);
            maximum_nebula
        } else {
            0.0
        };

        if maximum_nebula > self.current_nebula() {
            self.current_nebula.store(maximum_nebula);
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.current_metal.store_default();
        self.current_carbon.store_default();
        self.current_hydrogen.store_default();
        self.current_silicon.store_default();
        self.current_nebula.store_default();
        self.nebula_hue.store_default();
        self.base.reset_runtime_status();
    }

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
}

impl AsRef<SubsystemBase> for CargoSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
