
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Color;
use Error;
use Vector;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use unit::Mobility;
use net::Packet;
use net::BinaryReader;

downcast!(Pixel);
pub trait Pixel : Unit {

    fn color(&self) -> &Color;

    fn relevant(&self) -> bool {
        let color = self.color();
        color.red() > 0_f32 || color.green() > 0_f32 || color.blue() != 1_f32
    }
}

pub struct PixelData {
    unit:   UnitData,
    color:  Color,
}

impl PixelData {
    pub fn new(connector: &Arc<Connector>, group: &UniverseGroup, name: String, radius: f32,
               position: Vector, r: u8, g: u8, b: u8) -> PixelData {
        PixelData {
            unit: UnitData::new(
                connector,
                group,
                name,
                radius,
                0_f32,
                position,
                Vector::new(0_f32, 0_f32),
                false,
                false,
                true,
                Mobility::Still,
                UnitKind::Pixel
            ),
            color: Color::from_rgb(
                r as f32 / 255_f32,
                g as f32 / 255_f32,
                b as f32 / 255_f32,
            )
        }
    }

    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PixelData, Error> {
        Ok(PixelData {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::Pixel)?,
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