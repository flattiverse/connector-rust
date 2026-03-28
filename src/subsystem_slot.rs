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
    /// Primary energy-cell slot
    EnergyCell = 0x10,
    /// Primary ion-cell slot
    IonCell = 0x11,
    /// primary neutrino-cell slot.
    NeutrinoCell = 0x12,
    /// Hull integrity slot.
    Hull = 0x18,
    /// Shield integrity slot.
    Shield = 0x19,
    /// Primary scanner slot.
    PrimaryScanner = 0x20,
    /// Secondary scanner slot.
    SecondaryScanner = 0x21,
    /// Tertiary scanner slot.
    TertiaryScanner = 0x22,
    /// Primary energy slot.
    PrimaryEnergy = 0x30,
    /// Secondary energy slot.
    SecondaryEnergy = 0x31,
    /// Tertiary energy slot.
    TertiaryEnergy = 0x32,
    /// Dynamic shot launcher slot.
    DynamicShotLauncher = 0x40,
    /// Dynamic shot magazine slot.
    DynamicShotMagazine = 0x41,
    /// Dynamic shot fabricator slot.
    DynamicShotFabricator = 0x42,

    /// The subsystem slot is unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}
