use crate::network::PacketReader;

#[derive(Debug)]
pub struct Team {
    id: u8,
    name: String,
    red: u8,
    green: u8,
    blue: u8,
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

    #[inline]
    pub fn id(&self) -> u8 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn red(&self) -> u8 {
        self.red
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.green
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.blue
    }
}
