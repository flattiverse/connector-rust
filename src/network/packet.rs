use crate::network::{PacketHeader, PacketReader, PacketWriter};

pub const SERVER_DEFAULT_PACKET_SIZE: usize = 1048; // yes 10_48_

pub struct MultiPacketBuffer {
    pub payload: Vec<u8>,
    pub offest: usize,
}

impl MultiPacketBuffer {
    #[inline]
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload, offest: 0 }
    }

    pub fn next_packet(&mut self) -> Option<Packet> {
        let header = self.next_header()?;
        let size = usize::from(header.size());
        if self.offest + size < self.payload.len() {
            self.offest += size;
            Some(Packet::new(
                header,
                (&self.payload[self.offest..][..size]).to_vec(),
            ))
        } else {
            Some(Packet::new(header, Vec::default()))
        }
    }

    #[inline]
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
}

#[derive(Debug)]
pub struct Packet {
    header: PacketHeader,
    payload: Vec<u8>,
}

impl From<PacketHeader> for Packet {
    fn from(header: PacketHeader) -> Self {
        Self {
            payload: Vec::with_capacity(SERVER_DEFAULT_PACKET_SIZE - 8),
            header,
        }
    }
}

impl Default for Packet {
    #[inline]
    fn default() -> Self {
        Self::from(PacketHeader([0u8; 8]))
    }
}

impl Packet {
    #[inline]
    pub fn new(header: PacketHeader, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }

    pub fn header(&self) -> &PacketHeader {
        &self.header
    }

    #[inline]
    pub fn header_mut(&mut self) -> &mut PacketHeader {
        &mut self.header
    }

    #[inline]
    pub fn read<'a, T>(&'a self, f: impl FnOnce(PacketReader<'a>) -> T) -> T {
        f(PacketReader::new(self.header, &self.payload[..]))
    }

    #[inline]
    pub fn write(&mut self, f: impl FnOnce(&mut dyn PacketWriter)) {
        self.payload.clear();
        f(&mut self.payload);
    }

    pub fn into_vec(self) -> Vec<u8> {
        // TOOD performance?
        let mut vec = Vec::with_capacity(self.header.0.len() + self.payload.len());
        vec.extend(self.header.0);
        vec.extend(self.payload);
        vec
    }
}
