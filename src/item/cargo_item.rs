
use std::sync::Weak;

use Error;
use Connector;
use downcast::Any;
use net::BinaryReader;

use item::CargoItemKind;
use item::NebulaCargoItemData;
use item::CrystalCargoItemData;
use item::MissionTargetCargoItemData;


pub trait CargoItem : Any + Sync + Send {
    fn weight(&self) -> f32;

    fn kind(&self) -> CargoItemKind;
}
downcast!(CargoItem);


pub(crate) fn cargo_item_from_stream(connector: Weak<Connector>, master: bool, reader: &mut BinaryReader) -> Result<Box<CargoItem>, Error> {
    Ok(match reader.read_byte()? {
        0x00 => Box::new(NebulaCargoItemData        ::new(connector, master, reader)?),
        0x01 => Box::new(CrystalCargoItemData       ::new(connector, master, reader)?),
        0x02 => Box::new(MissionTargetCargoItemData ::new(connector, master, reader)?),
        id@_ => return Err(Error::InvalidCargoItem(id))
    })
}


pub(crate) struct CargoItemData {
    pub(crate) connector: Weak<Connector>,
    pub(crate) weight: f32,
    pub(crate) master: bool
}

impl CargoItemData {
    pub(crate) fn new(connector: Weak<Connector>, master: bool, reader: &mut BinaryReader) -> Result<CargoItemData, Error> {
        Ok(CargoItemData {
            connector:  connector,
            master:     master,
            weight:     reader.read_single()?,
        })
    }
}