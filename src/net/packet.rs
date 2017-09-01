
use std::io::Read;
use std::convert;

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

pub struct Packet {
    command: u8,
    session: u8,
    path_universe_group: u16,
    path_player: u16,
    path_universe: u8,
    path_ship: u8,
    path_sub: u8,
    data: Vec<u8>,
    constant_read: Option<Vec<u8>>,
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
            data: Vec::new(),
            constant_read: None
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
                let data_length = reader.read_u32()? + 65793;
                if data_length > max_packet_size {
                    return Err(Error::RequestedPacketSizeIsInvalid {
                        max: max_packet_size,
                        was: data_length
                    });
                }
                packet.data = reader.read_bytes(data_length as usize)?;
            }
        }

        Ok(packet)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn read(&self) -> &BinaryReader {
        &&self.data[..]
    }

    pub fn activate_constant_read(&mut self) {
        self.constant_read = Some(self.data.clone())
    }

    pub fn deactivate_constant_read(&mut self) {
        self.constant_read = None
    }

    pub fn write(&mut self) -> &mut BinaryWriter {
        &mut &mut self.data
    }

    pub fn compile(&self) -> Vec<u8> {
        let mut dest = Vec::new();
        self.write_to(&mut &mut dest);
        dest
    }

    pub(crate) fn write_to(&self, writer: &mut BinaryWriter) -> Resul<(), Error> {

    }
}