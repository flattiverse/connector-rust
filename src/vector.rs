use crate::network::{PacketReader, PacketWriter};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// Mutable two-dimensional vector with degree-based angle helpers.
/// The public fields [`Vector::x`] and [`Vector::y`] can be changed directly.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector {
    /// The X component of the vector.
    pub x: f32,
    /// The Y component of the vector.
    pub y: f32,
    pub(crate) last_angle: f32,
}

impl Vector {
    /// Creates a new vector with the given coordinates.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            last_angle: 0.0,
        }
    }

    #[inline]
    pub(crate) fn from_read(reader: &mut dyn PacketReader) -> Self {
        Self::default().with_read(reader)
    }

    #[inline]
    pub(crate) fn with_read(mut self, reader: &mut dyn PacketReader) -> Self {
        self.read(reader);
        self
    }

    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        if !reader.maybe_read_f32(&mut self.x) || !reader.maybe_read_f32(&mut self.y) {
            *self = Vector::default()
        }
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_f32(self.x);
        writer.write_f32(self.y);
    }

    /// Creates a vector from a direction and a length.
    ///
    /// * `x` Direction in degrees.
    /// * `length` Vector length.
    pub fn from_angle_length(angle: f32, length: f32) -> Self {
        Self::new(
            angle.to_radians().cos() * length,
            angle.to_radians().sin() * length,
        )
    }

    /// Direction of the vector in degrees.
    pub fn angle(&self) -> f32 {
        if self.x == 0.0 && self.y == 0.0 {
            self.last_angle
        } else {
            (self.y.atan2(self.x).to_degrees() + 360.0) % 360.0
        }
    }

    /// Setting this keeps the current [`Vector::length`] and rotates the vector accordingly.
    pub fn set_angle(&mut self, value: f32) {
        let alpha = value * std::f32::consts::PI / 180.0;
        let length = self.length();

        self.x = length * alpha.cos();
        self.y = length * alpha.sin();
    }

    /// Squared length of the vector.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    /// Length of the vector.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Setting this scales the vector while keeping its current direction whenever possible.
    pub fn set_length(&mut self, length: f32) {
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

    /// Helper: Consumes itself, applies [`Vector::set_length`] and returns itself.
    #[inline]
    pub fn with_length(mut self, length: f32) -> Self {
        self.set_length(length);
        self
    }

    /// Rotates the vector in place by the specified angle in degrees.
    ///
    /// * `degree` Rotation angle in degrees.
    pub fn rotated_by(&self, degree: f32) -> Self {
        let alpha = degree.to_radians();
        Self::new(
            alpha.cos().mul(self.x) - alpha.sin().mul(self.y),
            alpha.sin().mul(self.x) + alpha.cos().mul(self.y),
        )
    }

    /// Returns the wrapped angle difference in degrees between this vector and another vector.
    pub fn angle_from(&self, other: &Vector) -> f32 {
        let mut degree = other.last_angle - self.last_angle;
        if degree < 0.0 {
            degree += 360.0;
        }
        degree
    }

    /// Whether either component contains `NaN` or an infinity value.
    pub fn is_damaged(&self) -> bool {
        self.x.is_infinite() || self.x.is_nan() || self.y.is_infinite() || self.y.is_nan()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length)
    }
}

impl Add for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Add::add(*self, rhs)
    }
}

impl Sub for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Sub::sub(*self, rhs)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}
