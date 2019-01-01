
use std::io;
use sha2::Digest;
use sha2::Sha256;


use crate::Error;
use crate::Connector;
use crate::net::CryptRead;
use crate::net::CryptWrite;
use crate::net::BinaryReader;
use crate::net::BinaryWriter;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum PerformanceDiscreteMark {
    /// The performance of the measured system is so bad,
    /// that this test can't even measure it.
    ///
    /// Your PC is bad, and you should feel bad!
    Deficient = 0,
    /// Very poor
    Sufficient = 1,
    Poor = 2,
    Adequate = 3,
    Good = 4,
    VeryGood = 5,
    /// Did you buy your PC in the future
    /// and travelled back in time?
    TheSpaceTimeContinuumMayCrash = 6,
}

impl PerformanceDiscreteMark {
    pub fn for_average_mark(avg_mark: f64) -> PerformanceDiscreteMark {
        if avg_mark < 0.01 {
            PerformanceDiscreteMark::Deficient

        } else if avg_mark < 1.0 {
            PerformanceDiscreteMark::Sufficient

        } else if avg_mark < 2.0 {
            PerformanceDiscreteMark::Poor

        } else if avg_mark < 3.0 {
            PerformanceDiscreteMark::Adequate

        } else if avg_mark < 4.0 {
            PerformanceDiscreteMark::Good

        } else if avg_mark < 5.0 {
            PerformanceDiscreteMark:: VeryGood

        } else {
            PerformanceDiscreteMark::TheSpaceTimeContinuumMayCrash
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceMark {
    single_threaded_mark: f64,
    multi_threaded_mark: f64,
    memory_access_mark: f64,
    average_mark: f64,
    mark: PerformanceDiscreteMark,
    host: Option<String>,
}

impl Default for PerformanceMark {
    fn default() -> Self {
        PerformanceMark {
            single_threaded_mark: 0_f64,
            multi_threaded_mark:  0_f64,
            memory_access_mark:   0_f64,
            average_mark:         0_f64,
            mark: PerformanceDiscreteMark::Deficient,
            host: None,
        }
    }
}

impl PerformanceMark {
    pub fn from_save(single_threaded_measurement: i64, multi_threaded_measurement: i64, memory_access_measurement: i64, host: String) -> Result<PerformanceMark, Error> {

        let single_threaded_mark = (single_threaded_measurement as f64 / 268_435_456.0).ln().max(0f64);
        let multi_threaded_mark  = (multi_threaded_measurement  as f64 / 268_435_456.0).ln().max(0f64);
        let memory_access_mark   = (memory_access_measurement   as f64 / 1024.0     ).ln().max(0f64);
        let average_mark         = ((single_threaded_mark + multi_threaded_mark+ memory_access_mark) / 3.0).max(0f64);

        Ok(PerformanceMark {
            single_threaded_mark,
            multi_threaded_mark,
            memory_access_mark,
            host: Some(host),
            average_mark,
            mark: PerformanceDiscreteMark::for_average_mark(average_mark)
        })
    }

    pub fn from_reader(reader: &mut BinaryReader) -> Result<PerformanceMark, Error> {
        let single_threaded_mark = f64::from(reader.read_u16()?) / 100.0;
        let multi_threaded_mark  = f64::from(reader.read_u16()?) / 100.0;
        let memory_access_mark   = f64::from(reader.read_u16()?) / 100.0;
        let average_mark         = f64::from(reader.read_u16()?) / 100.0;

        Ok(PerformanceMark {
            single_threaded_mark,
            multi_threaded_mark,
            memory_access_mark,
            average_mark,
            mark: PerformanceDiscreteMark::for_average_mark(average_mark),
            host: None
        })
    }

    pub fn from_hash(hash: &[u8]) -> Result<PerformanceMark, Error> {
        if hash.len() != 64 {
            return Err(Error::InvalidHash)
        }

        let mut hasher = Sha256::default();
        hasher.input(Connector::hostname().as_bytes());
        let base_hash = hasher.result();

        let mut crypt = CryptRead::with_lfsr(
            &base_hash[..],
            u32::from(base_hash[1]) * 16_777_216_u32
                + u32::from(base_hash[14]) * 65_536_u32
                + u32::from(base_hash[5]) * 256_u32
                + u32::from(base_hash[7])
        );

        let reader = &mut crypt as &mut BinaryReader;

        let single_threaded_mark = reader.read_double()?;
        let multi_threaded_mark  = reader.read_double()?;
        let memory_access_mark   = reader.read_double()?;
        let average_mark         = reader.read_double()?;

        if single_threaded_mark.is_nan() || single_threaded_mark.is_infinite() ||
            multi_threaded_mark.is_nan() || multi_threaded_mark.is_infinite() ||
            memory_access_mark .is_nan() || memory_access_mark.is_infinite() ||
            average_mark.is_nan() || average_mark.is_infinite() {
            return Err(Error::InvalidHash)
        }

        Ok(PerformanceMark {
            single_threaded_mark,
            multi_threaded_mark,
            memory_access_mark,
            average_mark,
            mark: PerformanceDiscreteMark::for_average_mark(average_mark),
            host: Some(Connector::hostname())
        })
    }

    pub fn single_threaded_mark(&self) -> f64 {
        self.single_threaded_mark
    }

    pub fn multi_threaded_mark(&self) -> f64 {
        self.multi_threaded_mark
    }

    pub fn memory_access_mark(&self) -> f64 {
        self.memory_access_mark
    }

    pub fn average_mark(&self) -> f64 {
        self.average_mark
    }

    pub fn performance_discrete_mark(&self) -> PerformanceDiscreteMark {
        self.mark
    }

    pub fn write(&self, writer: &mut BinaryWriter) -> Result<(), io::Error> {
        if match self.host {
            None => true,
            Some(ref host) => !Connector::hostname().eq(host)
        } {
            // return writer.write_i64(0);
        }


        writer.write_u16((self.single_threaded_mark as f64 * 100.0 + 0.5) as u16)?;
        writer.write_u16((self.multi_threaded_mark  as f64 * 100.0 + 0.5) as u16)?;
        writer.write_u16((self.memory_access_mark   as f64 * 100.0 + 0.5) as u16)?;
        writer.write_u16((self.average_mark         as f64 * 100.0 + 0.5) as u16)?;
        Ok(())
    }

    pub fn generate_hash(&self) -> Result<Vec<u8>, Error> {
        if self.host.is_none() {
            return Err(Error::InvalidHostState);
        }

        let mut hasher = Sha256::default();
        hasher.input(Connector::hostname().as_bytes());
        let base_hash = hasher.result();

        let mut vec = Vec::new();

        {
            let mut crypt = CryptWrite::with_lfsr(
                &mut vec,
                u32::from(base_hash[1]) * 16_777_216_u32
                    + u32::from(base_hash[14]) * 65_536_u32
                    + u32::from(base_hash[5]) * 256_u32
                    + u32::from(base_hash[7])
            );

            let writer = &mut crypt as &mut BinaryWriter;

            writer.write_all(&base_hash)?;
            writer.write_f64(self.single_threaded_mark)?;
            writer.write_f64(self.multi_threaded_mark)?;
            writer.write_f64(self.memory_access_mark)?;
            writer.write_f64(self.average_mark)?;
        }

        Ok(vec)
    }
}

use std::fmt;

impl fmt::Display for PerformanceMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?} ({})", self.mark, self.average_mark)
    }
}