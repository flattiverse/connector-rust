/// Identifies the logical subsystem family independent of the concrete slot.
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
pub enum SubsystemKind {
    /// The main standard-energy battery.
    EnergyBattery,
    /// The ion battery.
    IonBattery,
    /// The neutrino battery.
    NeutrinoBattery,
    /// The standard-energy solar cell.
    EnergyCell,
    /// The ion cell.
    IonCell,
    /// The neutrino cell.
    NeutrinoCell,
    /// The hull subsystem.
    Hull,
    /// The shield subsystem.
    Shield,
    /// The armor subsystem.
    Armor,
    /// The repair subsystem.
    Repair,
    /// The cargo subsystem.
    Cargo,
    /// The resource-miner subsystem.
    ResourceMiner,
    /// The nebula-collector subsystem.
    NebulaCollector,
    /// The structure-optimizer subsystem.
    StructureOptimizer,
    /// The classic-ship engine family.
    ClassicShipEngine,
    /// The modern-ship engine family.
    ModernShipEngine,
    /// The dynamic scanner family for classic ships.
    DynamicScanner,
    /// The static scanner family for modern ships.
    StaticScanner,
    /// The classic dynamic shot launcher.
    DynamicShotLauncher,
    /// The modern static shot launcher.
    StaticShotLauncher,
    /// The classic dynamic shot magazine.
    DynamicShotMagazine,
    /// The modern static shot magazine.
    StaticShotMagazine,
    /// The classic dynamic shot fabricator.
    DynamicShotFabricator,
    /// The modern static shot fabricator.
    StaticShotFabricator,
    /// The classic dynamic interceptor launcher.
    DynamicInterceptorLauncher,
    /// The modern static interceptor launcher.
    StaticInterceptorLauncher,
    /// The classic dynamic interceptor magazine.
    DynamicInterceptorMagazine,
    /// The modern static interceptor magazine.
    StaticInterceptorMagazine,
    /// The classic dynamic interceptor fabricator.
    DynamicInterceptorFabricator,
    /// The modern static interceptor fabricator.
    StaticInterceptorFabricator,
    /// The classic railgun subsystem.
    ClassicRailgun,
    /// The modern railgun subsystem.
    ModernRailgun,
    /// The jump-drive subsystem.
    JumpDrive,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl SubsystemKind {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
