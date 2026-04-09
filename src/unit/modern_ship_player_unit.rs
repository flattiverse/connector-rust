use crate::galaxy_hierarchy::{Cluster, ModernShipGeometry};
use crate::network::PacketReader;
use crate::unit::{
    AbstractPlayerUnit, DynamicInterceptorFabricatorSubsystemInfo,
    DynamicInterceptorLauncherSubsystemInfo, DynamicScannerSubsystemInfo,
    DynamicShotFabricatorSubsystemInfo, DynamicShotLauncherSubsystemInfo, JumpDriveSubsystemInfo,
    MobileUnit, MobileUnitInternal, ModernRailgunSubsystemInfo, ModernShipEngineSubsystemInfo,
    NebulaCollectorSubsystemInfo, PlayerUnit, PlayerUnitInternal,
    StaticInterceptorMagazineSubsystemInfo, StaticShotMagazineSubsystemInfo, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::ops::Deref;
use std::sync::{Arc, Weak};

/// Visible snapshot of a modern-ship player unit in a cluster.
#[derive(Debug, Clone)]
pub struct ModernShipPlayerUnit {
    parent: AbstractPlayerUnit,
    nebula_collector: NebulaCollectorSubsystemInfo,
    engines: [ModernShipEngineSubsystemInfo; ModernShipGeometry::ENGINE_SLOTS.len()],
    scanners: [DynamicScannerSubsystemInfo; ModernShipGeometry::SCANNER_SLOTS.len()],
    shot_launchers:
        [DynamicShotLauncherSubsystemInfo; ModernShipGeometry::SHOT_LAUNCHER_SLOTS.len()],
    shot_magazines:
        [StaticShotMagazineSubsystemInfo; ModernShipGeometry::SHOT_MAGAZINE_SLOTS.len()],
    shot_fabricators:
        [DynamicShotFabricatorSubsystemInfo; ModernShipGeometry::SHOT_FABRICATOR_SLOTS.len()],
    interceptor_launchers: [DynamicInterceptorLauncherSubsystemInfo; 2],
    interceptor_magazines: [StaticInterceptorMagazineSubsystemInfo; 2],
    interceptor_fabricators: [DynamicInterceptorFabricatorSubsystemInfo; 2],
    railguns: [ModernRailgunSubsystemInfo; ModernShipGeometry::RAILGUN_SLOTS.len()],
    jump_drive: JumpDriveSubsystemInfo,
}

impl ModernShipPlayerUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractPlayerUnit::new(cluster, name, reader)?,
            nebula_collector: NebulaCollectorSubsystemInfo::default(),
            engines: core::array::repeat(Default::default()),
            scanners: core::array::repeat(Default::default()),
            shot_launchers: core::array::repeat(Default::default()),
            shot_magazines: core::array::repeat(Default::default()),
            shot_fabricators: core::array::repeat(Default::default()),
            interceptor_launchers: core::array::repeat(Default::default()),
            interceptor_magazines: core::array::repeat(Default::default()),
            interceptor_fabricators: core::array::repeat(Default::default()),
            railguns: core::array::repeat(Default::default()),
            jump_drive: Default::default(),
        }))
    }

    #[inline]
    pub fn nebula_collector(&self) -> &NebulaCollectorSubsystemInfo {
        &self.nebula_collector
    }

    #[inline]
    pub fn engines(&self) -> &[ModernShipEngineSubsystemInfo] {
        &self.engines
    }

    #[inline]
    pub fn scanners(&self) -> &[DynamicScannerSubsystemInfo] {
        &self.scanners
    }

    #[inline]
    pub fn shot_launchers(&self) -> &[DynamicShotLauncherSubsystemInfo] {
        &self.shot_launchers
    }

    #[inline]
    pub fn shot_magazines(&self) -> &[StaticShotMagazineSubsystemInfo] {
        &self.shot_magazines
    }

    #[inline]
    pub fn shot_fabricators(&self) -> &[DynamicShotFabricatorSubsystemInfo] {
        &self.shot_fabricators
    }

    #[inline]
    pub fn interceptor_launchers(&self) -> &[DynamicInterceptorLauncherSubsystemInfo] {
        &self.interceptor_launchers
    }

    #[inline]
    pub fn interceptor_magazines(&self) -> &[StaticInterceptorMagazineSubsystemInfo] {
        &self.interceptor_magazines
    }

    #[inline]
    pub fn interceptor_fabricators(&self) -> &[DynamicInterceptorFabricatorSubsystemInfo] {
        &self.interceptor_fabricators
    }

    #[inline]
    pub fn railguns(&self) -> &[ModernRailgunSubsystemInfo] {
        &self.railguns
    }

    #[inline]
    pub fn jump_drive(&self) -> &JumpDriveSubsystemInfo {
        &self.jump_drive
    }
}

impl UnitInternal for ModernShipPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.nebula_collector.update_from_reader(reader);

        for scanner in &self.scanners {
            scanner.update_from_reader(reader);
        }

        for engine in &self.engines {
            engine.update_from_reader(reader);
        }

        for ((launcher, magazine), fabricator) in self
            .shot_launchers
            .iter()
            .zip(self.shot_magazines.iter().map(Deref::deref))
            .zip(&self.shot_fabricators)
            .chain(
                self.interceptor_launchers
                    .iter()
                    .map(Deref::deref)
                    .zip(
                        self.interceptor_magazines
                            .iter()
                            .map(Deref::deref)
                            .map(Deref::deref),
                    )
                    .zip(self.interceptor_fabricators.iter().map(Deref::deref)),
            )
        {
            launcher.update_from_reader(reader);
            magazine.update_from_reader(reader);
            fabricator.update_from_reader(reader);
        }

        for railgun in &self.railguns {
            railgun.update_from_reader(reader);
        }

        self.jump_drive.update_from_reader(reader);
    }
}

impl UnitCastTable for ModernShipPlayerUnit {
    cast_fn!(mobile_unit_cast_fn, ModernShipPlayerUnit, dyn MobileUnit);
    cast_fn!(player_unit_cast_fn, ModernShipPlayerUnit, dyn PlayerUnit);
}

impl UnitHierarchy for ModernShipPlayerUnit {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_player_unit(&self) -> Option<&dyn PlayerUnit> {
        Some(self)
    }

    #[inline]
    fn as_modern_ship(&self) -> Option<&ModernShipPlayerUnit> {
        Some(self)
    }
}

impl Unit for ModernShipPlayerUnit {
    #[inline]
    fn radius(&self) -> f32 {
        ModernShipGeometry::RADIUS
    }

    #[inline]
    fn gravity(&self) -> f32 {
        0.0012
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::ModernShipPlayerUnit
    }
}

impl MobileUnitInternal for ModernShipPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for ModernShipPlayerUnit {}

impl PlayerUnitInternal for ModernShipPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn PlayerUnit {
        &self.parent
    }
}

impl PlayerUnit for ModernShipPlayerUnit {}
