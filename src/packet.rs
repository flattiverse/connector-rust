use byteorder::{ByteOrder, NetworkEndian};
use bytes::{Bytes, BytesMut};

#[derive(Debug)]
pub struct Packet {
    command: u8,
    session: u8,
    id: u32,
    helper: u8,
    base_address: u16,
    sub_address: u8,
    payload: Option<Bytes>,
    out_of_band: bool,
}

impl Packet {
    pub fn try_parse(data: &mut BytesMut) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let header = data[0];
        let oob = (header & 0b0011_0000) != 0;

        let (packet_len, mut offset) = if oob {
            (usize::from(header & 0b0000_1111), 1)
        } else {
            let packet_len = 1
                + (header & 0b1000_0000).min(1)
                + (header & 0b0100_0000).min(1)
                + (header & 0b0000_1000).min(2)
                + (header & 0b0000_0100).min(1)
                + ((header & 0b0000_0010) * 2)
                + (header & 0b0000_0001).min(1);
            let packet_len = usize::from(packet_len);

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
                None
            } else {
                let _header = data.split_to(offset);
                Some(data.split_to(packet_len).freeze())
            },
            out_of_band: oob,
        })
    }
}
