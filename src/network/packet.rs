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

    pub fn update_header(&mut self, f: impl FnOnce(&mut PacketHeader)) {
        if let Some(mut header) = self.next_header_without_advancing() {
            f(&mut header);
            self.write_header_without_advancing(header);
        }
    }

    pub fn next_header(&mut self) -> Option<PacketHeader> {
        self.next_header_without_advancing().map(|header| {
            self.offest += header.0.len();
            header
        })
    }

    fn next_header_without_advancing(&mut self) -> Option<PacketHeader> {
        let mut packet_header = [0u8; 8];
        if self.offest + packet_header.len() < self.payload.len() {
            packet_header
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = self.payload[self.offest + i]);
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

    pub fn write_header(&mut self, header: PacketHeader) {
        let header_len = header.0.len();
        self.write_header_without_advancing(header);
        self.offest += header_len;
    }

    pub fn write_header_without_advancing(&mut self, header: PacketHeader) {
        while self.offest + header.0.len() > self.payload.len() {
            self.payload.push(0x00);
        }
        self.payload[self.offest..][..header.0.len()].copy_from_slice(&header.0[..]);
    }
}
