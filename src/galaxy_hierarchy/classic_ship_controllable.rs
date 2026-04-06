use crate::galaxy_hierarchy::{
    AsSubsystemBase, ClassicShipEngineSubsystem, Controllable, ControllableSpecialization,
    Controls, DynamicInterceptorFabricatorSubsystem, DynamicInterceptorLauncherSubsystem,
    DynamicInterceptorMagazineSubsystem, DynamicScannerSubsystem, DynamicShotFabricatorSubsystem,
    DynamicShotLauncherSubsystem, DynamicShotMagazineSubsystem, JumpDriveSubsystem,
    NebulaCollectorSubsystem, RailgunDirection, RailgunSubsystem, SubsystemBase,
};
use crate::network::PacketReader;
use crate::utils::Readable;
use crate::{FlattiverseEvent, SubsystemSlot, SubsystemStatus, Vector};
use std::sync::{Arc, Weak};

/// Owner-side handle for one registered classic-ship controllable.
#[derive(Debug)]
pub struct ClassicShipControllable {
    pub(crate) nebula_collector: NebulaCollectorSubsystem,
    pub(crate) engine: ClassicShipEngineSubsystem,
    pub(crate) shot_launcher: DynamicShotLauncherSubsystem,
    pub(crate) shot_magazine: DynamicShotMagazineSubsystem,
    pub(crate) shot_fabricator: DynamicShotFabricatorSubsystem,
    pub(crate) interceptor_launcher: DynamicInterceptorLauncherSubsystem,
    pub(crate) interceptor_magazine: DynamicInterceptorMagazineSubsystem,
    pub(crate) interceptor_fabricator: DynamicInterceptorFabricatorSubsystem,
    pub(crate) railgun: RailgunSubsystem,
    pub(crate) main_scanner: DynamicScannerSubsystem,
    pub(crate) secondary_scanner: DynamicScannerSubsystem,
    pub(crate) jump_drive: JumpDriveSubsystem,
    pub(crate) equipped_crystals: [String; 3],
}

impl ClassicShipControllable {
    pub(crate) fn new() -> Self {
        Self {
            nebula_collector: NebulaCollectorSubsystem::create_classic_ship_nebula_collector(
                Weak::default(),
            ),
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
            interceptor_launcher: DynamicInterceptorLauncherSubsystem::new(
                Weak::default(),
                "InterceptorLauncher".to_string(),
                true,
                SubsystemSlot::DynamicInterceptorFabricator,
            ),
            interceptor_magazine: DynamicInterceptorMagazineSubsystem::new(
                Weak::default(),
                "InterceptorMagazine".to_string(),
                true,
                SubsystemSlot::DynamicInterceptorMagazine,
            ),
            interceptor_fabricator: DynamicInterceptorFabricatorSubsystem::new(
                Weak::default(),
                "InterceptorFabricator".to_string(),
                true,
                SubsystemSlot::DynamicInterceptorMagazine,
            ),
            railgun: RailgunSubsystem::new(
                Weak::default(),
                "Railgun".to_string(),
                true,
                SubsystemSlot::Railgun,
            ),
            main_scanner: DynamicScannerSubsystem::create_classic_ship_primary_scanner(
                Weak::default(),
            ),
            secondary_scanner: DynamicScannerSubsystem::create_classic_ship_secondary_scanner(
                Weak::default(),
            ),
            jump_drive: JumpDriveSubsystem::new(Weak::default(), true),
            equipped_crystals: [const { String::new() }; 3],
        }
    }

