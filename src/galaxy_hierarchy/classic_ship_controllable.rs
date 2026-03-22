use crate::galaxy_hierarchy::{
    AsSubsystemBase, ClassicShipEngineSubsystem, Controllable, ControllableSpecialization,
    Controls, ScannerSubsystem, ShotWeaponSubsystem, SubsystemBase,
};
use crate::network::PacketReader;
use crate::utils::Readable;
use crate::{FlattiverseEvent, SubsystemSlot, SubsystemStatus, Vector};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct ClassicShipSpecialization {
    pub(crate) engine: ClassicShipEngineSubsystem,
    pub(crate) weapon: ShotWeaponSubsystem,
    pub(crate) main_scanner: ScannerSubsystem,
    pub(crate) secondary_scanner: ScannerSubsystem,
}

impl ClassicShipSpecialization {
    pub(crate) fn new() -> Self {
        Self {
            engine: ClassicShipEngineSubsystem::new(Weak::default()),
            weapon: ShotWeaponSubsystem::new(
                Weak::default(),
                "Weapon".to_string(),
                true,
                SubsystemSlot::FrontShotLauncher,
            ),
            main_scanner: ScannerSubsystem::create_classic_ship_primary_scanner(Weak::default()),
            secondary_scanner: ScannerSubsystem::create_classic_ship_secondary_scanner(
                Weak::default(),
            ),
        }
    }

    /// The engine subsystem of the classic ship.
    #[inline]
    pub fn engine(&self) -> &ClassicShipEngineSubsystem {
        &self.engine
    }

    /// The weapon subsystem of the classic ship.
    #[inline]
    pub fn weapon(&self) -> &ShotWeaponSubsystem {
        &self.weapon
    }

    /// The primary scanner subsystem of the classic ship.
    #[inline]
    pub fn main_scanner(&self) -> &ScannerSubsystem {
        &self.main_scanner
    }

    /// The secondary scanner subsystem of the classic ship.
    #[inline]
    pub fn secondary_scanner(&self) -> &ScannerSubsystem {
        &self.secondary_scanner
    }

    pub(crate) fn reset_runtime(&self) {
        self.engine.reset_runtime();
        self.weapon.reset_runtime();
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
        self.weapon.update_runtime(
            Vector::from_read(reader),
            reader.read_uint16(),
            reader.read_f32(),
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
            self.weapon.as_subsystem_base(),
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
            self.weapon.create_runtime_event(),
        ]
        .into_iter()
        .flatten()
    }
}

impl TryFrom<Arc<Controllable>> for Controls<ClassicShipSpecialization> {
    type Error = Arc<Controllable>;

    fn try_from(controllable: Arc<Controllable>) -> Result<Self, Self::Error> {
        match controllable.specialization() {
            ControllableSpecialization::ClassicShip(p) => {
                Ok(Controls::<ClassicShipSpecialization>::proven(&p).control(controllable))
            }
        }
    }
}

impl Controls<ClassicShipSpecialization> {
    #[inline]
    pub fn as_classic_ship_specialization(&self) -> &ClassicShipSpecialization {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(specialization) => specialization,
            #[allow(unreachable_patterns)]
            _ => unreachable!("This was previously proven"),
        }
    }
}
