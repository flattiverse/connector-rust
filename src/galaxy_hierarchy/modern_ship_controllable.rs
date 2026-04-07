use crate::galaxy_hierarchy::{
    AsSubsystemBase, JumpDriveSubsystem, ModernRailgunSubsystem, ModernShipEngineSubsystem,
    ModernShipGeometry, NebulaCollectorSubsystem, RailgunDirection,
    StaticInterceptorFabricatorSubsystem, StaticInterceptorLauncherSubsystem,
    StaticInterceptorMagazineSubsystem, StaticScannerSubsystem, StaticShotFabricatorSubsystem,
    StaticShotLauncherSubsystem, StaticShotMagazineSubsystem, SubsystemBase, SystemExtIntern,
};
use crate::network::PacketReader;
use crate::utils::{Also, Readable};
use crate::{FlattiverseEvent, SubsystemSlot, SubsystemStatus, Vector};
use std::sync::Weak;

/// Owner-side handle for one registered modern-ship controllable.
#[derive(Debug)]
pub struct ModernShipControllable {
    pub(crate) nebula_collector: NebulaCollectorSubsystem,
    pub(crate) engines: Vec<ModernShipEngineSubsystem>,
    pub(crate) scanners: Vec<StaticScannerSubsystem>,
    pub(crate) shot_launchers: Vec<StaticShotLauncherSubsystem>,
    pub(crate) shot_magazines: Vec<StaticShotMagazineSubsystem>,
    pub(crate) shot_fabricators: Vec<StaticShotFabricatorSubsystem>,
    pub(crate) interceptor_launchers: Vec<StaticInterceptorLauncherSubsystem>,
    pub(crate) interceptor_magazines: Vec<StaticInterceptorMagazineSubsystem>,
    pub(crate) interceptor_fabricators: Vec<StaticInterceptorFabricatorSubsystem>,
    pub(crate) railguns: Vec<ModernRailgunSubsystem>,
    pub(crate) jump_drive: JumpDriveSubsystem,
    pub(crate) equipped_crystals: [String; 3],
}

impl ModernShipControllable {
    pub(crate) fn new() -> Self {
        Self {
            nebula_collector: NebulaCollectorSubsystem::create_classic_ship_nebula_collector(
                Weak::default(),
            ),
            engines: Vec::new(),
            scanners: Vec::new(),
            shot_launchers: Vec::new(),
            shot_magazines: Vec::new(),
            shot_fabricators: Vec::new(),
            interceptor_launchers: Vec::new(),
            interceptor_magazines: Vec::new(),
            interceptor_fabricators: Vec::new(),
            railguns: Vec::new(),
            jump_drive: JumpDriveSubsystem::new(Weak::default(), true),
            equipped_crystals: core::array::repeat(String::new()),
        }
    }

    pub(crate) fn read_initial_state(&mut self, reader: &mut dyn PacketReader) {
        NebulaCollectorState::read(reader).update_runtime(&self.nebula_collector);

        self.scanners = ModernShipGeometry::SCANNER_SLOTS
            .into_iter()
            .map(|slot| ScannerState::read(reader).init(slot))
            .collect();

        self.engines = ModernShipGeometry::ENGINE_SLOTS
            .into_iter()
            .map(|slot| EngineState::read(reader).init(slot))
            .collect();

        for index in 0..ModernShipGeometry::SHOT_LAUNCHER_SLOTS.len() {
            self.shot_launchers
                .push(LauncherState::read(reader).init_shot(index));
            self.shot_magazines
                .push(MagazineState::read(reader).init_shot(index));
            self.shot_fabricators
                .push(FabricatorState::read(reader).init_shot(index));
        }

        for index in 0..2 {
            self.interceptor_launchers
                .push(LauncherState::read(reader).init_interceptor(index));
            self.interceptor_magazines
                .push(MagazineState::read(reader).init_interceptor(index));
            self.interceptor_fabricators
                .push(FabricatorState::read(reader).init_interceptor(index));
        }

        self.railguns = ModernShipGeometry::RAILGUN_SLOTS
            .into_iter()
            .map(|slot| RailgunState::read(reader).init(slot))
            .collect();

        self.jump_drive.set_exists(reader.read_byte() != 0x00);
        self.jump_drive.set_reported_tier(reader.read_byte());
        self.jump_drive.set_energy_cost(reader.read_f32());

        self.equipped_crystals[0] = reader.read_string();
        self.equipped_crystals[1] = reader.read_string();
        self.equipped_crystals[2] = reader.read_string();
    }

