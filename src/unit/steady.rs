use crate::network::PacketReader;
use crate::utils::{Atomic, Readable};
use crate::Vector;

#[derive(Debug)]
pub struct SteadyUnit {
    gravity: Atomic<f32>,
    radius: Atomic<f32>,
    position: Atomic<Vector>,
}

impl SteadyUnit {
    #[inline]
    pub fn gravity(&self) -> f32 {
        self.gravity.load()
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius.load()
    }

    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }
}

impl Readable for SteadyUnit {
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self {
            position: Atomic::from_reader(reader),
            radius: Atomic::from_reader(reader),
            gravity: Atomic::from_reader(reader),
        }
    }
}
