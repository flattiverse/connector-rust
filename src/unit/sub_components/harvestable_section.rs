use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::HarvestableConfiguration;
use crate::{GameError, GameErrorKind};

#[derive(Debug, Clone, Default)]
pub struct HarvestableSection {
    inner_radius: f64,
    outer_radius: f64,
    angle_from: f64,
    angle_to: f64,

    iron: f64,
    silicon: f64,
    tungsten: f64,
    tritium: f64,

    configuration: Option<HarvestableConfiguration>,
}

impl From<Option<HarvestableConfiguration>> for HarvestableSection {
    fn from(configuration: Option<HarvestableConfiguration>) -> Self {
        Self {
            inner_radius: 100.0,
            outer_radius: 130.0,
            angle_from: 45.0,
            angle_to: 135.0,
            iron: 1.0,
            silicon: 1.0,
            tungsten: 1.0,
            tritium: 1.0,
            configuration,
        }
    }
}

impl HarvestableSection {
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

        self.iron = reader.read_double();
        self.silicon = reader.read_double();
        self.tungsten = reader.read_double();
        self.tritium = reader.read_double();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_double(self.inner_radius);
        writer.write_double(self.outer_radius);
        writer.write_double(self.angle_from);
        writer.write_double(self.angle_to);

        writer.write_double(self.iron);
        writer.write_double(self.silicon);
        writer.write_double(self.tungsten);
        writer.write_double(self.tritium);
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

    /// The inner radius (radius which is nearer to the harvestable) of this [`Section`].
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

    /// The outer radius (radius which is farer away from the harvestable) of this [`Section`].
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
        if from.is_infinite() || from.is_nan() || from < 0.0 || from > 360.0 || from == to {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if to.is_infinite() || to.is_nan() || to > 360.0 || to < 0.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.angle_from = from;
            self.angle_to = to;
            Ok(())
        }
    }

    /// The left angle, when you look from the middle point of the harvestable to this [`Section`].
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

    /// The right angle, when you look from the middle point of the harvestable to this [`Section`]
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

    /// The iron output of this area. This value multiplied with the extractor results in the iron
    /// loaded per second.
    #[inline]
    pub fn iron(&self) -> f64 {
        self.iron
    }

    pub fn set_iron(&mut self, iron: f64) -> Result<(), GameError> {
        if iron.is_infinite() || iron.is_nan() || iron > 500.0 || iron < -500.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.iron = iron;
            Ok(())
        }
    }

    /// The silicon output of this area. This value multiplied with the extractor results in the
    /// silicon loaded per second.
    #[inline]
    pub fn silicon(&self) -> f64 {
        self.silicon
    }

    pub fn set_silicon(&mut self, silicon: f64) -> Result<(), GameError> {
        if silicon.is_infinite() || silicon.is_nan() || silicon > 500.0 || silicon < -500.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.silicon = silicon;
            Ok(())
        }
    }

    /// The tungsten output of this area. This value multiplied with the extractor results in the
    /// tungsten loaded per second.
    #[inline]
    pub fn tungsten(&self) -> f64 {
        self.tungsten
    }

    pub fn set_tungsten(&mut self, tungsten: f64) -> Result<(), GameError> {
        if tungsten.is_infinite() || tungsten.is_nan() || tungsten > 500.0 || tungsten < -500.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.tungsten = tungsten;
            Ok(())
        }
    }

    /// The tritium output of this area. This value multiplied with the extractor results in the
    /// tritium loaded per second.
    #[inline]
    pub fn tritium(&self) -> f64 {
        self.tritium
    }

    pub fn set_tritium(&mut self, tritium: f64) -> Result<(), GameError> {
        if tritium.is_infinite() || tritium.is_nan() || tritium > 500.0 || tritium < -500.0 {
            Err(GameErrorKind::ParameterNotWithinSpecification.into())
        } else if self.configuration.is_none() {
            Err(GameErrorKind::NotConfigurable.into())
        } else {
            self.tritium = tritium;
            Ok(())
        }
    }
}
