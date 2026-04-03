use crate::galaxy_hierarchy::{Cluster, RailgunDirection};
use crate::network::PacketReader;
use crate::unit::{
    AbstractPlayerUnit, ClassicShipEngineSubsystemInfo, DynamicInterceptorFabricatorSubsystemInfo,
    DynamicInterceptorLauncherSubsystemInfo, DynamicInterceptorMagazineSubsystemInfo,
    DynamicScannerSubsystemInfo, DynamicShotFabricatorSubsystemInfo,
    DynamicShotLauncherSubsystemInfo, DynamicShotMagazineSubsystemInfo, JumpDriveSubsystemInfo,
    MobileUnit, MobileUnitInternal, NebulaCollectorSubsystemInfo, PlayerUnit, PlayerUnitInternal,
    RailgunSubsystemInfo, Unit, UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Readable;
use crate::{GameError, SubsystemStatus, Vector};
use std::sync::{Arc, Weak};

/// Visible snapshot of a classic-ship player unit in a cluster.
/// This mirrors what the local player can currently see about the ship and must not be confused
/// with the owner-side [`ClassicShipControllable`] used to command the local player's own ship.
///
/// [`ClassicShipControllable`]: crate::galaxy_hierarchy::ClassicShipControllable
#[derive(Debug, Clone)]
pub struct ClassicShipPlayerUnit {
    parent: AbstractPlayerUnit,
    nebula_collector: NebulaCollectorSubsystemInfo,
    engine: ClassicShipEngineSubsystemInfo,
    shot_launcher: DynamicShotLauncherSubsystemInfo,
    shot_magazine: DynamicShotMagazineSubsystemInfo,
    shot_fabricator: DynamicShotFabricatorSubsystemInfo,
    interceptor_launcher: DynamicInterceptorLauncherSubsystemInfo,
    interceptor_magazine: DynamicInterceptorMagazineSubsystemInfo,
    interceptor_fabricator: DynamicInterceptorFabricatorSubsystemInfo,
    railgun: RailgunSubsystemInfo,
    main_scanner: DynamicScannerSubsystemInfo,
    secondary_scanner: DynamicScannerSubsystemInfo,
    jump_drive: JumpDriveSubsystemInfo,
}

impl ClassicShipPlayerUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractPlayerUnit::new(cluster, name, reader)?,
            nebula_collector: Default::default(),
            engine: Default::default(),
            shot_launcher: Default::default(),
            shot_magazine: Default::default(),
            shot_fabricator: Default::default(),
            interceptor_launcher: Default::default(),
            interceptor_magazine: Default::default(),
            interceptor_fabricator: Default::default(),
            railgun: Default::default(),
            main_scanner: Default::default(),
            secondary_scanner: Default::default(),
            jump_drive: Default::default(),
        }))
    }

    /// Visible snapshot of the classic-ship engine configuration and tick runtime.
    #[inline]
    pub fn engine(&self) -> &ClassicShipEngineSubsystemInfo {
        &self.engine
    }

    /// Visible snapshot of the nebula collector subsystem.
    #[inline]
    pub fn nebula_collector(&self) -> &NebulaCollectorSubsystemInfo {
        &self.nebula_collector
    }

    /// Visible snapshot of the shot fabricator subsystem.
    #[inline]
    pub fn shot_launcher(&self) -> &DynamicShotLauncherSubsystemInfo {
        &self.shot_launcher
    }

    /// Visible snapshot of the shot fabricator subsystem.
    #[inline]
    pub fn shot_magazine(&self) -> &DynamicShotMagazineSubsystemInfo {
        &self.shot_magazine
    }

    /// Visible snapshot of the shot fabricator subsystem.
    #[inline]
    pub fn shot_fabricator(&self) -> &DynamicShotFabricatorSubsystemInfo {
        &self.shot_fabricator
    }

    /// Visible snapshot of the interceptor launcher subsystem and its configured interceptor profile.
    #[inline]
    pub fn interceptor_launcher(&self) -> &DynamicInterceptorLauncherSubsystemInfo {
        &self.interceptor_launcher
    }

    /// Visible snapshot of the interceptor magazine subsystem.
    #[inline]
    pub fn interceptor_magazine(&self) -> &DynamicInterceptorMagazineSubsystemInfo {
        &self.interceptor_magazine
    }

    /// Visible snapshot of the interceptor fabricator subsystem.
    #[inline]
    pub fn interceptor_fabricator(&self) -> &DynamicInterceptorFabricatorSubsystemInfo {
        &self.interceptor_fabricator
    }

    /// Visible snapshot of the railgun subsystem.
    #[inline]
    pub fn railgun(&self) -> &RailgunSubsystemInfo {
        &self.railgun
    }

    /// Visible snapshot of the primary scanner subsystem.
    #[inline]
    pub fn main_scanner(&self) -> &DynamicScannerSubsystemInfo {
        &self.main_scanner
    }

    /// Visible snapshot of the secondary scanner subsystem.
    /// On the current reference classic ship loadout this is usually not installed, so
    /// [`DynamicScannerSubsystemInfo::exists()`] is often `false`.
    #[inline]
    pub fn secondary_scanner(&self) -> &DynamicScannerSubsystemInfo {
        &self.secondary_scanner
    }

    /// Visible snapshot of the jump-drive subsystem.
    #[inline]
    pub fn jump_drive(&self) -> &JumpDriveSubsystemInfo {
        &self.jump_drive
    }
}

impl UnitInternal for ClassicShipPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.nebula_collector.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        self.main_scanner.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.secondary_scanner.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.engine.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            Vector::from_read(reader),
            Vector::from_read(reader),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.shot_launcher.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_uint16(),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            Vector::from_read(reader),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.shot_magazine.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.shot_fabricator.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_byte() != 0,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        self.interceptor_launcher.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_uint16(),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            Vector::from_read(reader),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        self.interceptor_magazine.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.interceptor_fabricator.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_byte() != 0,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        self.railgun.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            RailgunDirection::read(reader),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        self.jump_drive
            .update(reader.read_byte() != 0, reader.read_f32());
    }
}

impl UnitCastTable for ClassicShipPlayerUnit {
    cast_fn!(mobile_unit_cast_fn, ClassicShipPlayerUnit, dyn MobileUnit);
    cast_fn!(player_unit_cast_fn, ClassicShipPlayerUnit, dyn PlayerUnit);
}

impl UnitHierarchy for ClassicShipPlayerUnit {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_player_unit(&self) -> Option<&dyn PlayerUnit> {
        Some(self)
    }

    #[inline]
    fn as_classic_ship(&self) -> Option<&ClassicShipPlayerUnit> {
        Some(self)
    }
}

impl Unit for ClassicShipPlayerUnit {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::ClassicShipPlayerUnit
    }
}

impl MobileUnitInternal for ClassicShipPlayerUnit {}
impl MobileUnit for ClassicShipPlayerUnit {}

impl PlayerUnitInternal for ClassicShipPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn PlayerUnit {
        &self.parent
    }
}

impl PlayerUnit for ClassicShipPlayerUnit {}
