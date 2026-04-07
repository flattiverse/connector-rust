use crate::galaxy_hierarchy::{
    Controllable, Cost, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::utils::{Also, Atomic};
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ScannerSubsystemId(pub(crate) u8);

/// Represents a persistent scanner subsystem configuration.
#[derive(Debug)]
pub struct DynamicScannerSubsystem {
    base: SubsystemBase,

    id: ScannerSubsystemId,
    maximum_width: Atomic<f32>,
    maximum_length: Atomic<f32>,
    width_speed: Atomic<f32>,
    length_speed: Atomic<f32>,
    angle_speed: Atomic<f32>,

    current_width: Atomic<f32>,
    current_length: Atomic<f32>,
    current_angle: Atomic<f32>,

    target_width: Atomic<f32>,
    target_length: Atomic<f32>,
    target_angle: Atomic<f32>,

    active: Atomic<bool>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicScannerSubsystem {
    const MINIMUM_WIDTH_VALUE: f32 = 5.0;
    const MINIMUM_LENGTH_VALUE: f32 = 20.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        id: ScannerSubsystemId,
        exists: bool,
        maximum_width: f32,
        maximum_length: f32,
        width_speed: f32,
        length_speed: f32,
        angle_speed: f32,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            id,
            maximum_width: Atomic::from(if exists { maximum_width } else { 0.0 }),
            maximum_length: Atomic::from(if exists { maximum_length } else { 0.0 }),
            width_speed: Atomic::from(if exists { width_speed } else { 0.0 }),
            length_speed: Atomic::from(if exists { length_speed } else { 0.0 }),
            angle_speed: Atomic::from(if exists { angle_speed } else { 0.0 }),
            current_width: Default::default(),
            current_length: Default::default(),
            current_angle: Default::default(),
            target_width: Default::default(),
            target_length: Default::default(),
            target_angle: Default::default(),
            active: Default::default(),
            consumed_energy_this_tick: Default::default(),
            consumed_ions_this_tick: Default::default(),
            consumed_neutrinos_this_tick: Default::default(),
        }
        .also(|this| this.reset_runtime())
    }

    pub(crate) fn create_classic_ship_primary_scanner(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "MainScanner".to_string(),
            ScannerSubsystemId(0),
            true,
            90.0,
            300.0,
            2.5,
            10.0,
            5.0,
            SubsystemSlot::PrimaryScanner,
        )
    }

    pub(crate) fn create_classic_ship_secondary_scanner(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "SecondaryScanner".to_string(),
            ScannerSubsystemId(1),
            true,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            SubsystemSlot::PrimaryScanner,
        )
    }

    /// The minimum configurable scan width in degree.
    #[inline]
    pub fn minimum_width(&self) -> f32 {
        Self::MINIMUM_WIDTH_VALUE
    }

    /// The minimum configurable scan length.
    #[inline]
    pub fn minimum_length(&self) -> f32 {
        Self::MINIMUM_LENGTH_VALUE
    }

    #[inline]
    pub fn id(&self) -> ScannerSubsystemId {
        self.id
    }

    /// The maximum configurable scan width in degree.
    #[inline]
    pub fn maximum_width(&self) -> f32 {
        self.maximum_width.load()
    }

    /// The maximum configurable scan length.
    #[inline]
    pub fn maximum_length(&self) -> f32 {
        self.maximum_length.load()
    }

    /// The maximum width change per tick in degree.
    #[inline]
    pub fn width_speed(&self) -> f32 {
        self.width_speed.load()
    }

    /// The maximum length change per tick.
    #[inline]
    pub fn length_speed(&self) -> f32 {
        self.length_speed.load()
    }

    /// The maximum angle change per tick in degree.
    #[inline]
    pub fn angle_speed(&self) -> f32 {
        self.angle_speed.load()
    }

    pub(crate) fn set_capabilities(
        &self,
        maximum_width: f32,
        maximum_length: f32,
        width_speed: f32,
        length_speed: f32,
        angle_speed: f32,
    ) {
        if self.exists() {
            self.maximum_width.store(maximum_width);
            self.maximum_length.store(maximum_length);
            self.width_speed.store(width_speed);
            self.length_speed.store(length_speed);
            self.angle_speed.store(angle_speed);
        } else {
            self.maximum_width.store(0.0);
            self.maximum_length.store(0.0);
            self.width_speed.store(0.0);
            self.length_speed.store(0.0);
            self.angle_speed.store(0.0);
        }

        // TODO self.refresh_tier();
    }

    /// The currently configured scan width in degree.
    #[inline]
    pub fn current_width(&self) -> f32 {
        self.current_width.load()
    }

    /// The currently configured scan length.
    #[inline]
    pub fn current_length(&self) -> f32 {
        self.current_length.load()
    }

    /// The currently configured absolute scan center angle in degree.
    #[inline]
    pub fn current_angle(&self) -> f32 {
        self.current_angle.load()
    }

    /// The target scan width in degree.
    #[inline]
    pub fn target_width(&self) -> f32 {
        self.target_width.load()
    }

    /// The target scan length.
    #[inline]
    pub fn target_length(&self) -> f32 {
        self.target_length.load()
    }

    /// The target absolute scan center angle in degree.
    #[inline]
    pub fn target_angle(&self) -> f32 {
        self.target_angle.load()
    }

    /// Whether the scanner is currently active on the server.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// The energy consumed by the scanner during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed by the scanner during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed by the scanner during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the scanner tick costs. The current placeholder model scales with the scanner
    /// surface and is tuned so that a maximum scan of 90 x 300 costs about 20 energy per tick.
    /// This also accepts smaller runtime dimensions that occur while the scanner ramps up after
    /// activation.
    /// Returns `None` if the subsystem does not exist or the requested values are outside the valid
    /// range. Values just above the maximum width or length are clipped before the cost is
    /// calculated.
    pub fn calculate_cost(&self, width: f32, length: f32) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let width = RangeTolerance::clamped_maximum(width, self.maximum_width()).ok()?;
            let length = RangeTolerance::clamped_maximum(length, self.maximum_length()).ok()?;

            if self.maximum_length() > 430.0 {
                Cost::default()
                    .with_neutrinos(ShipBalancing::calculate_scanner_energy(width, length) / 100.0)
            } else {
                Cost::default().with_energy(ShipBalancing::calculate_scanner_energy(width, length))
            }
            .into_values_checked()
        }
    }

    /// Set the target scanner configuration on the server.
    /// Width and length values just outside the valid range are clipped before they are sent.
    /// Scanner angles are absolute world angles.
    pub async fn set(&self, width: f32, length: f32, angle: f32) -> Result<(), GameError> {
        let controllable = self.base.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let width = RangeTolerance::clamped_range(
                width,
                Self::MINIMUM_WIDTH_VALUE,
                self.maximum_width(),
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "width".to_string(),
            })?;

            let length = RangeTolerance::clamped_range(
                length,
                Self::MINIMUM_LENGTH_VALUE,
                self.maximum_length(),
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "length".to_string(),
            })?;

            let angle = RangeTolerance::validated_f32(angle).map_err(|reason| {
                GameErrorKind::InvalidArgument {
                    reason,
                    parameter: "angle".to_string(),
                }
            })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .dynamic_scanner_subsystem_set(controllable.id(), self.id, width, length, angle)
                .await
        }
    }

    /// Turns the scanner on.
    pub async fn on(&self) -> Result<(), GameError> {
        let controllable = self.base.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            controllable
                .cluster()
                .galaxy()
                .connection()
                .dynamic_scanner_subsystem_on(controllable.id(), self.id)
                .await
        }
    }

    /// Turns the scanner off.
    pub async fn off(&self) -> Result<(), GameError> {
        let controllable = self.base.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            controllable
                .cluster()
                .galaxy()
                .connection()
                .dynamic_scanner_subsystem_off(controllable.id(), self.id)
                .await
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.current_width.store_default();
        self.current_length.store_default();
        self.current_angle.store_default();
        self.target_width.store_default();
        self.target_length.store_default();
        self.target_angle.store_default();
        self.active.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
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
        self.active.store(active);
        self.current_width.store(current_width);
        self.current_length.store(current_length);
        self.current_angle.store(current_angle);
        self.target_width.store(target_width);
        self.target_length.store(target_length);
        self.target_angle.store(target_angle);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::DynamicScannerSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    active: self.active(),
                    current_width: self.current_width(),
                    current_length: self.current_length(),
                    current_angle: self.current_angle(),
                    target_width: self.target_width(),
                    target_length: self.target_length(),
                    target_angle: self.target_angle(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                }
                .into(),
            )
        }
    }

    // TODO pub fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for DynamicScannerSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