    pub(crate) fn iter_subsystem_bases(&self) -> impl Iterator<Item = &SubsystemBase> + '_ {
        [self.nebula_collector.as_subsystem_base()]
            .into_iter()
            .chain(self.engines.iter().map(|s| s.as_subsystem_base()))
            .chain(self.scanners.iter().map(|s| s.as_subsystem_base()))
            .chain(self.shot_launchers.iter().map(|s| s.as_subsystem_base()))
            .chain(self.shot_magazines.iter().map(|s| s.as_subsystem_base()))
            .chain(self.shot_fabricators.iter().map(|s| s.as_subsystem_base()))
            .chain(
                self.interceptor_launchers
                    .iter()
                    .map(|s| s.as_subsystem_base()),
            )
            .chain(
                self.interceptor_magazines
                    .iter()
                    .map(|s| s.as_subsystem_base()),
            )
            .chain(
                self.interceptor_fabricators
                    .iter()
                    .map(|s| s.as_subsystem_base()),
            )
            .chain(self.railguns.iter().map(|s| s.as_subsystem_base()))
            .chain([self.jump_drive.as_subsystem_base()])
    }

    #[inline]
    pub fn nebula_collector(&self) -> &NebulaCollectorSubsystem {
        &self.nebula_collector
    }

    #[inline]
    pub fn jump_drive(&self) -> &JumpDriveSubsystem {
        &self.jump_drive
    }

    #[inline]
    pub fn equipped_crystals(&self) -> &[String] {
        &self.equipped_crystals
    }

    #[inline]
    pub fn engines(&self) -> &[ModernShipEngineSubsystem] {
        &self.engines
    }

    #[inline]
    pub fn scanners(&self) -> &[StaticScannerSubsystem] {
        &self.scanners
    }

    #[inline]
    pub fn shot_launchers(&self) -> &[StaticShotLauncherSubsystem] {
        &self.shot_launchers
    }

    #[inline]
    pub fn shot_magazines(&self) -> &[StaticShotMagazineSubsystem] {
        &self.shot_magazines
    }

    #[inline]
    pub fn shot_fabricators(&self) -> &[StaticShotFabricatorSubsystem] {
        &self.shot_fabricators
    }

    #[inline]
    pub fn railguns(&self) -> &[ModernRailgunSubsystem] {
        &self.railguns
    }

    #[inline]
    pub fn engine_n(&self) -> &ModernShipEngineSubsystem {
        &self.engines[0]
    }

    #[inline]
    pub fn engine_ne(&self) -> &ModernShipEngineSubsystem {
        &self.engines[1]
    }

    #[inline]
    pub fn engine_e(&self) -> &ModernShipEngineSubsystem {
        &self.engines[2]
    }

    #[inline]
    pub fn engine_se(&self) -> &ModernShipEngineSubsystem {
        &self.engines[3]
    }

    #[inline]
    pub fn engine_s(&self) -> &ModernShipEngineSubsystem {
        &self.engines[4]
    }

    #[inline]
    pub fn engine_sw(&self) -> &ModernShipEngineSubsystem {
        &self.engines[5]
    }

    #[inline]
    pub fn engine_w(&self) -> &ModernShipEngineSubsystem {
        &self.engines[6]
    }

    #[inline]
    pub fn engine_nw(&self) -> &ModernShipEngineSubsystem {
        &self.engines[7]
    }

    #[inline]
    pub fn scanner_n(&self) -> &StaticScannerSubsystem {
        &self.scanners[0]
    }

    #[inline]
    pub fn scanner_ne(&self) -> &StaticScannerSubsystem {
        &self.scanners[1]
    }

    #[inline]
    pub fn scanner_e(&self) -> &StaticScannerSubsystem {
        &self.scanners[2]
    }

    #[inline]
    pub fn scanner_se(&self) -> &StaticScannerSubsystem {
        &self.scanners[3]
    }

    #[inline]
    pub fn scanner_s(&self) -> &StaticScannerSubsystem {
        &self.scanners[4]
    }

    #[inline]
    pub fn scanner_sw(&self) -> &StaticScannerSubsystem {
        &self.scanners[5]
    }

    #[inline]
    pub fn scanner_w(&self) -> &StaticScannerSubsystem {
        &self.scanners[6]
    }

    #[inline]
    pub fn scanner_nw(&self) -> &StaticScannerSubsystem {
        &self.scanners[7]
    }

    #[inline]
    pub fn shot_launcher_n(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[0]
    }

    #[inline]
    pub fn shot_launcher_ne(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[1]
    }

    #[inline]
    pub fn shot_launcher_e(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[2]
    }

    #[inline]
    pub fn shot_launcher_se(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[3]
    }

    #[inline]
    pub fn shot_launcher_s(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[4]
    }

    #[inline]
    pub fn shot_launcher_sw(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[5]
    }

    #[inline]
    pub fn shot_launcher_w(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[6]
    }

    #[inline]
    pub fn shot_launcher_nw(&self) -> &StaticShotLauncherSubsystem {
        &self.shot_launchers[7]
    }

    #[inline]
    pub fn shot_magazine_n(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[0]
    }

    #[inline]
    pub fn shot_magazine_ne(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[1]
    }

    #[inline]
    pub fn shot_magazine_e(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[2]
    }

    #[inline]
    pub fn shot_magazine_se(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[3]
    }

    #[inline]
    pub fn shot_magazine_s(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[4]
    }

    #[inline]
    pub fn shot_magazine_sw(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[5]
    }

    #[inline]
    pub fn shot_magazine_w(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[6]
    }

    #[inline]
    pub fn shot_magazine_nw(&self) -> &StaticShotMagazineSubsystem {
        &self.shot_magazines[7]
    }

    #[inline]
    pub fn shot_fabricator_n(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[0]
    }

    #[inline]
    pub fn shot_fabricator_ne(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[1]
    }

    #[inline]
    pub fn shot_fabricator_e(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[2]
    }

    #[inline]
    pub fn shot_fabricator_se(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[3]
    }

    #[inline]
    pub fn shot_fabricator_s(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[4]
    }

    #[inline]
    pub fn shot_fabricator_sw(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[5]
    }

    #[inline]
    pub fn shot_fabricator_w(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[6]
    }

    #[inline]
    pub fn shot_fabricator_nw(&self) -> &StaticShotFabricatorSubsystem {
        &self.shot_fabricators[7]
    }

    #[inline]
    pub fn interceptor_launcher_e(&self) -> &StaticInterceptorLauncherSubsystem {
        &self.interceptor_launchers[0]
    }

    #[inline]
    pub fn interceptor_launcher_w(&self) -> &StaticInterceptorLauncherSubsystem {
        &self.interceptor_launchers[1]
    }

    #[inline]
    pub fn interceptor_magazine_e(&self) -> &StaticInterceptorMagazineSubsystem {
        &self.interceptor_magazines[0]
    }

    #[inline]
    pub fn interceptor_magazine_w(&self) -> &StaticInterceptorMagazineSubsystem {
        &self.interceptor_magazines[1]
    }

    #[inline]
    pub fn interceptor_fabricator_e(&self) -> &StaticInterceptorFabricatorSubsystem {
        &self.interceptor_fabricators[0]
    }

    #[inline]
    pub fn interceptor_fabricator_w(&self) -> &StaticInterceptorFabricatorSubsystem {
        &self.interceptor_fabricators[1]
    }

    #[inline]
    pub fn railgun_n(&self) -> &ModernRailgunSubsystem {
        &self.railguns[0]
    }

    #[inline]
    pub fn railgun_ne(&self) -> &ModernRailgunSubsystem {
        &self.railguns[1]
    }

    #[inline]
    pub fn railgun_e(&self) -> &ModernRailgunSubsystem {
        &self.railguns[2]
    }

    #[inline]
    pub fn railgun_se(&self) -> &ModernRailgunSubsystem {
        &self.railguns[3]
    }

    #[inline]
    pub fn railgun_s(&self) -> &ModernRailgunSubsystem {
        &self.railguns[4]
    }

    #[inline]
    pub fn railgun_sw(&self) -> &ModernRailgunSubsystem {
        &self.railguns[5]
    }

    #[inline]
    pub fn railgun_w(&self) -> &ModernRailgunSubsystem {
        &self.railguns[6]
    }

    #[inline]
    pub fn railgun_nw(&self) -> &ModernRailgunSubsystem {
        &self.railguns[7]
    }

    // TODO pub fn get_projected_raw_structural_load()

    pub(crate) fn reset_runtime(&self) {
        self.nebula_collector.reset_runtime();

        self.engines.iter().for_each(|s| s.reset_runtime());
        self.scanners.iter().for_each(|s| s.reset_runtime());

        self.shot_launchers.iter().for_each(|s| s.reset_runtime());
        self.shot_magazines.iter().for_each(|s| s.reset_runtime());
        self.shot_fabricators.iter().for_each(|s| s.reset_runtime());

        self.interceptor_launchers
            .iter()
            .for_each(|s| s.reset_runtime());
        self.interceptor_magazines
            .iter()
            .for_each(|s| s.reset_runtime());
        self.interceptor_fabricators
            .iter()
            .for_each(|s| s.reset_runtime());

        self.railguns.iter().for_each(|s| s.reset_runtime());

        self.jump_drive.reset_runtime();
    }

    pub(crate) fn read_runtime(&self, reader: &mut dyn PacketReader) {
        NebulaCollectorState::read(reader).update_runtime(&self.nebula_collector);

        for scanner in &self.scanners {
            ScannerState::read(reader).update_runtime(scanner);
        }

        for engine in &self.engines {
            EngineState::read(reader).update_runtime(engine);
        }

        for index in 0..ModernShipGeometry::SHOT_LAUNCHER_SLOTS.len() {
            LauncherState::read(reader).update_shot_runtime(&self.shot_launchers[index]);
            MagazineState::read(reader).update_shot_runtime(&self.shot_magazines[index]);
            FabricatorState::read(reader).update_shot_runtime(&self.shot_fabricators[index])
        }

        for index in 0..2 {
            LauncherState::read(reader)
                .update_interceptor_runtime(&self.interceptor_launchers[index]);
            MagazineState::read(reader)
                .update_interceptor_runtime(&self.interceptor_magazines[index]);
            FabricatorState::read(reader)
                .update_interceptor_runtime(&self.interceptor_fabricators[index])
        }

        for railgun in &self.railguns {
            RailgunState::read(reader).update_runtime(railgun);
        }

        self.jump_drive.update_runtime(
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
    }

    pub(crate) fn iter_runtime_events(&self) -> impl Iterator<Item = FlattiverseEvent> + '_ {
        [self.nebula_collector.create_runtime_event()]
            .into_iter()
            .chain(self.scanners.iter().map(|s| s.create_runtime_event()))
            .chain(self.engines.iter().map(|s| s.create_runtime_event()))
            .chain(self.shot_launchers.iter().map(|s| s.create_runtime_event()))
            .chain(self.shot_magazines.iter().map(|s| s.create_runtime_event()))
            .chain(
                self.shot_fabricators
                    .iter()
                    .map(|s| s.create_runtime_event()),
            )
            .chain(
                self.interceptor_launchers
                    .iter()
                    .map(|s| s.create_runtime_event()),
            )
            .chain(
                self.interceptor_magazines
                    .iter()
                    .map(|s| s.create_runtime_event()),
            )
            .chain(
                self.interceptor_fabricators
                    .iter()
                    .map(|s| s.create_runtime_event()),
            )
            .chain(self.railguns.iter().map(|s| s.create_runtime_event()))
            .flatten()
    }
}

