
use crate::Error;
use crate::net::BinaryWriter;

#[derive(Clone, Debug)]
pub struct ScanInfo {
    from_degree: f32,
    to_degree: f32,
    range: f32,
    span: f32
}

impl ScanInfo {
    pub fn new(mut from_degree: f32, mut to_degree: f32, range: f32) -> Result<ScanInfo, Error> {
        let span;

        if !from_degree.is_finite() {
            return Err(Error::InvalidFromDegree(from_degree));
        }

        if !to_degree.is_finite() {
            return Err(Error::InvalidToDegree(to_degree));
        }

        if !range.is_finite() {
            return Err(Error::InvalidRange(range));
        }

        while from_degree < 0_f32 {
            from_degree += 3_600_f32;
        }

        while to_degree < 0_f32 {
            to_degree += 3_600_f32;
        }

        from_degree %= 360_f32;
        to_degree   %= 360_f32;

        if (from_degree - to_degree).abs() <= ::std::f32::EPSILON {
            span = 360_f32;

        } else if from_degree < to_degree {
            span = to_degree - from_degree;

        } else {
            span = 360_f32 - from_degree + to_degree;
        }

        Ok(ScanInfo {
            from_degree,
            to_degree,
            range,
            span
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_degree(&self) -> f32 {
        self.from_degree
    }

    pub fn to_degree(&self) -> f32 {
        self.to_degree
    }

    pub fn range(&self) -> f32 {
        self.range
    }

    pub fn span(&self) -> f32 {
        self.span
    }

    pub fn write(&self, writer: &mut BinaryWriter) -> Result<(), Error> {
        writer.write_f32(self.from_degree)?;
        writer.write_f32(self.to_degree)?;
        writer.write_f32(self.range)?;
        Ok(())
    }
}