
use std::sync::Weak;

use Error;
use Connector;
use item::CargoItem;
use item::CargoItemData;
use item::CargoItemKind;
use net::BinaryReader;

downcast!(MissionTargetCargoItem);
pub trait MissionTargetCargoItem : CargoItem {

}

pub(crate) struct MissionTargetCargoItemData {
    pub(crate) cargo_item_data: CargoItemData,
}

impl MissionTargetCargoItemData {
    pub(crate) fn new(connector: Weak<Connector>, master: bool, reader: &mut BinaryReader) -> Result<MissionTargetCargoItemData, Error> {
        Ok(MissionTargetCargoItemData {
            cargo_item_data: CargoItemData::new(connector, master, reader)?,
        })
    }
}

impl CargoItem for MissionTargetCargoItemData {
    fn weight(&self) -> f32 {
        self.cargo_item_data.weight
    }

    fn kind(&self) -> CargoItemKind {
        CargoItemKind::MissionTarget
    }
}

impl MissionTargetCargoItem for MissionTargetCargoItemData {

}
