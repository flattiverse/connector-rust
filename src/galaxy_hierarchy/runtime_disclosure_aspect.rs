/// Session-level runtime automation aspects.
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
pub enum RuntimeDisclosureAspect {
    /// Thrust or engine-control decisions.
    EngineControl = 0,
    /// Pathing and movement-target selection.
    Navigation = 1,
    /// Scanner activation and scan-shape handling.
    ScannerControl = 2,
    /// Ballistic setup and aiming.
    WeaponAiming = 3,
    /// Weapon target selection.
    WeaponTargetSelection = 4,
    /// Runtime resource management such as energy allocation.
    ResourceControl = 5,
    /// Multi-ship fleet coordination.
    FleetControl = 6,
    /// Mission selection and assignment.
    MissionControl = 7,
    /// Loadout and subsystem setup choices.
    LoadoutControl = 8,
    /// Chat behavior.
    Chat = 9,
    #[num_enum(catch_all)]
    Unknown(u8),
}
