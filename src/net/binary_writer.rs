
use std::io::Result;
use std::io::Write;

use std::str;

use byteorder::LittleEndian;
use byteorder::ByteOrder;

pub trait BinaryWriter: Write {

    /// C# BinaryWriter#Write7BitEncodedInt
    fn write_7bit_encoded_int(&mut self, int: u32) -> Result<()>;

    /// C# BinaryReader#ReadString
    fn write_string(&mut self, str: &str) -> Result<()>;

    fn write_bool(&mut self, v: bool) -> Result<()>;

    fn write_i64(&mut self, v: i64) -> Result<()>;

    fn write_f32(&mut self, v: f32) -> Result<()>;

    fn write_f64(&mut self, v: f64) -> Result<()>;

    fn write_u32(&mut self, v: u32) -> Result<()>;

    fn write_u16(&mut self, v: u16) -> Result<()>;

    fn write_u8(&mut self, v: u8) -> Result<()>;

    fn write_i32(&mut self, v: i32) -> Result<()>;

    fn write_i16(&mut self, v: i16) -> Result<()>;

    fn write_byte(&mut self, v: u8) -> Result<()>;
}

impl<T: Write> BinaryWriter for T {

    fn write_7bit_encoded_int(&mut self, mut value: u32) -> Result<()> {
        loop {
            // once again
            // salting with some magic numbers
            let num   : u32     = (value >> 7) & 0x01FFFFFF;
            let mut b : [u8; 1] = [value as u8 & 0x7F];

            if num != 0 {
                b[0] |= 0x80;
            }

            self.write_all(&b)?;
            value = num;

            if value == 0 {
                return Ok(())
            }
        }
    }

    fn write_string(&mut self, str: &str) -> Result<()> {
        let bytes = str.as_bytes();
        self.write_7bit_encoded_int(bytes.len() as u32)?;
        self.write(&bytes)?;
        Ok(())
    }

    fn write_bool(&mut self, v: bool) -> Result<()> {
        let mut b = [0u8];
        if v {
            b[0] = 0x01;
        }
        self.write_all(&b)
    }

    fn write_i64(&mut self, v: i64) -> Result<()> {
        let mut b = [0u8; 8];
        LittleEndian::write_i64(&mut b, v);
        self.write_all(&b)
    }

    fn write_f32(&mut self, v: f32) -> Result<()> {
        let mut b = [0u8; 4];
        LittleEndian::write_f32(&mut b, v);
        self.write_all(&b)
    }

    fn write_f64(&mut self, v: f64) -> Result<()> {
        let mut b = [0u8; 8];
        LittleEndian::write_f64(&mut b, v);
        self.write_all(&b)
    }

    fn write_u32(&mut self, v: u32) -> Result<()> {
        let mut b = [0u8; 4];
        LittleEndian::write_u32(&mut b, v);
        self.write_all(&b)
    }

    fn write_u16(&mut self, v: u16) -> Result<()> {
        let mut b = [0u8; 2];
        LittleEndian::write_u16(&mut b, v);
        self.write_all(&b)
    }

    fn write_u8(&mut self, v: u8) -> Result<()> {
        let b = [v];
        self.write_all(&b)
    }

    fn write_i32(&mut self, v: i32) -> Result<()> {
        let mut b = [0u8; 4];
        LittleEndian::write_i32(&mut b, v);
        self.write_all(&b)
    }

    fn write_i16(&mut self, v: i16) -> Result<()> {
        let mut b = [0u8; 2];
        LittleEndian::write_i16(&mut b, v);
        self.write_all(&b)
    }

    fn write_byte(&mut self, v: u8) -> Result<()> {
        self.write_u8(v)
    }
}
