use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPlayerUnit, ClassicShipEngineSubsystemInfo, DynamicScannerSubsystemInfo,
    DynamicShotFabricatorSubsystemInfo, DynamicShotLauncherSubsystemInfo,
    DynamicShotMagazineSubsystemInfo, MobileUnit, MobileUnitInternal, PlayerUnit,
    PlayerUnitInternal, Unit, UnitHierarchy, UnitInternal, UnitKind,
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
    engine: ClassicShipEngineSubsystemInfo,
    shot_launcher: DynamicShotLauncherSubsystemInfo,
    shot_magazine: DynamicShotMagazineSubsystemInfo,
    shot_fabricator: DynamicShotFabricatorSubsystemInfo,
    main_scanner: DynamicScannerSubsystemInfo,
    secondary_scanner: DynamicScannerSubsystemInfo,
}

impl ClassicShipPlayerUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractPlayerUnit::new(cluster, name, reader)?,
            engine: Default::default(),
            shot_launcher: Default::default(),
            shot_magazine: Default::default(),
            shot_fabricator: Default::default(),
            main_scanner: Default::default(),
            secondary_scanner: Default::default(),
        }))
    }

    /// Visible snapshot of the engine subsystem.
    #[inline]
    pub fn engine(&self) -> &ClassicShipEngineSubsystemInfo {
        &self.engine
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

    /// Visible snapshot of the primary scanner subsystem.
    #[inline]
    pub fn main_scanner(&self) -> &DynamicScannerSubsystemInfo {
        &self.main_scanner
    }

    /// Visible snapshot of the secondary scanner subsystem.
    #[inline]
    pub fn secondary_scanner(&self) -> &DynamicScannerSubsystemInfo {
        &self.secondary_scanner
    }
}

impl UnitInternal for ClassicShipPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

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
    }
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
