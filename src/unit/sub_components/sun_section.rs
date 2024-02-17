use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::SunConfiguration;
use crate::{GameError, GameErrorKind};

#[derive(Debug, Clone, Default)]
pub struct SunSection {
    inner_radius: f64,
    outer_radius: f64,
    angel_from: f64,
    angel_to: f64,
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
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.inner_radius = reader.read_2u(100.0);
        self.outer_radius = reader.read_2u(100.0);
        self.angel_from = reader.read_2u(100.0);
        self.angel_to = reader.read_2u(100.0);

        self.energy = reader.read_2u(100.0);
        self.ions = reader.read_2u(100.0);
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_2u(self.inner_radius, 100.0);
        writer.write_2u(self.outer_radius, 100.0);
        writer.write_2u(self.angel_from, 100.0);
        writer.write_2u(self.angel_to, 100.0);

        writer.write_2u(self.energy, 100.0);
        writer.write_2u(self.ions, 100.0);
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

    #[inline]
    pub fn angel_from(&self) -> f64 {
        self.angel_from
    }

    pub fn set_angel_from(&mut self, angle: f64) -> Result<(), GameError> {
        if angle.is_infinite() || angle.is_nan() || angle < 0.0 || angle >= self.angel_to {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.angel_from = angle;
            Ok(())
        }
    }

    #[inline]
    pub fn angel_to(&self) -> f64 {
        self.angel_to
    }

    pub fn set_angel_to(&mut self, angle: f64) -> Result<(), GameError> {
        if angle.is_infinite() || angle.is_nan() || angle > 360.0 || self.angel_from >= angle {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.angel_to = angle;
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