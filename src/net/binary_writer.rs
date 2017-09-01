
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Write;

use std::cmp;
use std::str;

use byteorder::BigEndian;
use byteorder::ByteOrder;

pub trait BinaryWriter {

    /// C# BinaryWriter#Write7BitEncodedInt
    fn write_7bit_encoded_int(&mut self, int: u32) -> Result<()>;

    /// C# BinaryReader#ReadString
    fn write_string(&mut self, str: &str) -> Result<()>;
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
        unimplemented!()
    }
}
