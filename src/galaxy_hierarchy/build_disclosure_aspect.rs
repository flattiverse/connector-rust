/// Session-level build/disclosure aspects.
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
pub enum BuildDisclosureAspect {
    /// Software architecture and design work.
    SoftwareDesign = 0,
    /// User interface work.
    UI = 1,
    /// Universe and scene rendering.
    UniverseRendering = 2,
    /// Input handling.
    Input = 3,
    /// Engine-control implementation work.
    EngineControl = 4,
    /// Navigation implementation work.
    Navigation = 5,
    /// Scanner-control implementation work.
    ScannerControl = 6,
    /// Weapon-system implementation work.
    WeaponSystems = 7,
    /// Resource-control implementation work.
    ResourceControl = 8,
    /// Fleet-control implementation work.
    FleetControl = 9,
    /// Mission-control implementation work.
    MissionControl = 10,
    /// Chat implementation work.
    Chat = 11,
    #[num_enum(catch_all)]
    Unknown(u8),
}
