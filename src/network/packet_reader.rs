use crate::network::PacketHeader;
use bytes::{Buf, BufMut, BytesMut};

pub trait PacketReader {
    fn read_int32(&mut self) -> i32;
    fn read_string(&mut self, length: usize) -> String;
}

impl PacketReader for BytesMut {
    #[inline]
    fn read_int32(&mut self) -> i32 {
        self.get_i32_le()
    }

    fn read_string(&mut self, length: usize) -> String {
        let value = String::from_utf8(self[..length].to_vec());
        self.advance(length);
        value.expect("Invalid UTF-8 Characters received")
    }
}
