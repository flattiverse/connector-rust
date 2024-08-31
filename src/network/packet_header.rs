use bytes::BytesMut;
use std::fmt::{Debug, Formatter};

pub struct PacketHeader(BytesMut);

impl From<BytesMut> for PacketHeader {
    #[inline]
    fn from(value: BytesMut) -> Self {
        debug_assert_eq!(
            value.len(),
            Self::SIZE,
            "Unexpected size={}, expecting size of {}",
            value.len(),
            Self::SIZE
        );
        Self(value)
    }
}

impl Debug for PacketHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketHeader")
            .field("command", &self.command())
            .field("session", &self.session())
            .field("size", &self.size())
            .finish()
    }
}

impl PacketHeader {
    pub const SIZE: usize = 4;

    #[inline]
    pub fn command(&self) -> u8 {
        self.0[0]
    }
    #[inline]
    pub fn set_command(&mut self, command: u8) {
        self.0[0] = command;
    }

    #[inline]
    pub fn session(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn set_session(&mut self, session: u8) {
        self.0[1] = session;
    }

    #[inline]
    pub fn size(&self) -> u16 {
        u16::from_le_bytes([self.0[2], self.0[3]])
    }

    #[inline]
    pub fn set_size(&mut self, size: u16) {
        let bytes = size.to_le_bytes();
        self.0[2] = bytes[0];
        self.0[3] = bytes[1];
    }
}

impl From<PacketHeader> for BytesMut {
    #[inline]
    fn from(header: PacketHeader) -> Self {
        header.0
    }
}
