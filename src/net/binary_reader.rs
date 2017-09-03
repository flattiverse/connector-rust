
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

use std::cmp;
use std::str;

use byteorder::LittleEndian;
use byteorder::ByteOrder;

pub trait BinaryReader: Read {

    fn read_7bit_encoded_int(&mut self) -> Result<u32>;

    fn read_string(&mut self) -> Result<String>;

    fn read_bool(&mut self) -> Result<bool>;

    fn read_byte(&mut self) -> Result<u8>;

    fn read_unsigned_byte(&mut self) -> Result<u8>;

    fn read_u32(&mut self) -> Result<u32>;

    fn read_i64(&mut self) -> Result<i64>;

    fn read_short(&mut self) -> Result<i16>;

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>>;

    /// Reads available bytes but not more than specified
    fn read_bytes_available(&mut self, up_to_size: usize) -> Result<Vec<u8>>;

    fn read_u16(&mut self) -> Result<u16>;

    fn read_int(&mut self) -> Result<i32>;

    fn read_single(&mut self) -> Result<f32>;

    fn read_double(&mut self) -> Result<f64>;
}

impl<T: Read> BinaryReader for T {
    fn read_7bit_encoded_int(&mut self) -> Result<u32> {
        let mut num  : u32 = 0;
        let mut num2 : u32 = 0;
        let mut i    : u32 = 0;

        while i < 5 {
            let mut b : [u8; 1] = [0; 1];
            self.read_exact(&mut b)?;
            num  |= ((b[0] & 127) as u32) << num2;
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
                Err(e)  => return Err(Error::new(ErrorKind::InvalidData, e))
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
        let mut v = vec!(0u8; size);
        self.read_exact(&mut v)?;
        Ok(v)
    }

    /// Reads available bytes but not more than specified
    fn read_bytes_available(&mut self, up_to_size: usize) -> Result<Vec<u8>> {
        let mut v = Vec::with_capacity(up_to_size);
        self.read(&mut v)?;
        Ok(v)
    }

    fn read_u16(&mut self) -> Result<u16> {
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