struct NebulaCollectorState {
    exists: bool,
    tier: u8,
    minimum_rate: f32,
    maximum_rate: f32,
    rate: f32,
    status: SubsystemStatus,
    consumed_energy_this_tick: f32,
    consumed_ions_this_tick: f32,
    consumed_neutrinos_this_tick: f32,
    collected_this_tick: f32,
    collected_hue_this_tick: f32,
}

impl NebulaCollectorState {
    fn update_runtime(self, collector: &NebulaCollectorSubsystem) {
        collector.set_exists(self.exists);
        collector.set_capabilities(self.minimum_rate, self.maximum_rate);
        collector.update_runtime(
            self.rate,
            self.status,
            self.consumed_energy_this_tick,
            self.consumed_ions_this_tick,
            self.consumed_neutrinos_this_tick,
            self.collected_this_tick,
            self.collected_hue_this_tick,
        );
        collector.set_reported_tier(self.tier);
    }
}

impl Readable for NebulaCollectorState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            minimum_rate: reader.read_f32(),
            maximum_rate: reader.read_f32(),
            rate: reader.read_f32(),
            status: SubsystemStatus::read(reader),
            consumed_energy_this_tick: reader.read_f32(),
            consumed_ions_this_tick: reader.read_f32(),
            consumed_neutrinos_this_tick: reader.read_f32(),
            collected_this_tick: reader.read_f32(),
            collected_hue_this_tick: reader.read_f32(),
        }
    }
}

