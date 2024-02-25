use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::SunConfiguration;
use crate::{GameError, GameErrorKind};

#[derive(Debug, Clone, Default)]
pub struct SunSection {
    inner_radius: f64,
    outer_radius: f64,
    angle_from: f64,
    angle_to: f64,
    energy: f64,
    ions: f64,
    configuration: Option<SunConfiguration>,
}

impl From<Option<SunConfiguration>> for SunSection {
    fn from(configuration: Option<SunConfiguration>) -> Self {
        Self {
            configuration,
            ..Default::default()
        }
    }
}

impl SunSection {
    #[inline]
    pub(crate) fn with_read(mut self, reader: &mut dyn PacketReader) -> Self {
        self.read(reader);
        self
    }

    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.inner_radius = reader.read_double();
        self.outer_radius = reader.read_double();
        self.angle_from = reader.read_double();
        self.angle_to = reader.read_double();

        self.energy = reader.read_double();
        self.ions = reader.read_double();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_double(self.inner_radius);
        writer.write_double(self.outer_radius);
        writer.write_double(self.angle_from);
        writer.write_double(self.angle_to);

        writer.write_double(self.energy);
        writer.write_double(self.ions);
    }

    /// Sets the radius for the inner and the outer radius at once.
    pub fn set_radii(&mut self, inner: f64, outer: f64) -> Result<(), GameError> {
        if inner.is_infinite() || inner.is_nan() || inner < 0.0 || inner >= outer {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if outer.is_infinite() || outer.is_nan() || outer > 2000.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.inner_radius = inner;
            self.outer_radius = outer;
            Ok(())
        }
    }

    #[inline]
    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }

    pub fn set_inner_radius(&mut self, radius: f64) -> Result<(), GameError> {
        if radius.is_infinite() || radius.is_nan() || radius < 0.0 || radius >= self.outer_radius {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.inner_radius = radius;
            Ok(())
        }
    }

    #[inline]
    pub fn outer_radius(&self) -> f64 {
        self.outer_radius
    }

    pub fn set_outer_radius(&mut self, radius: f64) -> Result<(), GameError> {
        if radius.is_infinite() || radius.is_nan() || radius < 0.0 || self.inner_radius >= radius {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.outer_radius = radius;
            Ok(())
        }
    }

    /// Sets the angle for the left (from) and the right (to) side at once.
    pub fn set_angles(&mut self, from: f64, to: f64) -> Result<(), GameError> {
        if from.is_infinite() || from.is_nan() || from < 0.0 || to > 360.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if to.is_infinite() || to.is_nan() || from < 0.0 || to > 360.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.angle_from = from;
            self.angle_to = to;
            Ok(())
        }
    }

    #[inline]
    pub fn angle_from(&self) -> f64 {
        self.angle_from
    }

    pub fn set_angle_from(&mut self, angle: f64) -> Result<(), GameError> {
        if angle.is_infinite() || angle.is_nan() || angle < 0.0 || angle > 360.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.angle_from = angle;
            Ok(())
        }
    }

    #[inline]
    pub fn angle_to(&self) -> f64 {
        self.angle_to
    }

    pub fn set_angle_to(&mut self, angle: f64) -> Result<(), GameError> {
        if angle.is_infinite() || angle.is_nan() || angle > 360.0 || angle < 0.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.angle_to = angle;
            Ok(())
        }
    }

    #[inline]
    pub fn energy(&self) -> f64 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: f64) -> Result<(), GameError> {
        if energy.is_infinite() || energy.is_nan() || energy > 500.0 || energy < -500.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.energy = energy;
            Ok(())
        }
    }

    #[inline]
    pub fn ions(&self) -> f64 {
        self.ions
    }

    pub fn set_ions(&mut self, ions: f64) -> Result<(), GameError> {
        if ions.is_infinite() || ions.is_nan() || ions > 50.0 || ions < -50.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.ions = ions;
            Ok(())
        }
    }
}
