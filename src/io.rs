// https://github.com/GhostTyper/BinaryMemoryReaderWriter

use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;

use std::cmp;
use std::str;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

pub trait BinaryReader: Read {
    fn read_7bit_encoded_int(&mut self) -> Result<u32>;

    fn read_string(&mut self) -> Result<String>;

    fn read_string_empty_is_none(&mut self) -> Result<Option<String>> {
        self.read_string().map(|string| {
            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        })
    }

    fn read_bool(&mut self) -> Result<bool>;

    fn read_byte(&mut self) -> Result<u8>;

    fn read_unsigned_byte(&mut self) -> Result<u8>;

    fn read_u32(&mut self) -> Result<u32>;

    fn read_i64(&mut self) -> Result<i64>;

    fn read_short(&mut self) -> Result<i16>;

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>>;

    /// Reads available bytes but not more than specified
    fn read_bytes_available(&mut self, up_to_size: usize) -> Result<Vec<u8>>;

    fn read_uint16(&mut self) -> Result<u16>;

    fn read_int(&mut self) -> Result<i32>;

    fn read_single(&mut self) -> Result<f32>;

    fn read_double(&mut self) -> Result<f64>;
}

impl<T: Read> BinaryReader for T {
    fn read_7bit_encoded_int(&mut self) -> Result<u32> {
        let mut num: u32 = 0;
        let mut num2: u32 = 0;
        let mut i: u32 = 0;

        while i < 5 {
            let mut b: [u8; 1] = [0; 1];
            self.read_exact(&mut b)?;
            num |= u32::from(b[0] & 127) << num2;
            num2 += 7;

            if b[0] & 128u8 == 0 {
                break;
            }

            i += 1;
        }

        if i < 5 {
            return Ok(num);
        }

        Err(Error::from(ErrorKind::InvalidData))
        // panic!("c#: 'Too many bytes in what should have been a 7 bit encoded Int32.'");
    }

    fn read_string(&mut self) -> Result<String> {
        let mut length = self.read_7bit_encoded_int()? as usize;
        let mut string = String::with_capacity(length as usize);
        let mut chunk = [0u8; 128];

        while length > 0 {
            let mut slice = &mut chunk[..cmp::min(128usize, length)];

            self.read_exact(&mut slice)?;
            match str::from_utf8(&slice) {
                Ok(str) => string.push_str(str),
                Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
            };

            length -= slice.len();
        }

        Ok(string)
    }

    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_byte()? != 0u8)
    }

    fn read_byte(&mut self) -> Result<u8> {
        let mut b = [0u8; 1];
        self.read_exact(&mut b)?;
        Ok(b[0])
    }

    fn read_unsigned_byte(&mut self) -> Result<u8> {
        self.read_byte()
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut b = [0u8; 4];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_u32(&b))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut b = [0u8; 8];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_i64(&b))
    }

    fn read_short(&mut self) -> Result<i16> {
        let mut b = [0u8; 2];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_i16(&b))
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut v = vec![0u8; size];
        self.read_exact(&mut v)?;
        Ok(v)
    }

    /// Reads available bytes but not more than specified
    fn read_bytes_available(&mut self, up_to_size: usize) -> Result<Vec<u8>> {
        let mut v = vec![0u8; up_to_size];
        let _ = self.read(&mut v[..])?;
        Ok(v)
    }

    fn read_uint16(&mut self) -> Result<u16> {
        let mut b = [0u8; 2];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_u16(&b))
    }

    fn read_int(&mut self) -> Result<i32> {
        let mut b = [0u8; 4];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_i32(&b))
    }

    fn read_single(&mut self) -> Result<f32> {
        let mut b = [0u8; 4];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_f32(&b))
    }

    fn read_double(&mut self) -> Result<f64> {
        let mut b = [0u8; 8];
        self.read_exact(&mut b)?;
        Ok(LittleEndian::read_f64(&b))
    }
}

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
            let num: u32 = (value >> 7) & 0x01_FF_FF_FF;
            let mut b: [u8; 1] = [value as u8 & 0x7F];

            if num != 0 {
                b[0] |= 0x80;
            }

            self.write_all(&b)?;
            value = num;

            if value == 0 {
                return Ok(());
            }
        }
    }

    fn write_string(&mut self, str: &str) -> Result<()> {
        let bytes = str.as_bytes();
        self.write_7bit_encoded_int(bytes.len() as u32)?;
        self.write_all(&bytes)?;
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
