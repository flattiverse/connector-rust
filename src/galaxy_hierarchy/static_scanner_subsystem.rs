use crate::galaxy_hierarchy::{
    Controllable, Cost, DynamicScannerSubsystem, ModernShipGeometry, RangeTolerance,
    ScannerSubsystemId, SubsystemBase, SubsystemExt,
};
use crate::{FlattiverseEvent, GameError, GameErrorKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Static scanner subsystem of a modern ship.
pub struct StaticScannerSubsystem {
    base: DynamicScannerSubsystem,
}

impl StaticScannerSubsystem {
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
            base: DynamicScannerSubsystem::new(
                controllable,
                name,
                id,
                exists,
                maximum_width,
                maximum_length,
                width_speed,
                length_speed,
                angle_speed,
                slot,
            ),
        }
    }

    /// The minimum configurable scan width in degree.
    #[inline]
    pub fn minimum_width(&self) -> f32 {
        self.base.minimum_width()
    }

    /// The minimum configurable scan length.
    #[inline]
    pub fn minimum_length(&self) -> f32 {
        self.base.minimum_length()
    }

    #[inline]
    pub fn id(&self) -> ScannerSubsystemId {
        self.base.id()
    }

    /// The maximum configurable scan width in degree.
    #[inline]
    pub fn maximum_width(&self) -> f32 {
        self.base.maximum_width()
    }

    /// The maximum configurable scan length.
    #[inline]
    pub fn maximum_length(&self) -> f32 {
        self.base.maximum_length()
    }

    /// The maximum width change per tick in degree.
    #[inline]
    pub fn width_speed(&self) -> f32 {
        self.base.width_speed()
    }

    /// The maximum length change per tick.
    #[inline]
    pub fn length_speed(&self) -> f32 {
        self.base.length_speed()
    }

    /// The maximum angle change per tick in degree.
    #[inline]
    pub fn angle_speed(&self) -> f32 {
        self.base.angle_speed()
    }

    /// The currently configured scan width in degree.
    #[inline]
    pub fn current_width(&self) -> f32 {
        self.base.current_width()
    }

    /// The currently configured scan length.
    #[inline]
    pub fn current_length(&self) -> f32 {
        self.base.current_length()
    }

    /// The currently configured absolute scan center angle in degree.
    #[inline]
    pub fn current_angle(&self) -> f32 {
        self.base.current_angle()
    }

    /// The target scan width in degree.
    #[inline]
    pub fn target_width(&self) -> f32 {
        self.base.target_width()
    }

    /// The target scan length.
    #[inline]
    pub fn target_length(&self) -> f32 {
        self.base.target_length()
    }

    /// The target absolute scan center angle in degree.
    #[inline]
    pub fn target_angle(&self) -> f32 {
        self.base.target_angle()
    }

    /// Whether the scanner is currently active on the server.
    #[inline]
    pub fn active(&self) -> bool {
        self.base.active()
    }

    /// The energy consumed by the scanner during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.base.consumed_energy_this_tick()
    }

    /// The ions consumed by the scanner during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.base.consumed_ions_this_tick()
    }

    /// The neutrinos consumed by the scanner during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.base.consumed_neutrinos_this_tick()
    }

    /// Calculates the scanner tick costs. The current placeholder model scales with the scanner
    /// surface and is tuned so that a maximum scan of 90 x 300 costs about 20 energy per tick.
    /// This also accepts smaller runtime dimensions that occur while the scanner ramps up after
    /// activation.
    /// Returns `None` if the subsystem does not exist or the requested values are outside the valid
    /// range. Values just above the maximum width or length are clipped before the cost is
    /// calculated.
    #[inline]
    pub fn calculate_cost(&self, width: f32, length: f32) -> Option<Cost> {
        self.base.calculate_cost(width, length)
    }

    /// Set the target scanner configuration on the server.
    /// Width and length values just outside the valid range are clipped before they are sent.
    /// Scanner angles are absolute world angles.
    pub async fn set(&self, width: f32, length: f32, angle_offset: f32) -> Result<(), GameError> {
        let controllable = self.base.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let width =
                RangeTolerance::clamped_range(width, self.minimum_width(), self.maximum_width())
                    .map_err(|reason| GameErrorKind::InvalidArgument {
                        reason,
                        parameter: "width".to_string(),
                    })?;

            let length =
                RangeTolerance::clamped_range(length, self.minimum_length(), self.maximum_length())
                    .map_err(|reason| GameErrorKind::InvalidArgument {
                        reason,
                        parameter: "length".to_string(),
                    })?;

            let angle_offset = RangeTolerance::clamped_range(
                angle_offset,
                -ModernShipGeometry::SCANNER_MAXIMUM_ANGLE_OFFSET,
                ModernShipGeometry::SCANNER_MAXIMUM_ANGLE_OFFSET,
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "angle".to_string(),
            })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .static_scanner_subsystem_set(
                    controllable.id(),
                    self.slot(),
                    width,
                    length,
                    angle_offset,
                )
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
                .static_scanner_subsystem_on(controllable.id(), self.slot())
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
                .static_scanner_subsystem_off(controllable.id(), self.slot())
                .await
        }
    }

    #[inline]
    pub(crate) fn reset_runtime(&self) {
        self.base.reset_runtime();
    }

    #[inline]
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
        self.base.update_runtime(
            active,
            current_width,
            current_length,
            current_angle,
            target_width,
            target_length,
            target_angle,
            status,
            consumed_energy_this_tick,
            consumed_ions_this_tick,
            consumed_neutrinos_this_tick,
        );
    }

    #[inline]
    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        self.base.create_runtime_event()
    }
}

impl AsRef<SubsystemBase> for StaticScannerSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
