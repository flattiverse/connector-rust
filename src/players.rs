use std::convert::TryFrom;
use std::io::Error as IoError;

use crate::io::BinaryReader;
use crate::packet::Packet;

#[derive(Debug, Clone)]
pub struct Player {
    id: i32,
    name: String,
    online: bool,
    ping: f32,
    account: u32,
}

impl Player {
    pub(crate) fn update_ping(&mut self, packet: &Packet) -> Result<(), IoError> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        self.ping = reader.read_single()?;
        Ok(())
    }
}

impl TryFrom<&Packet> for Player {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Player {
            id: i32::from(packet.base_address),
            account: packet.id,
            name: reader.read_string()?,
            online: reader.read_bool()?,
            ping: reader.read_single()?,
        })
    }
}

#[derive(Debug, Default)]
pub struct Team {
    id: u8,
    name: String,
    color_r: f32,
    color_g: f32,
    color_b: f32,
}

impl Team {
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color_rgb(&self) -> (f32, f32, f32) {
        (self.color_r, self.color_g, self.color_b)
    }

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
