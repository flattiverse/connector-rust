
use std::sync::Arc;
use std::sync::Weak;

use crate::Error;
use crate::Connector;
use crate::net::BinaryReader;

use crate::item::CargoItemKind;


pub trait CargoItem : Sync + Send {

    fn weight(&self) -> f32;

    fn kind(&self) -> CargoItemKind;
}


pub(crate) struct CargoItemData {
    pub(crate) connector: Weak<Connector>,
    pub(crate) weight: f32,
    pub(crate) master: bool
}

impl CargoItemData {
    pub(crate) fn new(connector: &Arc<Connector>, reader: &mut BinaryReader, master: bool) -> Result<CargoItemData, Error> {
        Ok(CargoItemData {
            master,
            connector: Arc::downgrade(connector),
            weight:    reader.read_single()?,
        })
    }
}

impl CargoItem for CargoItemData {
    fn weight(&self) -> f32 {
        self.weight
    }

    fn kind(&self) -> CargoItemKind {
        unimplemented!()
    }
}