struct ScannerState {
    exists: bool,
    tier: u8,
    maximum_width: f32,
    maximum_length: f32,
    width_speed: f32,
    length_speed: f32,
    angle_speed: f32,
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
}

impl ScannerState {
    fn init(&self, slot: SubsystemSlot) -> StaticScannerSubsystem {
        StaticScannerSubsystem::new(
            Weak::default(),
            format!("Scanner{}", slot_suffix(slot)),
            self.exists,
            self.maximum_width,
            self.maximum_length,
            self.width_speed,
            self.length_speed,
            self.angle_speed,
            slot,
        )
        .also(|scanner| self.update_runtime(scanner))
    }

    #[inline]
    fn update_runtime(&self, scanner: &StaticScannerSubsystem) {
        scanner.update_runtime(
            self.active,
            self.current_width,
            self.current_length,
            self.current_angle,
            self.target_width,
            self.target_length,
            self.target_angle,
            self.status,
            self.consumed_energy_this_tick,
            self.consumed_ions_this_tick,
            self.consumed_neutrinos_this_tick,
        );
        scanner.set_reported_tier(self.tier);
    }
}

impl Readable for ScannerState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            maximum_width: reader.read_f32(),
            maximum_length: reader.read_f32(),
            width_speed: reader.read_f32(),
            length_speed: reader.read_f32(),
            angle_speed: reader.read_f32(),
            active: reader.read_byte() != 0x00,
            current_width: reader.read_f32(),
            current_length: reader.read_f32(),
            current_angle: reader.read_f32(),
            target_width: reader.read_f32(),
            target_length: reader.read_f32(),
            target_angle: reader.read_f32(),
            status: SubsystemStatus::read(reader),
            consumed_energy_this_tick: reader.read_f32(),
            consumed_ions_this_tick: reader.read_f32(),
            consumed_neutrinos_this_tick: reader.read_f32(),
        }
    }
}

