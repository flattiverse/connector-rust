use crate::network::{PacketHeader, PacketReader, PacketWriter};
use bytes::BytesMut;

pub const SERVER_DEFAULT_PACKET_SIZE: usize = 1048; // yes 10_48_

pub struct MultiPacketBuffer(BytesMut);

impl From<BytesMut> for MultiPacketBuffer {
    #[inline]
    fn from(value: BytesMut) -> Self {
        Self(value)
    }
}

impl MultiPacketBuffer {
    pub fn next_packet(&mut self) -> Option<Packet> {
        let header = self.next_header()?;
        let size = usize::from(header.size());
        Some(Packet::new(header, self.0.split_to(size)))
    }

    #[inline]
    pub fn next_header(&mut self) -> Option<PacketHeader> {
        if self.0.len() >= PacketHeader::SIZE {
            Some(PacketHeader::from(self.0.split_to(PacketHeader::SIZE)))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    header: PacketHeader,
    payload: BytesMut,
}

impl Default for Packet {
    #[inline]
    fn default() -> Self {
        let mut bytes = BytesMut::with_capacity(SERVER_DEFAULT_PACKET_SIZE);
        let header_btytes = bytes.split_to(PacketHeader::SIZE);
        Self::new(PacketHeader::from(header_btytes), bytes)
    }
}

impl Packet {
    /// For performance reason, the [`ByteMut`] of the [`PacketHeader`] and the [`BytesMut`] of the
    /// `payload` should originate from an contiguous [`ByesMut`]. See [`BytesMut::unsplit`].
    #[inline]
    pub fn new(header: PacketHeader, payload: BytesMut) -> Self {
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
    pub fn read<'a, T>(&'a mut self, f: impl FnOnce(&'a mut dyn PacketReader) -> T) -> T {
        f(&mut self.payload)
    }

    #[inline]
    pub fn write(&mut self, f: impl FnOnce(&mut dyn PacketWriter)) {
        f(&mut self.payload);
    }

    #[inline]
    pub fn into_buf(mut self) -> BytesMut {
        let mut buf = BytesMut::from(self.header);
        buf.unsplit(self.payload.split());
        buf
    }
}