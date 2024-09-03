use bytes::{Buf, BytesMut};

pub trait PacketReader {
    fn read_sbyte(&mut self) -> i8;
    fn read_byte(&mut self) -> u8;
    fn read_int16(&mut self) -> i16;
    fn read_uint16(&mut self) -> u16;
    fn read_int32(&mut self) -> i32;
    fn read_uint32(&mut self) -> u32;
    fn read_int64(&mut self) -> i64;
    fn read_uint64(&mut self) -> u64;
    fn read_f32(&mut self) -> f32;
    fn read_boolean(&mut self) -> bool;

    fn read_bytes(&mut self, amount: usize) -> Vec<u8>;
    fn read_string(&mut self) -> String;
    fn read_nullable_byte(&mut self) -> Option<u8>;
    fn read_remaining_as_string(&mut self) -> String;

    fn peek_string(&self) -> String;
    fn jump_over_string(&mut self);

    fn opt_read_sbyte(&mut self) -> Option<i8>;
    fn opt_read_byte(&mut self) -> Option<u8>;
    fn opt_read_int16(&mut self) -> Option<i16>;
    fn opt_read_uint16(&mut self) -> Option<u16>;
    fn opt_read_int32(&mut self) -> Option<i32>;
    fn opt_read_uint32(&mut self) -> Option<u32>;
    fn opt_read_int64(&mut self) -> Option<i64>;
    fn opt_read_uint64(&mut self) -> Option<u64>;
    fn opt_read_f32(&mut self) -> Option<f32>;
    fn opt_read_boolean(&mut self) -> Option<bool>;

    fn maybe_read_sbyte(&mut self, value: &mut i8) -> bool;
    fn maybe_read_byte(&mut self, value: &mut u8) -> bool;
    fn maybe_read_int16(&mut self, value: &mut i16) -> bool;
    fn maybe_read_uint16(&mut self, value: &mut u16) -> bool;
    fn maybe_read_int32(&mut self, value: &mut i32) -> bool;
    fn maybe_read_uint32(&mut self, value: &mut u32) -> bool;
    fn maybe_read_int64(&mut self, value: &mut i64) -> bool;
    fn maybe_read_uint64(&mut self, value: &mut u64) -> bool;
    fn maybe_read_f32(&mut self, value: &mut f32) -> bool;
    fn maybe_read_boolean(&mut self, value: &mut bool) -> bool;
}

impl PacketReader for BytesMut {
    #[inline]
    fn read_sbyte(&mut self) -> i8 {
        self.get_i8()
    }

    #[inline]
    fn read_byte(&mut self) -> u8 {
        self.get_u8()
    }

    #[inline]
    fn read_int16(&mut self) -> i16 {
        self.get_i16_le()
    }

    #[inline]
    fn read_uint16(&mut self) -> u16 {
        self.get_u16_le()
    }

    #[inline]
    fn read_int32(&mut self) -> i32 {
        self.get_i32_le()
    }

    #[inline]
    fn read_uint32(&mut self) -> u32 {
        self.get_u32_le()
    }

    #[inline]
    fn read_int64(&mut self) -> i64 {
        self.get_i64_le()
    }

    #[inline]
    fn read_uint64(&mut self) -> u64 {
        self.get_u64_le()
    }

    #[inline]
    fn read_f32(&mut self) -> f32 {
        self.get_f32_le()
    }

    #[inline]
    fn read_boolean(&mut self) -> bool {
        self.read_sbyte() == 1
    }

    fn read_bytes(&mut self, amount: usize) -> Vec<u8> {
        let bytes = self[..amount].to_vec();
        self.advance(amount);
        bytes
    }

    fn read_string(&mut self) -> String {
        let length = self.read_byte();
        let length = if length == 0xFF {
            self.read_uint16() as usize
        } else {
            length as usize
        };

        let string = String::from_utf8(self[..length].to_vec());
        self.advance(length);
        string.expect("Invalid UTF-8 received")
    }

    fn read_nullable_byte(&mut self) -> Option<u8> {
        if self.read_boolean() {
            Some(self.read_byte())
        } else {
            None
        }
    }

    fn read_remaining_as_string(&mut self) -> String {
        let slice = &self[..];
        let length = slice.len();
        let string = String::from_utf8(slice.to_vec()).unwrap();
        self.advance(length);
        string
    }

    fn peek_string(&self) -> String {
        let length = self[0];
        let length = usize::from(length);
        String::from_utf8(self[1..][..length].to_vec()).expect("Invalid UTF-8 received")
    }

    fn jump_over_string(&mut self) {
        let length = self.read_byte();
        let length = usize::from(length);
        self.advance(length);
    }

    fn opt_read_sbyte(&mut self) -> Option<i8> {
        let mut value = Default::default();
        if self.maybe_read_sbyte(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn opt_read_byte(&mut self) -> Option<u8> {
        let mut value = Default::default();
        if self.maybe_read_byte(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_int16(&mut self) -> Option<i16> {
        let mut value = Default::default();
        if self.maybe_read_int16(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_uint16(&mut self) -> Option<u16> {
        let mut value = Default::default();
        if self.maybe_read_uint16(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_int32(&mut self) -> Option<i32> {
        let mut value = Default::default();
        if self.maybe_read_int32(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_uint32(&mut self) -> Option<u32> {
        let mut value = Default::default();
        if self.maybe_read_uint32(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_int64(&mut self) -> Option<i64> {
        let mut value = Default::default();
        if self.maybe_read_int64(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_uint64(&mut self) -> Option<u64> {
        let mut value = Default::default();
        if self.maybe_read_uint64(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_f32(&mut self) -> Option<f32> {
        let mut value = Default::default();
        if self.maybe_read_f32(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn opt_read_boolean(&mut self) -> Option<bool> {
        let mut value = Default::default();
        if self.maybe_read_boolean(&mut value) {
            Some(value)
        } else {
            None
        }
    }

    fn maybe_read_sbyte(&mut self, value: &mut i8) -> bool {
        if self.remaining() > 0 {
            *value = self.read_sbyte();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_byte(&mut self, value: &mut u8) -> bool {
        if self.remaining() > 0 {
            *value = self.read_byte();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_int16(&mut self, value: &mut i16) -> bool {
        if self.remaining() > 1 {
            *value = self.read_int16();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_uint16(&mut self, value: &mut u16) -> bool {
        if self.remaining() > 1 {
            *value = self.read_uint16();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_int32(&mut self, value: &mut i32) -> bool {
        if self.remaining() > 3 {
            *value = self.read_int32();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_uint32(&mut self, value: &mut u32) -> bool {
        if self.remaining() > 3 {
            *value = self.read_uint32();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_int64(&mut self, value: &mut i64) -> bool {
        if self.remaining() > 7 {
            *value = self.read_int64();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_uint64(&mut self, value: &mut u64) -> bool {
        if self.remaining() > 7 {
            *value = self.read_uint64();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_f32(&mut self, value: &mut f32) -> bool {
        if self.remaining() > 3 {
            *value = self.read_f32();
            true
        } else {
            *value = Default::default();
            false
        }
    }

    fn maybe_read_boolean(&mut self, value: &mut bool) -> bool {
        if self.remaining() > 0 {
            *value = self.read_boolean();
            true
        } else {
            *value = Default::default();
            false
        }
    }
}
