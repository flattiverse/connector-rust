use crate::network::PacketReader;
use crate::unit::sub_components::HarvestableSection;

#[derive(Debug)]
pub struct Harvestable {
    pub(crate) sections: Vec<HarvestableSection>,
}

impl Harvestable {
    pub(crate) fn new(reader: &mut dyn PacketReader) -> Self {
        Self {
            sections: (0..reader.read_byte())
                .map(|_| HarvestableSection::default().with_read(reader))
                .collect(),
        }
    }
}
