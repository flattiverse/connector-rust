use crate::galaxy_hierarchy::{GameMode, PlayerId, TeamId};
use crate::network::{PacketReader, PacketWriter};
use crate::Vector;
use num_enum::FromPrimitive;
use std::fmt::{Debug, Display, Formatter};
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};

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

macro_rules! impl_atomar_for_std {
    ($primitive:ty, $std:ty) => {
        impl Atomar for $primitive {
            type Container = $std;

            #[inline]
            fn into_container(self) -> Self::Container {
                <$std>::from(self)
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
    };
}

impl_atomar_for_std!(bool, AtomicBool);
impl_atomar_for_std!(u8, AtomicU8);
impl_atomar_for_std!(u16, AtomicU16);
impl_atomar_for_std!(u32, AtomicU32);
impl_atomar_for_std!(u64, AtomicU64);

macro_rules! impl_atomar_for_bits {
    ($primitive:ty, $base:ty) => {
        impl Atomar for $primitive {
            type Container = $base;

            #[inline]
            fn into_container(self) -> Self::Container {
                <$base>::from(self.to_bits())
            }

            #[inline(always)]
            fn store(self, container: &Self::Container, ordering: Ordering) {
                container.store(self.to_bits(), ordering)
            }

            #[inline(always)]
            fn load(container: &Self::Container, ordering: Ordering) -> Self {
                Self::from_bits(container.load(ordering))
            }
        }
    };
}

impl_atomar_for_bits!(f32, AtomicU32);
impl_atomar_for_bits!(f64, AtomicU64);

macro_rules! impl_read_write_for_atomar {
    ($ty:ty, $read:ident, $write:ident) => {
        impl Readable for $ty {
            #[inline]
            fn read(reader: &mut dyn PacketReader) -> Self {
                reader.$read()
            }
        }

        impl Writable for $ty {
            #[inline]
            fn write(&self, writer: &mut dyn PacketWriter) {
                writer.$write(*self)
            }
        }
    };
}

impl_read_write_for_atomar!(u16, read_uint16, write_uint16);
impl_read_write_for_atomar!(u32, read_uint32, write_uint32);
impl_read_write_for_atomar!(f64, read_double, write_double);

macro_rules! impl_atomar_for_primitive {
    ($primitive:ty, $($ty:ty),+) => {
        $(
            impl Atomar for $ty {
                type Container = <$primitive as Atomar>::Container;

                #[inline]
                fn into_container(self) -> Self::Container {
                    <$primitive as Atomar>::into_container(self.into())
                }

                #[inline]
                fn store(self, container: &Self::Container, ordering: Ordering) {
                    container.store(self.into(), ordering)
                }

                #[inline]
                fn load(container: &Self::Container, ordering: Ordering) -> Self {
                    <$ty>::from_primitive(container.load(ordering))
                }
            }
        )+
    };
}

impl_atomar_for_primitive!(u8, GameMode);

macro_rules! impl_atomar_for_id {
    ($primitive:ty, $($ty:path),+) => {
        $(
            impl Atomar for $ty {
                type Container = <$primitive as Atomar>::Container;

                #[inline]
                fn into_container(self) -> Self::Container {
                    <$primitive as Atomar>::into_container(self.0)
                }

                #[inline]
                fn store(self, container: &Self::Container, ordering: Ordering) {
                    container.store(self.0, ordering)
                }

                #[inline]
                fn load(container: &Self::Container, ordering: Ordering) -> Self {
                    $ty(container.load(ordering))
                }
            }
        )+
    };
}

impl_atomar_for_id!(u8, PlayerId, TeamId);

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
