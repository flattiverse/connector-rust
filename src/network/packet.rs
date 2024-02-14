use crate::network::{PacketHeader, PacketReader};

pub struct Packet {
    pub payload: Vec<u8>,
    pub offest: usize,
}

impl Packet {
    #[inline]
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload, offest: 0 }
    }

    pub fn next_header(&mut self) -> Option<PacketHeader> {
        let mut packet_header = [0u8; 8];
        if self.offest + packet_header.len() < self.payload.len() {
            packet_header
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = self.payload[self.offest + i]);
            self.offest += packet_header.len();
            Some(PacketHeader(packet_header))
        } else {
            None
        }
    }

    #[inline]
    pub fn next_reader(&mut self) -> Option<PacketReader> {
        let header = self.next_header()?;
        self.next_reader_from(header)
    }

    pub fn next_reader_from(&self, header: PacketHeader) -> Option<PacketReader> {
        let size = usize::from(header.size());
        if self.offest + size < self.payload.len() {
            Some(PacketReader::new(
                header,
                &self.payload[self.offest..][..size],
            ))
        } else {
            None
        }
    }
}
