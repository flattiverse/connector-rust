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
    fn write_f32(&mut self, number: f32);
    fn write_boolean(&mut self, value: bool);
    fn write_string_with_len_prefix(&mut self, text: &str);
    fn write_string_without_len(&mut self, text: &str);
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

    #[inline]
    fn write_f32(&mut self, number: f32) {
        self.put_f32_le(number);
    }

    #[inline]
    fn write_boolean(&mut self, value: bool) {
        self.put_u8(if value { 1 } else { 0 })
    }

    fn write_string_with_len_prefix(&mut self, text: &str) {
        let bytes = text.as_bytes();
        if bytes.len() < 255 {
            debug_assert!(bytes.len() + 1 <= self.capacity(), "Packet too long.");
            self.write_byte(bytes.len() as _);
            self.put_slice(bytes);
        } else {
            debug_assert!(bytes.len() + 3 <= self.capacity(), "Packet too long.");
            self.write_byte(0xFF);
            self.write_uint16(bytes.len() as _);
            self.put_slice(bytes);
        }
    }

    #[inline]
    fn write_string_without_len(&mut self, text: &str) {
        self.put_slice(text.as_bytes())
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
