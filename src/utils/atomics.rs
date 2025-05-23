use crate::galaxy_hierarchy::{GameMode, PlayerId, TeamId};
use crate::network::{PacketReader, PacketWriter};
use crate::Vector;
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
impl_read_write_for_atomar!(f32, read_f32, write_f32);

// macro_rules! impl_atomar_for_try_primitive {
//     ($primitive:ty, $($ty:ty),+) => {
//         $(
//             impl Atomar for $ty {
//                 type Container = <$primitive as Atomar>::Container;
//
//                 #[inline]
//                 fn into_container(self) -> Self::Container {
//                     <$primitive as Atomar>::into_container(self.into())
//                 }
//
//                 #[inline]
//                 fn store(self, container: &Self::Container, ordering: Ordering) {
//                     container.store(self.into(), ordering)
//                 }
//
//                 #[inline]
//                 fn load(container: &Self::Container, ordering: Ordering) -> Self {
//                     use num_enum::TryFromPrimitive;
//                     // .unwrap unreachable because the value was initialized by .into_container()
//                     <$ty>::try_from_primitive(container.load(ordering)).unwrap()
//                 }
//             }
//         )+
//     };
// }

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
                    use num_enum::FromPrimitive;
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
    type Container = (Atomic<u64>, Atomic<f32>);

    fn into_container(self) -> Self::Container {
        (
            Atomic::from({
                let bits_x = self.x.to_le_bytes();
                let bits_y = self.y.to_le_bytes();
                u64::from_le_bytes([
                    bits_x[0], bits_x[1], bits_x[2], bits_x[3], bits_y[0], bits_y[1], bits_y[2],
                    bits_y[3],
                ])
            }),
            Atomic::from(self.last_angle),
        )
    }

    fn store(self, container: &Self::Container, ordering: Ordering) {
        container.0.store_with(
            {
                let bits_x = self.x.to_le_bytes();
                let bits_y = self.y.to_le_bytes();
                u64::from_le_bytes([
                    bits_x[0], bits_x[1], bits_x[2], bits_x[3], bits_y[0], bits_y[1], bits_y[2],
                    bits_y[3],
                ])
            },
            ordering,
        );
        container.1.store_with(self.last_angle, ordering);
    }

    fn load(container: &Self::Container, ordering: Ordering) -> Self {
        let bytes = container.0.load_with(ordering).to_le_bytes();
        Vector {
            x: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            last_angle: container.1.load_with(ordering),
        }
    }
}

impl Readable for Vector {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self::default().with_read(reader)
    }
}

impl Writable for Vector {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        Vector::write(self, writer);
    }
}

impl Atomic<bool> {
    /// See [`AtomicBool::fetch_and`].
    #[inline]
    pub fn and_assign(&self, rhs: bool) {
        self.0.fetch_and(rhs, Ordering::Relaxed);
    }

    /// See [`AtomicBool::fetch_nand`].
    #[inline]
    pub fn nand_assign(&self, rhs: bool) {
        self.0.fetch_nand(rhs, Ordering::Relaxed);
    }

    /// See [`AtomicBool::fetch_or`].
    #[inline]
    pub fn or_assign(&self, rhs: bool) {
        self.0.fetch_or(rhs, Ordering::Relaxed);
    }

    /// See [`AtomicBool::fetch_xor`].
    #[inline]
    pub fn xor_assign(&self, rhs: bool) {
        self.0.fetch_xor(rhs, Ordering::Relaxed);
    }
}
