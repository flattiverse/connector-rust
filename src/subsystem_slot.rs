use crate::network::{PacketReader, PacketWriter};
use crate::utils::{Readable, Writable};
use num_enum::FromPrimitive;

/// Identifies the concrete subsystem slot within a controllable.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum SubsystemSlot {
    /// Primary energy battery slot.
    EnergyBattery = 0x00,
    /// Primary ion battery slot.
    IonBattery = 0x01,
    /// Primary neutrino battery slot.
    NeutrinoBattery = 0x02,
    /// Primary energy-cell slot.
    EnergyCell = 0x10,
    /// Primary ion-cell slot.
    IonCell = 0x11,
    /// Primary neutrino-cell slot.
    NeutrinoCell = 0x12,
    /// Hull integrity slot.
    Hull = 0x18,
    /// Shield integrity slot.
    Shield = 0x19,
    /// Armor integrity slot.
    Armor = 0x1A,
    /// Hull repair slot.
    Repair = 0x1B,
    /// Cargo slot.
    Cargo = 0x50,
    /// Resource miner slot.
    ResourceMiner = 0x51,
    /// Nebula collector slot.
    NebulaCollector = 0x52,
    /// Structure optimizer slot.
    StructureOptimizer = 0x53,
    /// Primary scanner slot.
    PrimaryScanner = 0x20,
    /// Secondary scanner slot.
    SecondaryScanner = 0x21,
    /// Tertiary scanner slot.
    TertiaryScanner = 0x22,
    /// Modern scanner slot at the ship nose.
    ModernScannerN = 0x23,
    /// Modern scanner slot at the ship north-east mount.
    ModernScannerNE = 0x24,
    /// Modern scanner slot at the ship east mount.
    ModernScannerE = 0x25,
    /// Modern scanner slot at the ship south-east mount.
    ModernScannerSE = 0x26,
    /// Modern scanner slot at the ship stern.
    ModernScannerS = 0x27,
    /// Modern scanner slot at the ship south-west mount.
    ModernScannerSW = 0x28,
    /// Modern scanner slot at the ship west mount.
    ModernScannerW = 0x29,
    /// Modern scanner slot at the ship north-west mount.
    ModernScannerNW = 0x2A,
    /// Primary engine slot.
    PrimaryEngine = 0x30,
    /// Secondary engine slot.
    SecondaryEngine = 0x31,
    /// Tertiary engine slot.
    TertiaryEngine = 0x32,
    /// Jump-drive slot.
    JumpDrive = 0x33,
    /// Modern engine slot at the ship nose.
    ModernEngineN = 0x34,
    /// Modern engine slot at the ship north-east mount.
    ModernEngineNE = 0x35,
    /// Modern engine slot at the ship east mount.
    ModernEngineE = 0x36,
    /// Modern engine slot at the ship south-east mount.
    ModernEngineSE = 0x37,
    /// Modern engine slot at the ship stern.
    ModernEngineS = 0x38,
    /// Modern engine slot at the ship south-west mount.
    ModernEngineSW = 0x39,
    /// Modern engine slot at the ship west mount.
    ModernEngineW = 0x3A,
    /// Modern engine slot at the ship north-west mount.
    ModernEngineNW = 0x3B,
    /// Dynamic shot launcher slot.
    DynamicShotLauncher = 0x40,
    /// Dynamic shot magazine slot.
    DynamicShotMagazine = 0x41,
    /// Dynamic shot fabricator slot.
    DynamicShotFabricator = 0x42,
    /// Dynamic interceptor launcher slot.
    DynamicInterceptorLauncher = 0x43,
    /// Dynamic interceptor magazine slot.
    DynamicInterceptorMagazine = 0x44,
    /// Dynamic interceptor fabricator slot.
    DynamicInterceptorFabricator = 0x45,
    /// Railgun slot.
    Railgun = 0x46,
    /// Static shot launcher slot at the ship nose.
    StaticShotLauncherN = 0x60,
    /// Static shot launcher slot at the ship north-east mount.
    StaticShotLauncherNE = 0x61,
    /// Static shot launcher slot at the ship east mount.
    StaticShotLauncherE = 0x62,
    /// Static shot launcher slot at the ship south-east mount.
    StaticShotLauncherSE = 0x63,
    /// Static shot launcher slot at the ship stern.
    StaticShotLauncherS = 0x64,
    /// Static shot launcher slot at the ship south-west mount.
    StaticShotLauncherSW = 0x65,
    /// Static shot launcher slot at the ship west mount.
    StaticShotLauncherW = 0x66,
    /// Static shot launcher slot at the ship north-west mount.
    StaticShotLauncherNW = 0x67,
    /// Static shot magazine slot at the ship nose.
    StaticShotMagazineN = 0x68,
    /// Static shot magazine slot at the ship north-east mount.
    StaticShotMagazineNE = 0x69,
    /// Static shot magazine slot at the ship east mount.
    StaticShotMagazineE = 0x6A,
    /// Static shot magazine slot at the ship south-east mount.
    StaticShotMagazineSE = 0x6B,
    /// Static shot magazine slot at the ship stern.
    StaticShotMagazineS = 0x6C,
    /// Static shot magazine slot at the ship south-west mount.
    StaticShotMagazineSW = 0x6D,
    /// Static shot magazine slot at the ship west mount.
    StaticShotMagazineW = 0x6E,
    /// Static shot magazine slot at the ship north-west mount.
    StaticShotMagazineNW = 0x6F,
    /// Static shot fabricator slot at the ship nose.
    StaticShotFabricatorN = 0x70,
    /// Static shot fabricator slot at the ship north-east mount.
    StaticShotFabricatorNE = 0x71,
    /// Static shot fabricator slot at the ship east mount.
    StaticShotFabricatorE = 0x72,
    /// Static shot fabricator slot at the ship south-east mount.
    StaticShotFabricatorSE = 0x73,
    /// Static shot fabricator slot at the ship stern.
    StaticShotFabricatorS = 0x74,
    /// Static shot fabricator slot at the ship south-west mount.
    StaticShotFabricatorSW = 0x75,
    /// Static shot fabricator slot at the ship west mount.
    StaticShotFabricatorW = 0x76,
    /// Static shot fabricator slot at the ship north-west mount.
    StaticShotFabricatorNW = 0x77,
    /// Static interceptor launcher slot at the ship east mount.
    StaticInterceptorLauncherE = 0x78,
    /// Static interceptor launcher slot at the ship west mount.
    StaticInterceptorLauncherW = 0x79,
    /// Static interceptor magazine slot at the ship east mount.
    StaticInterceptorMagazineE = 0x7A,
    /// Static interceptor magazine slot at the ship west mount.
    StaticInterceptorMagazineW = 0x7B,
    /// Static interceptor fabricator slot at the ship east mount.
    StaticInterceptorFabricatorE = 0x7C,
    /// Static interceptor fabricator slot at the ship west mount.
    StaticInterceptorFabricatorW = 0x7D,
    /// Modern railgun slot at the ship nose.
    ModernRailgunN = 0x80,
    /// Modern railgun slot at the ship north-east mount.
    ModernRailgunNE = 0x81,
    /// Modern railgun slot at the ship east mount.
    ModernRailgunE = 0x82,
    /// Modern railgun slot at the ship south-east mount.
    ModernRailgunSE = 0x83,
    /// Modern railgun slot at the ship stern.
    ModernRailgunS = 0x84,
    /// Modern railgun slot at the ship south-west mount.
    ModernRailgunSW = 0x85,
    /// Modern railgun slot at the ship west mount.
    ModernRailgunW = 0x86,
    /// Modern railgun slot at the ship north-west mount.
    ModernRailgunNW = 0x87,

    /// The subsystem slot is unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Readable for SubsystemSlot {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self::from_primitive(reader.read_byte())
    }
}

impl Writable for SubsystemSlot {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_byte(u8::from(*self));
    }
}