struct EngineState {
    exists: bool,
    tier: u8,
    maximum_forward_thrust: f32,
    maximum_reverse_thrust: f32,
    maximum_thrust_change_per_tick: f32,
    current_thrust: f32,
    target_thrust: f32,
    status: SubsystemStatus,
    consumed_energy_this_tick: f32,
    consumed_ions_this_tick: f32,
    consumed_neutrinos_this_tick: f32,
}

impl EngineState {
    fn init(&self, slot: SubsystemSlot) -> ModernShipEngineSubsystem {
        ModernShipEngineSubsystem::new(
            Weak::default(),
            format!("Engine{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|engine| {
            engine.set_capabilities(
                self.maximum_forward_thrust,
                self.maximum_reverse_thrust,
                self.maximum_thrust_change_per_tick,
            );
            self.update_runtime(engine);
        })
    }

    fn update_runtime(&self, engine: &ModernShipEngineSubsystem) {
        engine.update_runtime(
            self.current_thrust,
            self.target_thrust,
            self.status,
            self.consumed_energy_this_tick,
            self.consumed_ions_this_tick,
            self.consumed_neutrinos_this_tick,
        );
        engine.set_reported_tier(self.tier);
    }
}

impl Readable for EngineState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            maximum_forward_thrust: reader.read_f32(),
            maximum_reverse_thrust: reader.read_f32(),
            maximum_thrust_change_per_tick: reader.read_f32(),
            current_thrust: reader.read_f32(),
            target_thrust: reader.read_f32(),
            status: SubsystemStatus::read(reader),
            consumed_energy_this_tick: reader.read_f32(),
            consumed_ions_this_tick: reader.read_f32(),
            consumed_neutrinos_this_tick: reader.read_f32(),
        }
    }
}

struct LauncherState {
    exists: bool,
    tier: u8,
    minimum_relative_movement: f32,
    maximum_relative_movement: f32,
    minimum_ticks: u16,
    maximum_ticks: u16,
    minimum_load: f32,
    maximum_load: f32,
    minimum_damage: f32,
    maximum_damage: f32,
    relative_movement: Vector,
    ticks: u16,
    load: f32,
    damage: f32,
    status: SubsystemStatus,
    consumed_energy_this_tick: f32,
    consumed_ions_this_tick: f32,
    consumed_neutrinos_this_tick: f32,
}

