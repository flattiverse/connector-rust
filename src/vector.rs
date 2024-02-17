use crate::network::{PacketReader, PacketWriter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    last_angle: f64,
}

impl Vector {
    pub const fn from_xy(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            last_angle: 0.0,
        }
    }

    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.x = reader.read_4s(100000.0);
        self.y = reader.read_4s(100000.0);
        self.last_angle = 0.0;
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        if self.is_damaged() {
            writer.write_uint64(0);
        } else {
            if self.x < -21470.0 {
                writer.write_int32(-2147000000)
            } else if self.x > 21470.0 {
                writer.write_int32(2147000000)
            } else {
                writer.write_4s(self.x, 100000.0)
            }

            if self.y < -21470.0 {
                writer.write_int32(-2147000000)
            } else if self.y > 21470.0 {
                writer.write_int32(2147000000)
            } else {
                writer.write_4s(self.y, 100000.0)
            }
        }
    }

    pub fn from_angle_length(angle: f64, length: f64) -> Self {
        Self::from_xy(
            angle.to_radians().cos() * length,
            angle.to_radians().sin() * length,
        )
    }

    pub fn angle(&self) -> f64 {
        if self.x == 0.0 && self.y == 0.0 {
            self.last_angle
        } else {
            (self.y.atan2(self.x).to_degrees() + 360.0) % 360.0
        }
    }

    pub fn set_angle(&mut self, value: f64) {
        let alpha = value * std::f64::consts::PI / 180.0;
        let length = self.length();

        self.x = length * alpha.cos();
        self.y = length * alpha.sin();
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn set_length(&mut self, length: f64) {
        if length == 0.0 {
            self.last_angle = self.angle();
        }

        if self.x == 0.0 && self.y == 0.0 {
            let alpha = self.last_angle.to_radians();
            self.x = length * alpha.cos();
            self.y = length * alpha.sin();
        } else {
            let length_factor = length / self.length();
            self.x *= length_factor;
            self.y *= length_factor;
        }
    }

    pub fn with_length(mut self, length: f64) -> Self {
        self.set_length(length);
        self
    }

    pub fn rotated_by(&self, degree: f64) -> Self {
        let alpha = degree.to_radians();
        Self::from_xy(
            alpha.cos().mul(self.x) - alpha.sin().mul(self.y),
            alpha.sin().mul(self.x) + alpha.cos().mul(self.y),
        )
    }

    pub fn angle_from(&self, other: &Vector) -> f64 {
        let mut degree = other.last_angle - self.last_angle;
        if degree < 0.0 {
            degree += 360.0;
        }
        degree
    }

    pub fn is_damaged(&self) -> bool {
        self.x.is_infinite() || self.x.is_nan() || self.y.is_infinite() || self.y.is_nan()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self::from_xy(self.x / length, self.y / length)
    }
}

impl Add for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector::from_xy(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::from_xy(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::from_xy(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vector::from_xy(self.x / rhs, self.y / rhs)
    }
}
