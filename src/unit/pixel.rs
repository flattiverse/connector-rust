
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Color;
use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(Pixel);
pub trait Pixel : Unit {

    fn color(&self) -> &Color;

    fn kind(&self) -> UnitKind {
        UnitKind::Pixel
    }
}

pub struct PixelData {
    unit:   UnitData,
    color:  Color,
}

impl PixelData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PixelData, Error> {
        Ok(PixelData {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            color:  Color::from_rgb(
                reader.read_unsigned_byte()? as f32 / 255_f32,
                reader.read_unsigned_byte()? as f32 / 255_f32,
                reader.read_unsigned_byte()? as f32 / 255_f32,
            ),
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for PixelData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for PixelData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<PixelData> + BorrowMut<PixelData> + Unit> Pixel for  T {
    fn color(&self) -> &Color {
        &self.borrow().color
    }
}