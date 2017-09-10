
use std::fmt;
use std::sync::Arc;

use Task;
use Error;
use Connector;
use net::BinaryReader;
use net::BinaryWriter;

const TOLERANCE : f32 = 0.25f32;

#[derive(Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    last_angle: f32,
    connector: Option<Arc<Connector>>,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector {
            x,
            y,
            last_angle: 0f32,
            connector: None,
        }
    }

    pub fn from_reader(reader: &mut BinaryReader) -> Result<Vector, Error> {
        Ok(Vector {
            x: reader.read_single()?,
            y: reader.read_single()?,
            last_angle: 0f32,
            connector: None,
        })
    }

    pub fn from_reader_with_connector(reader: &mut BinaryReader, connector: &Arc<Connector>) -> Result<Vector, Error> {
        Ok(Vector {
            x: reader.read_single()?,
            y: reader.read_single()?,
            last_angle: 0f32,
            connector: Some(connector.clone()),
        })
    }

    pub fn from_angle_length(angle: f32, length: f32) -> Vector {
        Vector {
            x: angle.to_radians().cos() * length,
            y: angle.to_radians().sin() * length,
            last_angle: angle,
            connector: None
        }
    }

    pub fn write(&self, writer: &mut BinaryWriter) -> Result<(), Error> {
        writer.write_f32(self.x)?;
        writer.write_f32(self.y)?;
        Ok(())
    }

    pub fn x(&self) -> f32 {
        if let Some(ref connector) = self.connector {
            connector.register_task_quitely_if_unknown(Task::UsedVector);
        }
        self.x
    }

    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn angle(&self) -> f32 {
        if self.x == 0f32 && self.y == 0f32 {
            self.last_angle
        } else {
            (self.y.atan2(self.x).to_degrees() + 360f32) % 360f32
        }
    }

    pub fn set_angle(&mut self, angle: f32) -> &mut Self {
        let alpha = angle.to_radians();
        let length = self.length();

        self.x = length * alpha.cos();
        self.y = length * alpha.sin();
        self
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    pub fn set_length(&mut self, length: f32) -> &mut Self {
        if length == 0f32 {
            self.last_angle = self.angle();
        }

        if self.x == 0f32 && self.y == 0f32 {
            let alpha = self.last_angle.to_radians();
            self.x = length * alpha.cos();
            self.y = length * alpha.sin();

        } else {
            let length_factor = length / self.length();
            self.x *= length_factor;
            self.y *= length_factor;
        }
        self
    }

    pub fn rotate_by(&self, angle: f32) -> Vector {
        let alpha = angle.to_radians();
        let cos = alpha.cos();
        let sin = alpha.sin();
        Vector::new(
            cos * self.x - sin * self.y,
            sin * self.x + cos * self.y
        )
    }

    pub fn get_angle_from(&self, other: &Vector) -> f32 {
        let mut deg = other.angle() - self.angle();

        if deg < 0f32 {
            deg += 360f32;
        }

        deg
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }

    pub fn mul(&self, factor: f32) -> Vector {
        let mut vector = Vector::new(self.x * factor, self.y * factor);

        if factor == 0f32 {
            vector.last_angle = self.angle();
        }

        vector
    }

    pub fn div(&self, divisor: f32) -> Vector {
        Vector::new(self.x / divisor, self.y / divisor)
    }

    pub fn bigger_than(&self, other: &Vector) -> bool {
        (self.x*self.x + self.y*self.y) > (other.x*other.x + other.y*other.y)
    }

    pub fn bigger_than_length(&self, length: f32) -> bool {
        (self.x*self.x + self.y*self.y) > (length*length)
    }

    pub fn smaller_than(&self, other: &Vector) -> bool {
        (self.x*self.x + self.y*self.y) < (other.x*other.x + other.y*other.y)
    }

    pub fn smaller_than_length(&self, length: f32) -> bool {
        (self.x*self.x + self.y*self.y) < (length*length)
    }

    /// Checks if the length of the given [Vector]
    /// matches the given value with a certain tolerance
    pub fn equals(vec: &Vector, r: f32) -> bool {
        let length = vec.length();
        length - TOLERANCE < r && length + TOLERANCE > r
    }

    pub fn negate(&self) -> Vector {
        Vector::new(-self.x, -self.y)
    }

    pub fn damaged(&self) -> bool {
        !self.x.is_finite() || !self.y.is_finite()
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        Vector::equals(self, other.length())
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector: ")?;
        (&self as &fmt::Display).fmt(f)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.damaged() {
            write!(f, "DAMAGED: ")?;
        }

        if self.x.is_infinite() {
            write!(f, "{}INV", if self.x > 0f32 {"+"} else {"-"})?;

        } else if self.x.is_nan() {
            write!(f, "NAN")?;

        } else {
            write!(f, "{}", self.x)?;
        }

        write!(f, "/")?;

        if self.y.is_infinite() {
            write!(f, "{}INV", if self.y > 0f32 {"+"} else {"-"})?;

        } else if self.y.is_nan() {
            write!(f, "NAN")?;

        } else {
            write!(f, "{}", self.y)?;
        }

        Ok(())
    }
}