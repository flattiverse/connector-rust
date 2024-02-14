#[derive(Debug, Copy, Clone)]
pub struct PacketHeader(pub(crate) [u8; 8]);

impl PacketHeader {
    #[inline]
    pub fn command(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn session(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn player(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn controllable(&self) -> u8 {
        self.0[3]
    }

    #[inline]
    pub fn param(&self) -> u16 {
        u16::from_be_bytes([self.0[4], self.0[5]])
    }

    #[inline]
    pub fn param0(&self) -> u8 {
        self.0[4]
    }

    #[inline]
    pub fn param1(&self) -> u8 {
        self.0[5]
    }

    #[inline]
    pub fn size(&self) -> u16 {
        u16::from_be_bytes([self.0[6], self.0[7]])
    }

    #[inline]
    pub fn direct_assign(&self) -> u64 {
        u64::from_be_bytes(self.0)
    }
}

impl From<[u8; 8]> for PacketHeader {
    #[inline]
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}
