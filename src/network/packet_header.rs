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
            .field("id", &self.id())
            .field("id0", &self.id0())
            .field("id1", &self.id1())
            .field("param", &self.param())
            .field("param0", &self.param0())
            .field("param1", &self.param1())
            .field("size", &self.size())
            .finish()
    }
}

impl PacketHeader {
    pub const SIZE: usize = 8;

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
    pub fn id(&self) -> u16 {
        u16::from_le_bytes([self.id0(), self.id1()])
    }

    #[inline]
    pub fn set_id(&mut self, id: u16) {
        let [id0, id1] = id.to_le_bytes();
        self.set_id0(id0);
        self.set_id1(id1);
    }

    #[inline]
    pub fn id0(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn set_id0(&mut self, id0: u8) {
        self.0[2] = id0;
    }

    #[inline]
    pub fn id1(&self) -> u8 {
        self.0[3]
    }

    #[inline]
    pub fn set_id1(&mut self, id1: u8) {
        self.0[3] = id1;
    }

    #[inline]
    pub fn param(&self) -> u16 {
        u16::from_le_bytes([self.0[4], self.0[5]])
    }

    #[inline]
    pub fn set_param(&mut self, param: u16) {
        let [p0, p1] = param.to_le_bytes();
        self.set_param0(p0);
        self.set_param1(p1);
    }

    #[inline]
    pub fn param0(&self) -> u8 {
        self.0[4]
    }

    #[inline]
    pub fn set_param0(&mut self, param0: u8) {
        self.0[4] = param0;
    }

    #[inline]
    pub fn param1(&self) -> u8 {
        self.0[5]
    }

    #[inline]
    pub fn set_param1(&mut self, param1: u8) {
        self.0[5] = param1;
    }

    #[inline]
    pub fn size(&self) -> u16 {
        u16::from_le_bytes([self.0[6], self.0[7]])
    }
}

impl From<PacketHeader> for BytesMut {
    #[inline]
    fn from(header: PacketHeader) -> Self {
        header.0
    }
}
