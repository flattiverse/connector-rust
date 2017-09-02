
use std::io::Read;
use std::io::Write;
use std::io::Result;

const DEFAULT_LFSR : u32   = 0x1337_1337;
const CACHE_SIZE   : usize = 1024 * 15; // 15 * 1 kb

pub struct CryptStream {
    lfsr: u32,
    last_lfsr: u8
}

impl CryptStream {
    pub fn new() -> CryptStream {
        CryptStream {
            lfsr: DEFAULT_LFSR-1,
            last_lfsr: 0
        }
    }

    pub fn with_lfsr(lfsr: u32) -> CryptStream {
        CryptStream {
            lfsr: if lfsr == 0 {0x1337_1338} else {lfsr},
            last_lfsr: 0
        }
    }

    pub fn crypt_in_place(&mut self, buffer: &mut [u8]) {
        for i in 0..buffer.len() {
            buffer[i] = self.crypt(buffer[i]);
        }
    }

    pub fn crypt_to(&mut self, buffer: &[u8], sink: &mut [u8]) {
        for i in 0..buffer.len() {
            sink[i] = self.crypt(buffer[i]);
        }
    }

    pub fn crypt(&mut self, value: u8) -> u8 {
        for _ in 0..9 {
            let left = self.lfsr / 2;
            let right = if self.lfsr & 0x01 == 0x01 {3489660929u32} else {0u32};

            self.lfsr = left ^ right;
        }

        self.last_lfsr = (self.lfsr % 256) as u8;
        value ^ self.last_lfsr
    }
}


pub struct CryptRead<T: Read> {
    source: T,
    crypt: CryptStream
}

impl<T: Read> CryptRead<T> {
    pub fn new(source: T) -> CryptRead<T> {
        CryptRead {
            source: source,
            crypt: CryptStream::new()
        }
    }

    pub fn with_lfsr(source: T, lfsr: u32) -> CryptRead<T> {
        CryptRead {
            source: source,
            crypt: CryptStream::with_lfsr(lfsr)
        }
    }
}

impl<T: Read> Read for CryptRead<T> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        let size = self.source.read(&mut buf)?;
        self.crypt.crypt_in_place(&mut buf[..size]);
        Ok(size)
    }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        self.source.read_exact(&mut buf)?;
        self.crypt.crypt_in_place(&mut buf);
        Ok(())
    }
}




pub struct CryptWrite<T: Write> {
    sink: T,
    crypt: CryptStream
}

impl<T: Write> CryptWrite<T> {
    pub fn new(sink: T) -> CryptWrite<T> {
        CryptWrite {
            sink: sink,
            crypt: CryptStream::new()
        }
    }

    pub fn with_lfsr(sink: T, lfsr: u32) -> CryptWrite<T> {
        CryptWrite {
            sink: sink,
            crypt: CryptStream::with_lfsr(lfsr)
        }
    }
}

impl<T: Write> Write for CryptWrite<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_all(&buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.sink.flush()
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        let mut encrypted = vec!(0u8; buf.len());
        self.crypt.crypt_to(&buf, &mut encrypted);
        self.sink.write_all(&encrypted)
    }
}