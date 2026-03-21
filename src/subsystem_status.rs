use crate::network::{PacketReader, PacketWriter};
use crate::utils::{Atomar, Readable, Writable};
use num_enum::FromPrimitive;
use std::sync::atomic::{AtomicU8, Ordering};

/// Runtime state of a subsystem for the current server tick.
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
pub enum SubsystemStatus {
    /// The subsystem was off and therefore did not act.
    Off = 0x00,
    /// The subsystem was enabled and successfully performed its work.
    Worked = 0x01,
    /// The subsystem was enabled but failed, typically because resources were missing.
    Failed = 0x02,
    /// The subsystem is currently upgrading and therefore unavailble.
    Upgrading = 0x03,

    /// The subsystem status is unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Default for SubsystemStatus {
    #[inline]
    fn default() -> Self {
        Self::Off
    }
}

impl Atomar for SubsystemStatus {
    type Container = AtomicU8;

    #[inline]
    fn into_container(self) -> Self::Container {
        AtomicU8::from(u8::from(self))
    }

    #[inline]
    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.store(u8::from(self), ordering);
    }

    #[inline]
    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        SubsystemStatus::from_primitive(container.load(ordering))
    }
}

impl Readable for SubsystemStatus {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        SubsystemStatus::from_primitive(reader.read_byte())
    }
}

impl Writable for SubsystemStatus {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_byte(u8::from(*self));
    }
}
