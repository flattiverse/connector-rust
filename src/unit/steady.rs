use crate::network::PacketReader;
use crate::unit::{UnitBase, UnitExt, UnitExtSealed};
use crate::utils::{Atomic, Readable};
use crate::Vector;

#[derive(Debug, Clone)]
pub struct SteadyUnit {
    gravity: Atomic<f32>,
    radius: Atomic<f32>,
    position: Atomic<Vector>,
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

impl<'a> UnitExtSealed<'a> for (&'a UnitBase, &'a SteadyUnit)
where
    Self: 'a,
{
    type Parent = &'a UnitBase;

    #[inline]
    fn parent(self) -> Self::Parent {
        self.0
    }
}

impl<'b> UnitExt<'b> for (&'b UnitBase, &'b SteadyUnit) {
    #[inline]
    fn radius(self) -> f32 {
        self.1.radius.load()
    }

    #[inline]
    fn position(self) -> Vector {
        self.1.position.load()
    }

    #[inline]
    fn gravity(self) -> f32 {
        self.1.gravity.load()
    }
}
