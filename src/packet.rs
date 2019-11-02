use byteorder::ReadBytesExt;
use bytes::{BufMut, Bytes, BytesMut};

use crate::io::BinaryReader;

#[derive(Default, Debug)]
pub struct Packet {
    pub(crate) command: u8,
    pub(crate) session: u8,
    pub(crate) id: u32,
    pub(crate) helper: u8,
    pub(crate) base_address: u16,
    pub(crate) sub_address: u8,
    pub(crate) payload: Option<Bytes>,
    pub(crate) out_of_band: bool,
}

impl Packet {
    pub(crate) fn new_oob() -> Self {
        Packet {
            out_of_band: true,
            ..Default::default()
        }
    }

    pub fn payload(&self) -> &[u8] {
        self.payload.as_ref().map(|v| &v[..]).unwrap_or(&[][..])
    }

    pub fn try_parse(data: &mut BytesMut) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let slice = &mut &data[..]; // slice over available bytes
        let reader = slice as &mut dyn BinaryReader;

        let header = reader.read_byte().ok()?;
        let oob = (header & 0b0011_0000) == 0b0011_0000;

        let packet_len = if oob {
            usize::from(header & 0b0000_1111)
        } else {
            match (header & 0b0011_0000) >> 4 {
                0 => 0,
                1 => usize::from(reader.read_u8().ok()?) + 1,
                2 => usize::from(reader.read_uint16().ok()?) + 1,
                l => panic!("Unexpected header len information {}", l),
            }
        };

        if oob {
            Some(Packet {
                out_of_band: true,
                payload: Some({
                    let read = data.len() - slice.len();
                    if slice.len() < packet_len {
                        return None; // payload not yet in the input buffer
                    }
                    let _header = data.split_to(read); // discard the header bytes
                    data.split_to(packet_len).freeze()
                }),
                ..Default::default()
            })
        } else {
            Some(Packet {
                command: if header & 0b1000_0000 > 0 {
                    reader.read_u8().ok()?
                } else {
                    0
                },
                session: if header & 0b0100_0000 > 0 {
                    reader.read_u8().ok()?
                } else {
                    0
                },
                base_address: if header & 0b0000_1000 > 0 {
                    reader.read_uint16().ok()?
                } else {
                    0
                },
                sub_address: if header & 0b0000_0100 > 0 {
                    reader.read_u8().ok()?
                } else {
                    0
                },
                id: if header & 0b0000_0010 > 0 {
                    reader.read_u32().ok()?
                } else {
                    0
                },
                helper: if header & 0b0000_0001 > 0 {
                    reader.read_u8().ok()?
                } else {
                    0
                },
                payload: {
                    if slice.len() < packet_len {
                        return None; // payload not yet in the input buffer
                    }
                    let read = data.len() - slice.len();
                    let _header = data.split_to(read); // discard the header bytes
                    if packet_len == 0 {
                        None
                    } else {
                        Some(data.split_to(packet_len).freeze())
                    }
                },
                out_of_band: oob,
            })
        }
    }

    pub fn write(&self, data: &mut BytesMut) {
        let payload_len = self.payload.as_ref().map(Bytes::len).unwrap_or(0);
        data.reserve(payload_len + 10);
        let mut header = 0_u8;

        if !self.out_of_band {
            if self.command > 0 {
                header |= 0b1000_0000;
            }

            if self.session > 0 {
                header |= 0b0100_0000;
            }

            if self.base_address > 0 {
                header |= 0b0000_1000;
            }

            if self.sub_address > 0 {
                header |= 0b0000_0100;
            }

            if self.id > 0 {
                header |= 0b0000_0010;
            }

            if self.helper > 0 {
                header |= 0b0000_0001;
            }

            if payload_len > 65_536 {
                panic!(
                    "Payload is not allowed to exceed 65 KiB but is {}",
                    payload_len
                );
            } else if payload_len > 256 {
                header |= 0b0010_0000;
            } else if payload_len > 0 {
                header |= 0b0001_0000;
            }
        }

        if self.out_of_band {
            data.put_u8(0b0011_0000 | payload_len as u8);
            for _ in 0..payload_len {
                data.put_u8(0x55);
            }
        } else {
            data.put_u8(header);

            if payload_len > 256 {
                data.put_u32_le((payload_len - 1) as u32);
            } else if payload_len > 0 {
                data.put_u8((payload_len - 1) as u8);
            }

            if self.command > 0 {
                data.put_u8(self.command);
            }

            if self.session > 0 {
                data.put_u8(self.session);
            }

            if self.base_address > 0 {
                data.put_u16_le(self.base_address);
            }

            if self.sub_address > 0 {
                data.put_u8(self.sub_address);
            }

            if self.id > 0 {
                data.put_u32_le(self.id);
            }

            if self.helper > 0 {
                data.put_u8(self.helper);
            } else if let Some(payload) = &self.payload {
                data.put_slice(&payload[..]);
            }
        }
    }
}
