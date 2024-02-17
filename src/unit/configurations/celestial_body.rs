use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::unit_configuration::UnitConfiguration;
use crate::{GameError, GameErrorKind, Vector};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct CelestialBodyConfiguration {
    pub(crate) base: UnitConfiguration,
    pub(crate) position: Vector,
    pub(crate) radius: f64,
    pub(crate) graity: f64,
}

impl CelestialBodyConfiguration {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.base.read(reader);
        self.position.read(reader);
        self.radius = reader.read_4u(100.0);
        self.graity = reader.read_4s(10000.0);

        // reserved for orbiting units
        let _ = reader.read_byte();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        self.base.write(writer);
        self.position.write(writer);
        writer.write_4u(self.radius, 100.0);
        writer.write_4s(self.graity, 10000.0);

        // no orbiting configuration
        writer.write_byte(0x00);
    }

    #[inline]
    pub fn position(&self) -> Vector {
        self.position
    }

    pub fn set_position(&mut self, position: Vector) -> Result<(), GameError> {
        if position.is_damaged()
            || position.x < -20000.0
            || position.y < -20000.0
            || position.x > 20000.0
            || position.y > 20000.0
        {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else {
            self.position = position;
            Ok(())
        }
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f64) -> Result<(), GameError> {
        if radius.is_infinite() || radius.is_nan() || radius < 0.001 || radius > 2000.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else {
            self.radius = radius;
            Ok(())
        }
    }

    #[inline]
    pub fn gravity(&self) -> f64 {
        self.graity
    }

    pub fn set_gravity(&mut self, gravity: f64) -> Result<(), GameError> {
        if gravity.is_infinite() || gravity.is_nan() || gravity < 30.0 || gravity > 30.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else {
            self.graity = gravity;
            Ok(())
        }
    }
}

impl Deref for CelestialBodyConfiguration {
    type Target = UnitConfiguration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CelestialBodyConfiguration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
