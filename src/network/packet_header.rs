#[derive(Debug, Copy, Clone)]
pub struct PacketHeader(pub(crate) [u8; 8]);

impl PacketHeader {
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
    pub fn player(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn set_player(&mut self, player: u8) {
        self.0[2] = player;
    }

    #[inline]
    pub fn controllable(&self) -> u8 {
        self.0[3]
    }

    #[inline]
    pub fn seet_controllable(&mut self, controllable: u8) {
        self.0[3] = controllable;
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

impl From<[u8; 8]> for PacketHeader {
    #[inline]
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}
