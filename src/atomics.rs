use crate::hierarchy::GalaxyId;
use crate::network::{PacketReader, PacketWriter};
use crate::Vector;
use std::fmt::{Debug, Display, Formatter};
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering};

/// Everything that shall be read and written to with an immutable reference. Might lie to you to
/// enable that (see the implementation for [`Vector`] for example). In the use case of Flattiverse
/// this is considered fine or at least good enough.
#[derive(Debug, Copy, Clone, Default)]
pub struct Atomic<T: Atomar>(T::Container);

impl<T: Atomar> From<T> for Atomic<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into_container())
    }
}

impl<T: Atomar> Atomic<T> {
    #[inline]
    pub fn from_reader(reader: &mut dyn PacketReader) -> Self
    where
        T: Readable,
    {
        From::<T>::from(T::read(reader))
    }

    #[inline(always)]
    pub fn store(&self, value: T) {
        self.store_with(value, Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn store_with(&self, value: T, ordering: Ordering) {
        value.store(&self.0, ordering)
    }

    #[inline(always)]
    pub fn load(&self) -> T {
        self.load_with(Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn load_with(&self, ordering: Ordering) -> T {
        T::load(&self.0, ordering)
    }
}

impl<T: Atomar> Atomic<T>
where
    T: Readable,
{
    #[inline]
    pub fn read(&self, reader: &mut dyn PacketReader) {
        self.store(T::read(reader))
    }
}

impl<T: Atomar> Atomic<T>
where
    T: Writable,
{
    #[inline]
    pub fn write(&self, writer: &mut dyn PacketWriter) {
        self.load().write(writer)
    }
}

impl<T: Atomar> Display for Atomic<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.load(), f)
    }
}

pub trait Atomar {
    type Container;

    fn into_container(self) -> Self::Container;

    fn store(self, container: &Self::Container, ordering: Ordering);

    fn load(container: &Self::Container, ordering: Ordering) -> Self;
}

pub trait Readable {
    fn read(reader: &mut dyn PacketReader) -> Self;
}

pub trait Writable {
    fn write(&self, writer: &mut dyn PacketWriter);
}

impl Atomar for bool {
    type Container = AtomicBool;

    #[inline]
    fn into_container(self) -> Self::Container {
        AtomicBool::from(self)
    }

    #[inline(always)]
    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.store(self, ordering)
    }

    #[inline(always)]
    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        container.load(ordering)
    }
}

impl Atomar for u16 {
    type Container = AtomicU16;

    #[inline]
    fn into_container(self) -> Self::Container {
        AtomicU16::from(self)
    }

    #[inline(always)]
    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.store(self, ordering)
    }

    #[inline(always)]
    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        container.load(ordering)
    }
}

impl Readable for u16 {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        reader.read_uint16()
    }
}

impl Writable for u16 {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_uint16(*self)
    }
}

impl Atomar for f64 {
    type Container = AtomicU64;

    #[inline]
    fn into_container(self) -> Self::Container {
        AtomicU64::new(self.to_bits())
    }

    #[inline(always)]
    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.store(self.to_bits(), ordering)
    }

    #[inline(always)]
    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        f64::from_bits(container.load(ordering))
    }
}

impl Readable for f64 {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        reader.read_double()
    }
}

impl Writable for f64 {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_double(*self)
    }
}

/// This implementation is quite the lie...
impl Atomar for Vector {
    type Container = (Atomic<f64>, Atomic<f64>, Atomic<f64>);

    fn into_container(self) -> Self::Container {
        (
            Atomic::from(self.x),
            Atomic::from(self.y),
            Atomic::from(self.last_angle),
        )
    }

    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.0.store_with(self.x, ordering);
        container.1.store_with(self.y, ordering);
        container.2.store_with(self.last_angle, ordering);
    }

    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        Vector {
            x: container.0.load_with(ordering),
            y: container.1.load_with(ordering),
            last_angle: container.2.load_with(ordering),
        }
    }
}

impl Readable for Vector {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        Vector::from_xy(reader.read_double(), reader.read_double())
    }
}

impl Writable for Vector {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_double(self.x);
        writer.write_double(self.y);
    }
}

impl Atomar for GalaxyId {
    type Container = AtomicU16;

    fn into_container(self) -> Self::Container {
        Self::Container::new(self.0)
    }

    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.store(self.0, ordering)
    }

    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        Self(container.load(ordering))
    }
}
