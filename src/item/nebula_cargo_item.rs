
use std::sync::Weak;

use Color;
use Error;
use Connector;
use item::CargoItem;
use item::CargoItemData;
use item::CargoItemKind;
use net::BinaryReader;

downcast!(NebulaCargoItem);
pub trait NebulaCargoItem : CargoItem {
    fn color(&self) -> &Color;

    fn red(&self) -> f32 {
        self.color().red()
    }

    fn green(&self) -> f32 {
        self.color().green()
    }

    fn blue(&self) -> f32 {
        self.color().blue()
    }

    fn alpha(&self) -> f32 {
        self.color().alpha()
    }
}



pub(crate) struct NebulaCargoItemData {
    pub(crate) cargo_item_data: CargoItemData,
    pub(crate) color:           Color
}

impl NebulaCargoItemData {
    pub(crate) fn new(connector: Weak<Connector>, master: bool, reader: &mut BinaryReader) -> Result<NebulaCargoItemData, Error> {
        Ok(NebulaCargoItemData {
            cargo_item_data: CargoItemData::new(connector, master, reader)?,
            color:           Color::from_hue(reader.read_single()?)?
        })
    }
}

impl CargoItem for NebulaCargoItemData {
    fn weight(&self) -> f32 {
        self.cargo_item_data.weight
    }

    fn kind(&self) -> CargoItemKind {
        CargoItemKind::Nebula
    }
}