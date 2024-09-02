use crate::network::PacketReader;
use crate::runtime::{Atomic, Readable};

#[derive(Debug)]
pub struct SteadyUnit {
    gravity: Atomic<f32>,
}

impl SteadyUnit {
    #[inline]
    pub fn gravity(&self) -> f32 {
        self.gravity.load()
    }
}

impl Readable for SteadyUnit {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            gravity: Atomic::from_reader(reader),
        }
    }
}
