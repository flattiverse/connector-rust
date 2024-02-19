use crate::network::{PacketReader, PacketWriter};
use crate::unit::UnitKind;

pub trait Configuration {
    fn read(&mut self, reader: &mut dyn PacketReader);
    fn write(&self, writer: &mut dyn PacketWriter);
    fn kind(&self) -> UnitKind;
}
