use crate::atomics::Atomic;
use crate::network::PacketReader;
use crate::Vector;

#[derive(Debug)]
pub struct CelestialBody {
    pub(crate) name: String,
    pub(crate) position: Atomic<Vector>,
    pub(crate) radius: Atomic<f64>,
    pub(crate) gravity: Atomic<f64>,
}

impl CelestialBody {
    pub(crate) fn new(reader: &mut dyn PacketReader) -> Self {
        Self {
            name: reader.read_string(),
            position: Atomic::from_reader(reader),
            radius: Atomic::from_reader(reader),
            gravity: {
                let gravity = Atomic::from_reader(reader);

                let _orbiting = reader.read_byte();

                gravity
            },
        }
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        let _ = reader.read_string(); // 'jump over string'

        self.position.read(reader);
        self.radius.read(reader);
        self.gravity.read(reader);

        let _orbiting = reader.read_byte();
    }
}
