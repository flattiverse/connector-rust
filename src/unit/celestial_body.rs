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
            gravity: {
                let gravity = reader.read_double();

                let _orbiting = reader.read_byte();

                gravity
            },
        }
    }

    pub(crate) fn update(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string(); // 'jump over string'
        self.position = Vector::default().with_read(reader);
        self.radius = reader.read_double();
        self.gravity = reader.read_double();

        let _orbiting = reader.read_byte();
    }
}