impl LauncherState {
    fn init_shot(self, index: usize) -> StaticShotLauncherSubsystem {
        let slot = ModernShipGeometry::SHOT_LAUNCHER_SLOTS[index];
        StaticShotLauncherSubsystem::new(
            Weak::default(),
            format!("ShotLauncher{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|launcher| {
            self.update_shot_runtime(launcher);
        })
    }

    fn init_interceptor(self, index: usize) -> StaticInterceptorLauncherSubsystem {
        let slot = [
            SubsystemSlot::StaticInterceptorLauncherE,
            SubsystemSlot::StaticInterceptorLauncherW,
        ][index];
        StaticInterceptorLauncherSubsystem::new(
            Weak::default(),
            format!("InterceptorLauncher{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|launcher| {
            self.update_interceptor_runtime(launcher);
        })
    }

    #[inline]
    fn update_shot_runtime(&self, launcher: &StaticShotLauncherSubsystem) {
        self.update_capabilities(launcher, StaticShotLauncherSubsystem::set_capabilities);
        self.update_runtime(launcher, StaticShotLauncherSubsystem::update_runtime);
        launcher.set_reported_tier(self.tier);
    }

    #[inline]
    fn update_interceptor_runtime(&self, launcher: &StaticInterceptorLauncherSubsystem) {
        self.update_capabilities(
            launcher,
            StaticInterceptorLauncherSubsystem::set_capabilities,
        );
        self.update_runtime(launcher, StaticInterceptorLauncherSubsystem::update_runtime);
        launcher.set_reported_tier(self.tier);
    }

    #[inline]
    fn update_runtime<T>(
        &self,
        it: &T,
        update_fn: impl Fn(&T, Vector, u16, f32, f32, SubsystemStatus, f32, f32, f32),
    ) {
        update_fn(
            it,
            self.relative_movement,
            self.ticks,
            self.load,
            self.damage,
            self.status,
            self.consumed_energy_this_tick,
            self.consumed_ions_this_tick,
            self.consumed_neutrinos_this_tick,
        );
    }

    #[inline]
    fn update_capabilities<T>(
        &self,
        it: &T,
        capabilities_fn: impl Fn(&T, f32, f32, u16, u16, f32, f32, f32, f32),
    ) {
        capabilities_fn(
            it,
            self.minimum_relative_movement,
            self.maximum_relative_movement,
            self.minimum_ticks,
            self.maximum_ticks,
            self.minimum_load,
            self.maximum_load,
            self.minimum_damage,
            self.maximum_damage,
        )
    }
}

impl Readable for LauncherState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            minimum_relative_movement: reader.read_f32(),
            maximum_relative_movement: reader.read_f32(),
            minimum_ticks: reader.read_uint16(),
            maximum_ticks: reader.read_uint16(),
            minimum_load: reader.read_f32(),
            maximum_load: reader.read_f32(),
            minimum_damage: reader.read_f32(),
            maximum_damage: reader.read_f32(),
            relative_movement: Vector::from_read(reader),
            ticks: reader.read_uint16(),
            load: reader.read_f32(),
            damage: reader.read_f32(),
            status: SubsystemStatus::read(reader),
            consumed_energy_this_tick: reader.read_f32(),
            consumed_ions_this_tick: reader.read_f32(),
            consumed_neutrinos_this_tick: reader.read_f32(),
        }
    }
}

struct MagazineState {
    exists: bool,
    tier: u8,
    maximum_shots: f32,
    current_shots: f32,
    status: SubsystemStatus,
}

impl MagazineState {
    fn init_shot(self, index: usize) -> StaticShotMagazineSubsystem {
        let slot = ModernShipGeometry::SHOT_MAGAZINE_SLOTS[index];
        StaticShotMagazineSubsystem::new(
            Weak::default(),
            format!("ShotMagazine{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|magazine| {
            self.update_shot_runtime(magazine);
        })
    }

    fn init_interceptor(self, index: usize) -> StaticInterceptorMagazineSubsystem {
        let slot = [
            SubsystemSlot::StaticInterceptorMagazineE,
            SubsystemSlot::StaticInterceptorMagazineW,
        ][index];
        StaticInterceptorMagazineSubsystem::new(
            Weak::default(),
            format!("InterceptorMagazine{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|magazine| {
            self.update_interceptor_runtime(magazine);
        })
    }

    #[inline]
    fn update_shot_runtime(&self, launcher: &StaticShotMagazineSubsystem) {
        self.update_maximum_shots(launcher, StaticShotMagazineSubsystem::set_maximum_shots);
        self.update_runtime(launcher, StaticShotMagazineSubsystem::update_runtime);
        launcher.set_reported_tier(self.tier);
    }

