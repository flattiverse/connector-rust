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
    fn read_1s(&mut self, shift: f64) -> f64;
    fn read_1u(&mut self, shift: f64) -> f64;
    fn read_2s(&mut self, shift: f64) -> f64;
    fn read_2u(&mut self, shift: f64) -> f64;
    fn read_3u(&mut self, shift: f64) -> f64;
    fn read_4s(&mut self, shift: f64) -> f64;
    fn read_4u(&mut self, shift: f64) -> f64;
    fn read_8s(&mut self, shift: f64) -> f64;
    fn read_8u(&mut self, shift: f64) -> f64;
    fn read_boolean(&mut self) -> bool;
    fn read_string(&mut self) -> String;
    fn read_nullable_byte(&mut self) -> Option<u8>;

    fn peek_string(&self) -> String;
    fn jump_over_string(&mut self);
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
    fn read_1s(&mut self, shift: f64) -> f64 {
        f64::from(self.read_sbyte()) / shift
    }

    #[inline]
    fn read_1u(&mut self, shift: f64) -> f64 {
        f64::from(self.read_byte()) / shift
    }

    #[inline]
    fn read_2s(&mut self, shift: f64) -> f64 {
        f64::from(self.read_int16()) / shift
    }

    #[inline]
    fn read_2u(&mut self, shift: f64) -> f64 {
        f64::from(self.read_uint16()) / shift
    }

    fn read_3u(&mut self, shift: f64) -> f64 {
        let mut data = [0u8; 3];
        (&self[..3]).copy_to_slice(&mut data);
        self.advance(3);
        let first_byte = u32::from(data[0]);
        let other_bytes = u32::from(u16::from_le_bytes([data[1], data[2]]));
        let value = (first_byte * u32::from(u16::MAX)) + other_bytes;
        value as f64 / shift
    }

    #[inline]
    fn read_4s(&mut self, shift: f64) -> f64 {
        f64::from(self.read_int32()) / shift
    }

    #[inline]
    fn read_4u(&mut self, shift: f64) -> f64 {
        f64::from(self.read_uint32()) / shift
    }

    #[inline]
    fn read_8s(&mut self, shift: f64) -> f64 {
        self.read_int64() as f64 / shift
    }

    #[inline]
    fn read_8u(&mut self, shift: f64) -> f64 {
        self.read_uint64() as f64 / shift
    }

    #[inline]
    fn read_boolean(&mut self) -> bool {
        self.read_sbyte() == 1
    }

    fn read_string(&mut self) -> String {
        let length = self.read_byte();
        let length = length as usize;
        let string = String::from_utf8((&self[..length]).to_vec());
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

    fn peek_string(&self) -> String {
        let length = self[0];
        let length = usize::from(length);
        String::from_utf8((&self[..length]).to_vec()).expect("Invalid UTF-8 received")
    }

    fn jump_over_string(&mut self) {
        let length = self.read_byte();
        let length = usize::from(length);
        self.advance(length);
    }
}
