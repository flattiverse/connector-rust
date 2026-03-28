use crate::galaxy_hierarchy::{
    AsSubsystemBase, ClassicShipEngineSubsystem, Controllable, ControllableSpecialization,
    Controls, DynamicScannerSubsystem, DynamicShotFabricatorSubsystem,
    DynamicShotLauncherSubsystem, DynamicShotMagazineSubsystem, SubsystemBase,
};
use crate::network::PacketReader;
use crate::utils::Readable;
use crate::{FlattiverseEvent, SubsystemSlot, SubsystemStatus, Vector};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct ClassicShipControllable {
    pub(crate) engine: ClassicShipEngineSubsystem,
    pub(crate) shot_launcher: DynamicShotLauncherSubsystem,
    pub(crate) shot_magazine: DynamicShotMagazineSubsystem,
    pub(crate) shot_fabricator: DynamicShotFabricatorSubsystem,
    pub(crate) main_scanner: DynamicScannerSubsystem,
    pub(crate) secondary_scanner: DynamicScannerSubsystem,
}

impl ClassicShipControllable {
    pub(crate) fn new() -> Self {
        Self {
            engine: ClassicShipEngineSubsystem::new(Weak::default()),
            shot_launcher: DynamicShotLauncherSubsystem::new(
                Weak::default(),
                "ShotLauncher".to_string(),
                true,
                SubsystemSlot::DynamicShotLauncher,
            ),
            shot_magazine: DynamicShotMagazineSubsystem::new(
                Weak::default(),
                "ShotMagazine".to_string(),
                true,
                SubsystemSlot::DynamicShotMagazine,
            ),
            shot_fabricator: DynamicShotFabricatorSubsystem::new(
                Weak::default(),
                "ShotFabricator".to_string(),
                true,
                SubsystemSlot::DynamicShotFabricator,
            ),
            main_scanner: DynamicScannerSubsystem::create_classic_ship_primary_scanner(
                Weak::default(),
            ),
            secondary_scanner: DynamicScannerSubsystem::create_classic_ship_secondary_scanner(
                Weak::default(),
            ),
        }
    }

    /// The engine subsystem of the classic ship.
    #[inline]
    pub fn engine(&self) -> &ClassicShipEngineSubsystem {
        &self.engine
    }

    /// The shot launcher subsystem of the classic ship.
    #[inline]
    pub fn shot_launcher(&self) -> &DynamicShotLauncherSubsystem {
        &self.shot_launcher
    }

    /// The shot magazine subsystem of the classic ship.
    #[inline]
    pub fn shot_magazine(&self) -> &DynamicShotMagazineSubsystem {
        &self.shot_magazine
    }

    /// The shot fabricator subsystem of the classic ship.
    #[inline]
    pub fn shot_fabricator(&self) -> &DynamicShotFabricatorSubsystem {
        &self.shot_fabricator
    }

    /// The primary scanner subsystem of the classic ship.
    #[inline]
    pub fn main_scanner(&self) -> &DynamicScannerSubsystem {
        &self.main_scanner
    }

    /// The secondary scanner subsystem of the classic ship.
    #[inline]
    pub fn secondary_scanner(&self) -> &DynamicScannerSubsystem {
        &self.secondary_scanner
    }

    pub(crate) fn reset_runtime(&self) {
        self.engine.reset_runtime();
        self.shot_launcher.reset_runtime();
        self.shot_magazine.reset_runtime();
        self.shot_fabricator.reset_runtime();
        self.main_scanner.reset_runtime();
        self.secondary_scanner.reset_runtime();
    }

    pub(crate) fn read_runtime(&self, reader: &mut dyn PacketReader) {
        self.main_scanner.update_runtime(
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
        self.secondary_scanner.update_runtime(
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
        self.engine.update_runtime(
            Vector::from_read(reader),
            Vector::from_read(reader),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.shot_launcher.update_runtime(
            Vector::from_read(reader),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.shot_magazine
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        self.shot_fabricator.update_runtime(
            reader.read_byte() != 0,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
    }

    pub(crate) fn iter_subsystem_bases(&self) -> impl Iterator<Item = &SubsystemBase> + '_ {
        [
            self.engine.as_subsystem_base(),
            self.shot_launcher.as_subsystem_base(),
            self.shot_magazine.as_subsystem_base(),
            self.shot_fabricator.as_subsystem_base(),
            self.main_scanner.as_subsystem_base(),
            self.secondary_scanner.as_subsystem_base(),
        ]
        .into_iter()
    }

    pub(crate) fn iter_runtime_events(&self) -> impl Iterator<Item = FlattiverseEvent> {
        [
            self.main_scanner.create_runtime_event(),
            self.secondary_scanner.create_runtime_event(),
            self.engine.create_runtime_event(),
            self.shot_launcher.create_runtime_event(),
            self.shot_magazine.create_runtime_event(),
            self.shot_fabricator.create_runtime_event(),
        ]
        .into_iter()
        .flatten()
    }
}

impl TryFrom<Arc<Controllable>> for Controls<ClassicShipControllable> {
    type Error = Arc<Controllable>;

    fn try_from(controllable: Arc<Controllable>) -> Result<Self, Self::Error> {
        match controllable.specialization() {
            ControllableSpecialization::ClassicShip(p) => {
                Ok(Controls::<ClassicShipControllable>::proven(&p).control(controllable))
            }
        }
    }
}

impl Controls<ClassicShipControllable> {
    #[inline]
    pub fn as_classic_ship_specialization(&self) -> &ClassicShipControllable {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(specialization) => specialization,
            #[allow(unreachable_patterns)]
            _ => unreachable!("This was previously proven"),
        }
    }
}