    #[inline]
    fn update_interceptor_runtime(&self, launcher: &StaticInterceptorMagazineSubsystem) {
        self.update_maximum_shots(
            launcher,
            StaticInterceptorMagazineSubsystem::set_maximum_shots,
        );
        self.update_runtime(launcher, StaticInterceptorMagazineSubsystem::update_runtime);
        launcher.set_reported_tier(self.tier);
    }

    #[inline]
    fn update_runtime<T>(&self, it: &T, update_fn: impl Fn(&T, f32, SubsystemStatus)) {
        update_fn(it, self.current_shots, self.status);
    }

    fn update_maximum_shots<T>(&self, it: &T, set_fn: impl Fn(&T, f32)) {
        set_fn(it, self.maximum_shots);
    }
}

impl Readable for MagazineState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            maximum_shots: reader.read_f32(),
            current_shots: reader.read_f32(),
            status: SubsystemStatus::read(reader),
        }
    }
}

struct FabricatorState {
    exists: bool,
    tier: u8,
    minimum_rate: f32,
    maximum_rate: f32,
    active: bool,
    rate: f32,
    status: SubsystemStatus,
    consumed_energy_this_tick: f32,
    consumed_ions_this_tick: f32,
    consumed_neutrinos_this_tick: f32,
}

impl FabricatorState {
    fn init_shot(self, index: usize) -> StaticShotFabricatorSubsystem {
        let slot = ModernShipGeometry::SHOT_MAGAZINE_SLOTS[index];
        StaticShotFabricatorSubsystem::new(
            Weak::default(),
            format!("ShotFabricator{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|fabricator| {
            fabricator.set_maximum_rate(self.maximum_rate);
            self.update_shot_runtime(fabricator);
        })
    }

    fn init_interceptor(self, index: usize) -> StaticInterceptorFabricatorSubsystem {
        let slot = [
            SubsystemSlot::StaticInterceptorFabricatorE,
            SubsystemSlot::StaticInterceptorFabricatorW,
        ][index];
        StaticInterceptorFabricatorSubsystem::new(
            Weak::default(),
            format!("InterceptorFabricator{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|fabricator| {
            fabricator.set_maximum_rate(self.maximum_rate);
            self.update_interceptor_runtime(fabricator);
        })
    }

    #[inline]
    fn update_shot_runtime(&self, launcher: &StaticShotFabricatorSubsystem) {
        self.update_runtime(launcher, StaticShotFabricatorSubsystem::update_runtime);
        launcher.set_reported_tier(self.tier);
    }

    #[inline]
    fn update_interceptor_runtime(&self, launcher: &StaticInterceptorFabricatorSubsystem) {
        self.update_runtime(
            launcher,
            StaticInterceptorFabricatorSubsystem::update_runtime,
        );
        launcher.set_reported_tier(self.tier);
    }

    #[inline]
    fn update_runtime<T>(
        &self,
        it: &T,
        update_fn: impl Fn(&T, bool, f32, SubsystemStatus, f32, f32, f32),
    ) {
        update_fn(
            it,
            self.active,
            self.rate,
            self.status,
            self.consumed_energy_this_tick,
            self.consumed_ions_this_tick,
            self.consumed_neutrinos_this_tick,
        );
    }
}

impl Readable for FabricatorState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            minimum_rate: reader.read_f32(),
            maximum_rate: reader.read_f32(),
            active: reader.read_byte() != 0x00,
            rate: reader.read_f32(),
            status: SubsystemStatus::read(reader),
            consumed_energy_this_tick: reader.read_f32(),
            consumed_ions_this_tick: reader.read_f32(),
            consumed_neutrinos_this_tick: reader.read_f32(),
        }
    }
}

struct RailgunState {
    exists: bool,
    tier: u8,
    projectile_speed: f32,
    projectile_lifetime: u16,
    energy_cost: f32,
    metal_cost: f32,
    direction: RailgunDirection,
    status: SubsystemStatus,
    consumed_energy_this_tick: f32,
    consumed_ions_this_tick: f32,
    consumed_neutrinos_this_tick: f32,
}

