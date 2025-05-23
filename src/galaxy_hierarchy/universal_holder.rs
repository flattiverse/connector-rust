use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, Index, IndexMut};

pub struct UniversalHolder<I, T> {
    data: Vec<Option<T>>,
    _i: PhantomData<I>,
}

impl<I, T> Debug for UniversalHolder<I, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!(
            "UniversalHolder<{}, {}>",
            type_name::<I>(),
            type_name::<T>()
        ))
        .finish_non_exhaustive()
    }
}

impl<I, T> UniversalHolder<I, T> {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            data: (0..size).map(|_| None).collect(),
            _i: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flatten()
    }
}

impl<I: Indexer, T> UniversalHolder<I, T> {
    #[inline]
    pub fn remove(&mut self, index: I) -> Option<T> {
        core::mem::take(&mut self.data[index.index()])
    }

    #[inline]
    pub fn set(&mut self, index: I, value: impl Into<Option<T>>) {
        self.data[index.index()] = value.into();
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&T> {
        self.data[index.index()].as_ref()
    }

    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.data[index.index()].as_mut()
    }
}

impl<T: NamedUnit> UniversalHolder<(), T> {
    pub fn push(&mut self, value: impl Into<T>) {
        *self.data.iter_mut().find(|slot| slot.is_none()).unwrap() = Some(value.into());
    }
}

impl<I, T: NamedUnit> UniversalHolder<I, T> {
    pub fn remove_by_name(&mut self, name: &str) -> Option<T> {
        self.data.iter_mut().find_map(|slot| match slot {
            Some(value) if &*value.name() == name => slot.take(),
            _ => None,
        })
    }

    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        self.data
            .iter()
            .flat_map(|d| d.as_ref())
            .find(|d| &*d.name() == name)
    }

    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut T> {
        self.data
            .iter_mut()
            .flat_map(|d| d.as_mut())
            .find(|d| &*d.name() == name)
    }
}

impl<I: Indexer + Debug + Copy, T> Index<I> for UniversalHolder<I, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        match self.get(index) {
            Some(value) => value,
            None => panic!("There is no entry for the given Index={index:?}"),
        }
    }
}

impl<I: Indexer + Debug + Copy, T> IndexMut<I> for UniversalHolder<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        match self.get_mut(index) {
            Some(value) => value,
            None => panic!("There is no entry for the given Index={index:?}"),
        }
    }
}

impl<'a, I, T: NamedUnit> Index<&'a str> for UniversalHolder<I, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &'a str) -> &Self::Output {
        match self.get_by_name(index) {
            Some(value) => value,
            None => panic!("There is no entry for the given Index={index:?}"),
        }
    }
}

pub trait Indexer {
    fn index(&self) -> usize;
}

pub trait NamedUnit {
    fn name(&self) -> impl Deref<Target = str> + '_;
}

pub trait Identifiable<T: Indexer> {
    fn id(&self) -> T;
}
