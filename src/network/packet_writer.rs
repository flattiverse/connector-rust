use bytes::{BufMut, BytesMut};
use std::io::Write;

pub trait PacketWriter {
    fn write_4s(&mut self, number: f64, shift: f64);
}

impl PacketWriter for BytesMut {
    fn write_4s(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        let value = value as i32;
        self.put_i32_le(value);
    }
}
