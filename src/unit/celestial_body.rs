use crate::hierarchy::ClusterId;
use crate::network::PacketReader;
use crate::Vector;

#[derive(Debug)]
pub struct CelestialBody {
    pub(crate) name: String,
    pub(crate) cluster: ClusterId,
    pub(crate) position: Vector,
    pub(crate) radius: f64,
    pub(crate) gravity: f64,
}

impl CelestialBody {
    pub(crate) fn new(cluster: ClusterId, reader: &mut dyn PacketReader) -> Self {
        Self {
            cluster,
            name: reader.read_string(),
            position: Vector::default().with_read(reader),
            radius: reader.read_3u(1_000.0),
            gravity: reader.read_4u(10_000.0),
        }
    }

    pub(crate) fn update(&mut self, reader: &mut dyn PacketReader) {
        self.position = Vector::default().with_read(reader);
        self.radius = reader.read_3u(1_000.0);
        self.gravity = reader.read_4u(10_000.0);
    }
}
