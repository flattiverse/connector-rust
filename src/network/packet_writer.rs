use bytes::{BufMut, BytesMut};

pub trait PacketWriter {
    fn write_sbyte(&mut self, number: i8);
    fn write_byte(&mut self, number: u8);
    fn write_int16(&mut self, number: i16);
    fn write_uint16(&mut self, number: u16);
    fn write_int32(&mut self, number: i32);
    fn write_uint32(&mut self, number: u32);
    fn write_int64(&mut self, number: i64);
    fn write_uint64(&mut self, number: u64);
    fn write_1s(&mut self, number: f64, shift: f64);
    fn write_1u(&mut self, number: f64, shift: f64);
    fn write_2s(&mut self, number: f64, shift: f64);
    fn write_2u(&mut self, number: f64, shift: f64);
    fn write_4s(&mut self, number: f64, shift: f64);
    fn write_4u(&mut self, number: f64, shift: f64);
    fn write_8s(&mut self, number: f64, shift: f64);
    fn write_8u(&mut self, number: f64, shift: f64);
    fn write_boolean(&mut self, value: bool);
    fn write_string(&mut self, text: &str);
    fn write_nullable_byte(&mut self, value: Option<u8>);
}

impl PacketWriter for BytesMut {
    #[inline]
    fn write_sbyte(&mut self, number: i8) {
        self.put_i8(number);
    }

    #[inline]
    fn write_byte(&mut self, number: u8) {
        self.put_u8(number);
    }

    #[inline]
    fn write_int16(&mut self, number: i16) {
        self.put_i16_le(number);
    }

    #[inline]
    fn write_uint16(&mut self, number: u16) {
        self.put_u16_le(number);
    }

    #[inline]
    fn write_int32(&mut self, number: i32) {
        self.put_i32_le(number);
    }

    #[inline]
    fn write_uint32(&mut self, number: u32) {
        self.put_u32_le(number);
    }

    #[inline]
    fn write_int64(&mut self, number: i64) {
        self.put_i64_le(number);
    }

    #[inline]
    fn write_uint64(&mut self, number: u64) {
        self.put_u64_le(number);
    }

    fn write_1s(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_i8(value as _);
    }

    fn write_1u(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_u8(value as _);
    }

    fn write_2s(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_i16_le(value as _);
    }

    fn write_2u(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_u16_le(value as _);
    }

    fn write_4s(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_i32_le(value as _);
    }

    fn write_4u(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_u32_le(value as _);
    }

    fn write_8s(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_i64_le(value as _);
    }

    fn write_8u(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        self.put_u64_le(value as _);
    }

    #[inline]
    fn write_boolean(&mut self, value: bool) {
        self.put_u8(if value { 1 } else { 0 })
    }

    fn write_string(&mut self, text: &str) {
        let bytes = text.as_bytes();
        self.write_byte(bytes.len() as _);
        self.put_slice(bytes);
    }

    fn write_nullable_byte(&mut self, value: Option<u8>) {
        match value {
            Some(value) => {
                self.write_boolean(true);
                self.write_byte(value);
            }
            None => self.write_boolean(false),
        }
    }
}