impl RailgunState {
    fn init(&self, slot: SubsystemSlot) -> ModernRailgunSubsystem {
        ModernRailgunSubsystem::new(
            Weak::default(),
            format!("RailgunState{}", slot_suffix(slot)),
            self.exists,
            slot,
        )
        .also(|railgun| {
            self.update_runtime(railgun);
        })
    }

    fn update_runtime(&self, railgun: &ModernRailgunSubsystem) {
        railgun.set_capabilities(
            self.projectile_speed,
            self.projectile_lifetime,
            self.energy_cost,
            self.metal_cost,
        );
        railgun.update_runtime(
            self.direction,
            self.status,
            self.consumed_energy_this_tick,
            self.consumed_ions_this_tick,
            self.consumed_neutrinos_this_tick,
        );
        railgun.set_reported_tier(self.tier);
    }
}

impl Readable for RailgunState {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            exists: reader.read_byte() != 0x00,
            tier: reader.read_byte(),
            projectile_speed: reader.read_f32(),
            projectile_lifetime: reader.read_uint16(),
            energy_cost: reader.read_f32(),
            metal_cost: reader.read_f32(),
            direction: RailgunDirection::read(reader),
            status: SubsystemStatus::read(reader),
            consumed_energy_this_tick: reader.read_f32(),
            consumed_ions_this_tick: reader.read_f32(),
            consumed_neutrinos_this_tick: reader.read_f32(),
        }
    }
}

fn slot_suffix(slot: SubsystemSlot) -> &'static str {
    match slot {
        SubsystemSlot::ModernEngineN
        | SubsystemSlot::ModernScannerN
        | SubsystemSlot::StaticShotLauncherN
        | SubsystemSlot::StaticShotMagazineN
        | SubsystemSlot::StaticShotFabricatorN
        | SubsystemSlot::ModernRailgunN => "N",
        SubsystemSlot::ModernEngineNE
        | SubsystemSlot::ModernScannerNE
        | SubsystemSlot::StaticShotLauncherNE
        | SubsystemSlot::StaticShotMagazineNE
        | SubsystemSlot::StaticShotFabricatorNE
        | SubsystemSlot::ModernRailgunNE => "NE",
        SubsystemSlot::ModernEngineE
        | SubsystemSlot::ModernScannerE
        | SubsystemSlot::StaticShotLauncherE
        | SubsystemSlot::StaticShotMagazineE
        | SubsystemSlot::StaticShotFabricatorE
        | SubsystemSlot::StaticInterceptorLauncherE
        | SubsystemSlot::StaticInterceptorMagazineE
        | SubsystemSlot::StaticInterceptorFabricatorE
        | SubsystemSlot::ModernRailgunE => "E",
        SubsystemSlot::ModernEngineSE
        | SubsystemSlot::ModernScannerSE
        | SubsystemSlot::StaticShotLauncherSE
        | SubsystemSlot::StaticShotMagazineSE
        | SubsystemSlot::StaticShotFabricatorSE
        | SubsystemSlot::ModernRailgunSE => "SE",
        SubsystemSlot::ModernEngineS
        | SubsystemSlot::ModernScannerS
        | SubsystemSlot::StaticShotLauncherS
        | SubsystemSlot::StaticShotMagazineS
        | SubsystemSlot::StaticShotFabricatorS
        | SubsystemSlot::ModernRailgunS => "S",
        SubsystemSlot::ModernEngineSW
        | SubsystemSlot::ModernScannerSW
        | SubsystemSlot::StaticShotLauncherSW
        | SubsystemSlot::StaticShotMagazineSW
        | SubsystemSlot::StaticShotFabricatorSW
        | SubsystemSlot::ModernRailgunSW => "SW",
        SubsystemSlot::ModernEngineW
        | SubsystemSlot::ModernScannerW
        | SubsystemSlot::StaticShotLauncherW
        | SubsystemSlot::StaticShotMagazineW
        | SubsystemSlot::StaticShotFabricatorW
        | SubsystemSlot::StaticInterceptorLauncherW
        | SubsystemSlot::StaticInterceptorMagazineW
        | SubsystemSlot::StaticInterceptorFabricatorW
        | SubsystemSlot::ModernRailgunW => "W",
        SubsystemSlot::ModernEngineNW
        | SubsystemSlot::ModernScannerNW
        | SubsystemSlot::StaticShotLauncherNW
        | SubsystemSlot::StaticShotMagazineNW
        | SubsystemSlot::StaticShotFabricatorNW
        | SubsystemSlot::ModernRailgunNW => "NW",
        _ => unreachable!(),
    }
}
