use byteorder::{ByteOrder, NetworkEndian};
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Default, Debug)]
pub struct Packet {
    command: u8,
    session: u8,
    id: u32,
    helper: u8,
    base_address: u16,
    sub_address: u8,
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

    pub fn try_parse(data: &mut BytesMut) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let header = data[0];
        let oob = (header & 0b0011_0000) == 0b0011_0000;

        let (packet_len, mut offset) = if oob {
            (usize::from(header & 0b0000_1111), 1)
        } else {
            match (header & 0b0011_0000) >> 4 {
                0 => {
                    // no payload
                    (0, 1_usize)
                }
                1 => {
                    // 1 further byte for length information
                    if data.len() <= 2 {
                        return None;
                    }

                    let len = usize::from(data[1]) + 1;

                    (len, 2_usize)
                }
                2 => {
                    // 2 further bytes for length information
                    if data.len() <= 3 {
                        return None;
                    }

                    let len = usize::from(NetworkEndian::read_u16(&data[1..3])) + 1;

                    (len, 3_usize)
                }
                l => panic!("Unexpected header len information {}", l),
            }
        };

        // packet not yet fully received
        if data.len() < offset + packet_len {
            return None;
        }

        Some(Packet {
            command: if header & 0b1000_0000 > 0 {
                let v = data[offset];
                offset += 1;
                v
            } else {
                0
            },
            session: if header & 0b0100_0000 > 0 {
                let v = data[offset];
                offset += 1;
                v
            } else {
                0
            },
            base_address: if header & 0b0000_1000 > 0 {
                let v = NetworkEndian::read_u16(&data[offset..offset + 2]);
                offset += 2;
                v
            } else {
                0
            },
            sub_address: if header & 0b0000_0100 > 0 {
                let v = data[offset];
                offset += 1;
                v
            } else {
                0
            },
            id: if header & 0b0000_0010 > 0 {
                let v = NetworkEndian::read_u32(&data[offset..offset + 4]);
                offset += 4;
                v
            } else {
                0
            },
            helper: if header & 0b0000_0001 > 0 {
                let v = data[offset];
                offset += 1;
                v
            } else {
                0
            },
            payload: if packet_len == 0 {
                let _header = data.split_to(offset);
                None
            } else {
                let _header = data.split_to(offset);
                Some(data.split_to(packet_len).freeze())
            },
            out_of_band: oob,
        })
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
                data.put_u32_be((payload_len - 1) as u32);
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
                data.put_u16_be(self.base_address);
            }

            if self.id > 0 {
                data.put_u32_be(self.id);
            }

            if self.helper > 0 {
                data.put_u8(self.helper);
            } else if let Some(payload) = &self.payload {
                data.put_slice(&payload[..]);
            }
        }
    }
}
