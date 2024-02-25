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
            radius: reader.read_double(),
            gravity: reader.read_double(),
        }
    }

    pub(crate) fn update(&mut self, reader: &mut dyn PacketReader) {
        self.position = Vector::default().with_read(reader);
        self.radius = reader.read_double();
        self.gravity = reader.read_double();
    }
}
