#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive,
)]
pub enum UnitKind {
    #[default]
    Sun = 0x00,
    BlackHole = 0x01,
    Planet = 0x04,
    Moon = 0x05,
    Meteroid = 0x06,
    PlayerUnit = 0xF0,
}
