use crate::network::PacketReader;

#[derive(Debug)]
pub struct Cluster {
    id: u8,
    galaxy: i32,
    name: String,
}

impl Cluster {
    pub fn new(id: u8, galaxy: i32, reader: &mut dyn PacketReader) {
        Self {
            id,
            galaxy,
            name: reader.read_string(),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}
