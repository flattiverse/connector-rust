use crate::network::{PacketReader, PacketWriter};
use crate::unit::UnitKind;

pub trait Configuration {
    fn unit_name(&self) -> &str;

    #[inline]
    fn with_read(mut self, reader: &mut dyn PacketReader) -> Self
    where
        Self: Sized,
    {
        self.read(reader);
        self
    }

    fn read(&mut self, reader: &mut dyn PacketReader);
    fn write(&self, writer: &mut dyn PacketWriter);
    fn kind(&self) -> UnitKind;
}
