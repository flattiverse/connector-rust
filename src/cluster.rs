use crate::network::PacketReader;

#[derive(Debug)]
pub struct Cluster {
    pub id: u8,
    pub galaxy: i32,
    pub name: String,
}

impl Cluster {
    pub fn new(id: u8, galaxy: i32, reader: &mut dyn PacketReader) -> Self {
        Self {
            id,
            galaxy,
            name: reader.read_string(),
        }
    }
}
