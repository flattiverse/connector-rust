use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    ClassicShipEngineSubsystemInfo, DynamicScannerSubsystemInfo,
    DynamicShotFabricatorSubsystemInfo, DynamicShotLauncherSubsystemInfo,
    DynamicShotMagazineSubsystemInfo, PlayerUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind,
};
use crate::utils::Readable;
use crate::{SubsystemStatus, Vector};
use std::sync::Weak;

/// A classic ship for noobs.
#[derive(Debug, Clone)]
pub struct ClassicShipPlayerUnit {
    base: UnitBase,
    player_unit: PlayerUnit,
    engine: ClassicShipEngineSubsystemInfo,
    shot_launcher: DynamicShotLauncherSubsystemInfo,
    shot_magazine: DynamicShotMagazineSubsystemInfo,
    shot_fabricator: DynamicShotFabricatorSubsystemInfo,
    main_scanner: DynamicScannerSubsystemInfo,
    secondary_scanner: DynamicScannerSubsystemInfo,
}

impl ClassicShipPlayerUnit {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        let galaxy = cluster.upgrade().unwrap().galaxy();

        Self {
            base: UnitBase::new(cluster, name),
            player_unit: PlayerUnit::read(&*galaxy, reader),
            engine: Default::default(),
            shot_launcher: Default::default(),
            shot_magazine: Default::default(),
            shot_fabricator: Default::default(),
            main_scanner: Default::default(),
            secondary_scanner: Default::default(),
        }
    }

    #[inline]
    pub const fn gravity(&self) -> f32 {
        0.0012
    }

    #[inline]
    pub const fn radius(&self) -> f32 {
        14.0
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

impl AsRef<UnitBase> for ClassicShipPlayerUnit {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<PlayerUnit> for ClassicShipPlayerUnit {
    #[inline]
    fn as_ref(&self) -> &PlayerUnit {
        &self.player_unit
    }
}

impl<'a> UnitExtSealed<'a> for &'a ClassicShipPlayerUnit {
    type Parent = (&'a UnitBase, &'a PlayerUnit);

    #[inline]
    fn parent(self) -> (&'a UnitBase, &'a PlayerUnit) {
        (&self.base, &self.player_unit)
    }

    #[inline]
    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

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

impl<'a> UnitExt<'a> for &'a ClassicShipPlayerUnit {
    #[inline]
    fn radius(self) -> f32 {
        14.0
    }

    #[inline]
    fn gravity(self) -> f32 {
        0.0012
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::ClassicShipPlayerUnit
    }
}
