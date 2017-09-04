
use super::super::Error;
use super::BinaryReader;
use super::BinaryWriter;
use super::is_set_u8;

const HEADER_FLAG_SESSION           : u8 = 0x80;
const HEADER_FLAG_UNIVERSE_GROUP    : u8 = 0x40;
const HEADER_FLAG_PLAYER            : u8 = 0x20;
const HEADER_FLAG_UNIVERSE          : u8 = 0x10;
const HEADER_FLAG_SHIP              : u8 = 0x08;
const HEADER_FLAG_SUB               : u8 = 0x04;

#[derive(Debug)]
pub struct Packet {
    command: u8,
    session: u8,
    path_universe_group: u16,
    path_player: u16,
    path_universe: u8,
    path_ship: u8,
    path_sub: u8,
    data: Vec<u8>
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            command: 0,
            session: 0,
            path_universe_group: 0,
            path_universe: 0,
            path_player: 0,
            path_ship: 0,
            path_sub: 0,
            data: Vec::new()
        }
    }

    pub(crate) fn from_reader(max_packet_size: u32, reader: &mut BinaryReader) -> Result<Packet, Error> {
        let mut packet = Packet::new();
        let header = reader.read_byte()?;

        packet.command = reader.read_byte()?;

        if is_set_u8(header, HEADER_FLAG_SESSION) {
            packet.session = reader.read_byte()?;
        }

        if is_set_u8(header, HEADER_FLAG_UNIVERSE_GROUP) {
            packet.path_universe_group = reader.read_u16()?;
        }

        if is_set_u8(header, HEADER_FLAG_PLAYER) {
            packet.path_player = reader.read_u16()?;
        }

        if is_set_u8(header, HEADER_FLAG_UNIVERSE) {
            packet.path_universe = reader.read_byte()?;
        }

        if is_set_u8(header, HEADER_FLAG_SHIP) {
            packet.path_ship = reader.read_byte()?;
        }

        if is_set_u8(header, HEADER_FLAG_SUB) {
            packet.path_sub = reader.read_byte()?;
        }

        match header & 0x03 {
            0x00 => packet.data = Vec::with_capacity(0),
            0x01 => {
                let data_length = reader.read_byte()? as u32 + 1;
                if data_length > max_packet_size {
                    return Err(Error::RequestedPacketSizeIsInvalid {
                        max: max_packet_size,
                        was: data_length
                    });
                }
                packet.data = reader.read_bytes(data_length as usize)?;
            },
            0x02 => {
                let data_length = reader.read_u16()? as u32 + 257;
                if data_length > max_packet_size {
                    return Err(Error::RequestedPacketSizeIsInvalid {
                        max: max_packet_size,
                        was: data_length
                    });
                }
                packet.data = reader.read_bytes(data_length as usize)?;
            },
            0x03 => {
                let data_length = reader.read_int()? as u32 + 65793;
                if data_length > max_packet_size {
                    return Err(Error::RequestedPacketSizeIsInvalid {
                        max: max_packet_size,
                        was: data_length
                    });
                }
                packet.data = reader.read_bytes(data_length as usize)?;
            }
            _ => {}
        }

        Ok(packet)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn read(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn write(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn compile(&self) -> Result<Vec<u8>, Error> {
        let mut dest = Vec::new();
        self.write_to(&mut &mut dest)?;
        Ok(dest)
    }

    pub(crate) fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), Error> {
        let mut header = 0u8;

        if self.session != 0x00 {
            header |= HEADER_FLAG_SESSION;
        }

        if self.path_universe_group != 0 {
            header |= HEADER_FLAG_UNIVERSE_GROUP;
        }

        if self.path_player != 0 {
            header |= HEADER_FLAG_PLAYER;
        }

        if self.path_universe != 0 {
            header |= HEADER_FLAG_UNIVERSE;
        }

        if self.path_ship != 0 {
            header |= HEADER_FLAG_SHIP;
        }

        if self.path_sub != 0 {
            header |= HEADER_FLAG_SUB;
        }

        if self.data.len() > 65792 {
            header |= 0x03;

        } else if self.data.len() > 256 {
            header |= 0x02;

        } else if self.data.len() > 0 {
            header |= 0x01;
        }

        writer.write_byte(header)?;
        writer.write_u8(self.command)?;

        if self.session != 0 {
            writer.write_u8(self.session)?;
        }

        if self.path_universe_group != 0 {
            writer.write_u16(self.path_universe_group)?;
        }

        if self.path_player != 0 {
            writer.write_u16(self.path_player)?;
        }

        if self.path_universe != 0 {
            writer.write_u8(self.path_universe)?;
        }

        if self.path_ship != 0 {
            writer.write_u8(self.path_ship)?;
        }

        if self.path_sub != 0 {
            writer.write_u8(self.path_sub)?;
        }

        if self.data.len() > 65792 {
            writer.write_u32((self.data.len() - 65793) as u32)?;
            writer.write_all(&self.data)?;

        } else if self.data.len() > 256 {
            writer.write_u16((self.data.len() - 257) as u16)?;
            writer.write_all(&self.data)?;

        } else if self.data.len() > 0 {
            writer.write_u8((self.data.len() - 1) as u8)?;
            writer.write_all(&self.data)?;
        }

        Ok(())
    }

    pub fn command(&self) -> u8 {
        self.command
    }

    pub fn session(&self) -> u8 {
        self.session
    }

    pub fn set_session(&mut self, session: u8) {
        self.session = session;
    }

    pub fn path_universe_group(&self) -> u16 {
        self.path_universe_group
    }

    pub fn path_player(&self) -> u16 {
        self.path_player
    }

    pub fn path_universe(&self) -> u8 {
        self.path_universe
    }

    pub fn path_ship(&self) -> u8 {
        self.path_ship
    }

    pub fn path_sub(&self) -> u8 {
        self.path_sub
    }

    pub fn set_command(&mut self, command: u8) {
        self.command = command;
    }

    pub fn set_path_universe_group(&mut self, path_universe_group: u16) {
        self.path_universe_group = path_universe_group;
    }

    pub fn set_path_player(&mut self, path_player: u16) {
        self.path_player = path_player;
    }

    pub fn set_path_universe(&mut self, path_universe: u8) {
        self.path_universe = path_universe;
    }

    pub fn set_path_ship(&mut self, path_ship: u8) {
        self.path_ship = path_ship;
    }

    pub fn set_path_sub(&mut self, path_sub: u8) {
        self.path_sub = path_sub;
    }
}