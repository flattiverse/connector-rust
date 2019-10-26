use crate::io::BinaryReader;
use crate::packet::Packet;
use std::convert::TryFrom;
use std::io::Error as IoError;

#[derive(Debug, Default)]
pub struct Team {
    id: u8,
    name: String,
    color_r: f32,
    color_g: f32,
    color_b: f32,
}

impl Team {
    pub(crate) fn update(&mut self, reader: &mut dyn BinaryReader) -> Result<(), IoError> {
        self.name = reader.read_string()?;
        self.color_r = f32::from(reader.read_byte()?) / 255_f32;
        self.color_g = f32::from(reader.read_byte()?) / 255_f32;
        self.color_b = f32::from(reader.read_byte()?) / 255_f32;
        Ok(())
    }
}

impl TryFrom<&Packet> for Team {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let mut team = Team {
            id: packet.sub_address,
            ..Default::default()
        };
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        team.update(reader)?;
        Ok(team)
    }
}
