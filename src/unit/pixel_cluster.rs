
use std::io::Read;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

use flate2::read::GzDecoder;

impl_downcast!(PixelCluster);
pub trait PixelCluster : Unit {

    fn data(&self) -> &Vec<u8>;

    fn kind(&self) -> UnitKind {
        UnitKind::PixelCluster
    }
}

pub struct PixelClusterData {
    unit: UnitData,
    data: Vec<u8>,
}

impl PixelClusterData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PixelClusterData, Error> {
        Ok(PixelClusterData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?,
            data: {
                let count = reader.read_unsigned_byte()?;
                if count == 0 {
                    let mut vec = vec!(0u8; 768);
                    reader.read_exact(&mut vec[..])?;
                    vec

                } else {
                    let bytes = reader.read_bytes_available(count as usize)?;
                    let mut read = &mut &bytes[..] as &mut Read;
                    let mut decoder = GzDecoder::new(read)?;
                    let mut vec = Vec::new();
                    decoder.read_to_end(&mut vec)?;
                    vec
                }
            }
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for PixelClusterData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for PixelClusterData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<PixelClusterData> + BorrowMut<PixelClusterData> + Unit> PixelCluster for  T {
    fn data(&self) -> &Vec<u8> {
        &self.borrow().data
    }
}