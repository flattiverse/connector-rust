use crate::hierarchy::ClusterId;
use crate::network::PacketReader;
use crate::unit::sub_components::SunSection;
use crate::unit::{CelestialBody, Unit, UnitKind};
use crate::Vector;

#[derive(Debug)]
pub struct Sun {
    body: CelestialBody,
    sections: Vec<SunSection>,
}

impl Sun {
    pub fn new(cluster: ClusterId, reader: &mut dyn PacketReader) -> Self {
        Self {
            body: CelestialBody::new(cluster, reader),
            sections: (0..reader.read_byte())
                .map(|_| SunSection::default().with_read(reader))
                .collect(),
        }
    }

    // TODO pub async fn configure
    // TODO pub async fn remove

    #[inline]
    pub fn sections(&self) -> &[SunSection] {
        &self.sections
    }
}

impl Unit for Sun {
    #[inline]
    fn name(&self) -> &str {
        &self.body.name
    }

    #[inline]
    fn cluster(&self) -> ClusterId {
        self.body.cluster
    }

    #[inline]
    fn position(&self) -> Vector {
        self.body.position
    }

    #[inline]
    fn gravity(&self) -> f64 {
        self.body.gravity
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.body.radius
    }

    fn update(&mut self, reader: &mut dyn PacketReader) {
        self.body.update(reader);
        self.sections = (0..reader.read_byte())
            .map(|_| SunSection::default().with_read(reader))
            .collect();
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Sun
    }
}