    pub(crate) fn read_initial_state(&mut self, reader: &mut dyn PacketReader) {
        let _nebula_collector_exists = reader.read_byte();
        let _nebula_collector_minimum_rate = reader.read_f32();
        let _nebula_collector_maximum_rate = reader.read_f32();
        self.nebula_collector.update_runtime(
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        let _main_scanner_exists = reader.read_byte();
        let _main_scanner_maximum_width = reader.read_f32();
        let _main_scanner_maximum_length = reader.read_f32();
        let _main_scanner_width_speed = reader.read_f32();
        let _main_scanner_length_speed = reader.read_f32();
        let _main_scanner_angle_speed = reader.read_f32();
        self.main_scanner.update_runtime(
            reader.read_byte() != 0x00,
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

        let _secondary_scanner_exists = reader.read_byte();
        let _secondary_scanner_maximum_width = reader.read_f32();
        let _secondary_scanner_maximum_length = reader.read_f32();
        let _secondary_scanner_width_speed = reader.read_f32();
        let _secondary_scanner_length_speed = reader.read_f32();
        let _secondary_scanner_angle_speed = reader.read_f32();
        self.secondary_scanner.update_runtime(
            reader.read_byte() != 0x00,
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

        let _engine_exists = reader.read_byte();
        let _engine_maximum = reader.read_f32();
        self.engine.update_runtime(
            Vector::from_read(reader),
            Vector::from_read(reader),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        let _shot_launcher_exists = reader.read_byte();
        let _shot_launcher_minimum_relative_movement = reader.read_f32();
        let _shot_launcher_maximum_relative_movement = reader.read_f32();
        let _shot_launcher_minimum_ticks = reader.read_uint16();
        let _shot_launcher_maximum_ticks = reader.read_uint16();
        let _shot_launcher_minimum_load = reader.read_f32();
        let _shot_launcher_maximum_load = reader.read_f32();
        let _shot_launcher_minimum_damage = reader.read_f32();
        let _shot_launcher_maximum_damage = reader.read_f32();
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

        let _shot_magazine_exists = reader.read_byte();
        let _shot_magazine_maximum_shots = reader.read_f32();
        self.shot_magazine
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));

        let _shot_fabricator_exists = reader.read_byte();
        let _shot_fabricator_minimum_rate = reader.read_f32();
        self.shot_fabricator.set_maximum_rate(reader.read_f32());
        self.shot_fabricator.update_runtime(
            reader.read_byte() != 0x00,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        let _interceptor_launcher_exists = reader.read_byte();
        let _interceptor_launcher_minimum_relative_movement = reader.read_f32();
        let _interceptor_launcher_maximum_relative_movement = reader.read_f32();
        let _interceptor_launcher_minimum_ticks = reader.read_uint16();
        let _interceptor_launcher_maximum_ticks = reader.read_uint16();
        let _interceptor_launcher_minimum_load = reader.read_f32();
        let _interceptor_launcher_maximum_load = reader.read_f32();
        let _interceptor_launcher_minimum_damage = reader.read_f32();
        let _interceptor_launcher_maximum_damage = reader.read_f32();
        self.interceptor_launcher.update_runtime(
            Vector::from_read(reader),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        let _interceptor_magazine_exists = reader.read_byte();
        let _interceptor_magazine_maximum_shots = reader.read_f32();
        self.interceptor_magazine
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));

        let _interceptor_fabricator_exists = reader.read_byte();
        let _interceptor_fabricator_minimum_rate = reader.read_f32();
        self.interceptor_fabricator
            .set_maximum_rate(reader.read_f32());
        self.interceptor_fabricator.update_runtime(
            reader.read_byte() != 0x00,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        let _railgun_exists = reader.read_byte();
        let _railgun_energy_cost = reader.read_f32();
        let _railgun_metal_cost = reader.read_f32();
        self.railgun.update_runtime(
            RailgunDirection::read(reader),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

        let _jump_drive_exists = reader.read_byte();
        self.jump_drive.set_energy_cost(reader.read_f32());

        self.equipped_crystals[0] = reader.read_string();
        self.equipped_crystals[1] = reader.read_string();
        self.equipped_crystals[2] = reader.read_string();
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

    /// The interceptor launcher subsystem of the classic ship.
    #[inline]
    pub fn interceptor_launcher(&self) -> &DynamicInterceptorLauncherSubsystem {
        &self.interceptor_launcher
    }

    /// The interceptor magazine subsystem of the classic ship.
    #[inline]
    pub fn interceptor_magazine(&self) -> &DynamicInterceptorMagazineSubsystem {
        &self.interceptor_magazine
    }

    /// The interceptor fabricator subsystem of the classic ship.
    #[inline]
    pub fn interceptor_fabricator(&self) -> &DynamicInterceptorFabricatorSubsystem {
        &self.interceptor_fabricator
    }

    /// The railgun subsystem of the classic ship.
    #[inline]
    pub fn railgun(&self) -> &RailgunSubsystem {
        &self.railgun
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

    /// The nebula collector subsystem of the classic ship.
    #[inline]
    pub fn nebula_collector(&self) -> &NebulaCollectorSubsystem {
        &self.nebula_collector
    }

    /// The jump-drive subsystem of the classic ship.
    #[inline]
    pub fn jump_drive(&self) -> &JumpDriveSubsystem {
        &self.jump_drive
    }

    /// The three equipped crystal names. Empty slots are reported as empty strings.
    #[inline]
    pub fn equipped_crystals(&self) -> &[String] {
        &self.equipped_crystals
    }

    pub(crate) fn reset_runtime(&self) {
        self.nebula_collector.reset_runtime();
        self.engine.reset_runtime();
        self.shot_launcher.reset_runtime();
        self.shot_magazine.reset_runtime();
        self.shot_fabricator.reset_runtime();
        self.interceptor_launcher.reset_runtime();
        self.interceptor_magazine.reset_runtime();
        self.interceptor_fabricator.reset_runtime();
        self.railgun.reset_runtime();
        self.main_scanner.reset_runtime();
        self.secondary_scanner.reset_runtime();
        self.jump_drive.reset_runtime();
    }

    pub(crate) fn read_runtime(&self, reader: &mut dyn PacketReader) {
        self.nebula_collector.update_runtime(
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
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
        self.interceptor_launcher.update_runtime(
            Vector::from_read(reader),
            reader.read_uint16(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.interceptor_magazine
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        self.interceptor_fabricator.update_runtime(
            reader.read_byte() != 0,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
        self.railgun.update_runtime(
            RailgunDirection::read(reader),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
    }

    pub(crate) fn iter_subsystem_bases(&self) -> impl Iterator<Item = &SubsystemBase> + '_ {
        [
            self.nebula_collector.as_subsystem_base(),
            self.engine.as_subsystem_base(),
            self.shot_launcher.as_subsystem_base(),
            self.shot_magazine.as_subsystem_base(),
            self.shot_fabricator.as_subsystem_base(),
            self.interceptor_launcher.as_subsystem_base(),
            self.interceptor_magazine.as_subsystem_base(),
            self.interceptor_fabricator.as_subsystem_base(),
            self.railgun.as_subsystem_base(),
            self.main_scanner.as_subsystem_base(),
            self.secondary_scanner.as_subsystem_base(),
            self.jump_drive.as_subsystem_base(),
        ]
        .into_iter()
    }

    pub(crate) fn iter_runtime_events(&self) -> impl Iterator<Item = FlattiverseEvent> {
        [
            self.nebula_collector.create_runtime_event(),
            self.main_scanner.create_runtime_event(),
            self.secondary_scanner.create_runtime_event(),
            self.engine.create_runtime_event(),
            self.shot_launcher.create_runtime_event(),
            self.shot_magazine.create_runtime_event(),
            self.shot_fabricator.create_runtime_event(),
            self.interceptor_launcher.create_runtime_event(),
            self.interceptor_magazine.create_runtime_event(),
            self.interceptor_fabricator.create_runtime_event(),
            self.railgun.create_runtime_event(),
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
                Ok(Controls::<ClassicShipControllable>::proven(p).control(controllable))
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
