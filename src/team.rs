use crate::network::PacketReader;

#[derive(Debug)]
pub struct Team {
    pub id: u8,
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Team {
    pub fn new(id: u8, reader: &mut dyn PacketReader) -> Self {
        Self {
            id,
            name: reader.read_string(),
            red: reader.read_byte(),
            green: reader.read_byte(),
            blue: reader.read_byte(),
        }
    }
}
