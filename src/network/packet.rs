use crate::network::{PacketHeader, PacketReader, PacketWriter};
use bytes::{BufMut, BytesMut};

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
        bytes.put_bytes(0, PacketHeader::SIZE);
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
    pub fn read<T>(&mut self, f: impl FnOnce(&mut dyn PacketReader) -> T) -> T {
        let response = f(&mut self.payload);
        if !self.payload.is_empty() {
            warn!(
                "[0x{:02x}] There are still {} bytes remaining: {:?}",
                self.header.command(),
                self.payload.len(),
                self.header
            );
        }
        response
    }

    #[inline]
    pub fn with_write<T>(mut self, f: impl FnOnce(&mut dyn PacketWriter) -> T) -> Self {
        let _ = f(&mut self.payload);
        self
    }

    #[inline]
    pub fn write<T>(&mut self, f: impl FnOnce(&mut dyn PacketWriter) -> T) -> T {
        let result = f(&mut self.payload);
        let len = self.payload.len() as u16;
        self.header_mut().set_size(len);
        result
    }

    #[inline]
    pub fn into_buf(mut self) -> BytesMut {
        let mut buf = BytesMut::from(self.header);
        buf.unsplit(self.payload.split());
        buf
    }
}
