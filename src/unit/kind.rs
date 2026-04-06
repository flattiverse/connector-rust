use crate::network::{PacketReader, PacketWriter};
use crate::utils::{Readable, Writable};
use num_enum::FromPrimitive;

/// Specifies of which kind a unit is.
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
pub enum UnitKind {
    /// Represents a sun.
    Sun = 0x00,
    /// Represents a black hole.
    BlackHole = 0x01,
    /// Represents a current field that induces movement on mobile units.
    CurrentField = 0x02,
    /// Represents a nebula.
    Nebula = 0x03,
    /// Represents a storm source that periodically spawns whirls.
    Storm = 0x20,
    /// Represents a storm whirl that is still announcing itself.
    StormCommencingWhirl = 0x21,
    /// Represents an active storm whirl.
    StormActiveWhirl = 0x22,
    /// Represents a planet.
    Planet = 0x08,
    /// Represents a moon.
    Moon = 0x09,
    /// Represents a meteoroid.
    Meteoroid = 0x0A,
    /// Represents a buoy.
    Buoy = 0x10,
    /// Represents a worm-hole.
    WormHole = 0x11,
    /// Represents a mission target with configurable waypoint vectors.
    MissionTarget = 0x14,
    /// Represents a flag target.
    Flag = 0x15,
    /// Represents a domination point target.
    DominationPoint = 0x16,
    /// Represents an energy charge power-up.
    EnergyChargePowerUp = 0x70,
    /// Represents an ion charge power-up.
    IonChargePowerUp = 0x71,
    /// Represents a neutrino charge power-up.
    NeutrinoChargePowerUp = 0x72,
    /// Represents a metal cargo power-up.
    MetalCargoPowerUp = 0x73,
    /// Represents a carbon cargo power-up.
    CarbonCargoPowerUp = 0x74,
    /// Represents a hydrogen cargo power-up.
    HydrogenCargoPowerUp = 0x75,
    /// Represents a silicon cargo power-up.
    SiliconCargoPowerUp = 0x76,
    /// Represents a shield charge power-up.
    ShieldChargePowerUp = 0x77,
    /// Represents a hull repair power-up.
    HullRepairPowerUp = 0x78,
    /// Represents a shot charge power-up.
    ShotChargePowerUp = 0x79,
    /// Represents a switch that can affect gates.
    Switch = 0x60,
    /// Represents a gate that can open and close.
    Gate = 0x61,
    /// Represents a shot.
    Shot = 0xE0,
    /// Represents an interceptor projectile.
    Interceptor = 0xE1,
    /// Represents a rail projectile.
    Rail = 0xE2,
    /// Represents a classical player ship.
    ClassicShipPlayerUnit = 0xF0,
    /// Represents a new style player ship.
    ModernShipPlayerUnit = 0xF1,
    /// Represents an interceptor explosion.
    InterceptorExplosion = 0xFE,
    /// Explosion unit.
    Explosion = 0xFF,
    #[num_enum(catch_all)]
    Unknown(u8) = 0xAF,
}

impl UnitKind {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

impl Readable for UnitKind {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self::from_primitive(reader.read_byte())
    }
}

impl Writable for UnitKind {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_byte(u8::from(*self))
    }
}
