use crate::{Identifiable, Indexer, NamedUnit};
use arc_swap::ArcSwapOption;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct UniversalArcHolder<I, T> {
    data: Vec<ArcSwapOption<T>>,
    _i: PhantomData<I>,
}

impl<I, T> Debug for UniversalArcHolder<I, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!(
            "UniversalHolderArc<{}, {}>",
            type_name::<I>(),
            type_name::<T>()
        ))
        .finish_non_exhaustive()
    }
}

impl<I, T> UniversalArcHolder<I, T> {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            data: (0..size).map(|_| ArcSwapOption::from(None)).collect(),
            _i: PhantomData::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Arc<T>> + '_ {
        self.data.iter().flat_map(|arc| arc.load_full())
    }
}

impl<I: Indexer, T> UniversalArcHolder<I, T> {
    #[inline]
    pub fn remove(&self, index: I) -> Arc<T>
    where
        I: Debug + Copy,
    {
        self.remove_opt(index)
            .unwrap_or_else(|| unreachable!("There is no entry for the given Index={index:?}"))
    }

    #[inline]
    pub fn remove_opt(&self, index: I) -> Option<Arc<T>> {
        self.data[index.index()].swap(None)
    }

    #[inline]
    pub fn populate(&self, value: impl Into<T>) -> Arc<T>
    where
        T: Identifiable<I>,
    {
        let value = Arc::new(value.into());
        let index = value.id();
        self.set(index, Some(Arc::clone(&value)));
        value
    }

    #[inline]
    pub fn set(&self, index: I, value: impl Into<Option<Arc<T>>>) {
        self.data[index.index()].store(value.into());
    }

    #[inline]
    pub fn get(&self, index: I) -> Arc<T>
    where
        I: Debug + Copy,
    {
        self.data
            .get(index.index())
            .map(|v| v.load_full())
            .flatten()
            .unwrap_or_else(|| unreachable!("There is no entry for the given Index={index:?}"))
    }

    #[inline]
    pub fn get_opt(&self, index: I) -> Option<Arc<T>> {
        self.data[index.index()].load_full()
    }
}

impl<T: NamedUnit> UniversalArcHolder<(), T> {
    pub fn push(&self, value: impl Into<Arc<T>>) {
        let value = value.into();
        if self
            .data
            .iter()
            .find(|slot| {
                slot.compare_and_swap(&None::<Arc<T>>, Some(Arc::clone(&value)))
                    .is_none()
            })
            .is_none()
        {
            // TODO grow??
            unreachable!()
        }
    }
}

impl<I, T: NamedUnit> UniversalArcHolder<I, T> {
    #[inline]
    pub fn remove_by_name(&self, name: &str) -> Arc<T> {
        self.remove_by_name_opt(name)
            .unwrap_or_else(|| unreachable!("There is no entry for the given name={name:?}"))
    }

    pub fn remove_by_name_opt(&self, name: &str) -> Option<Arc<T>> {
        self.data.iter().find_map(|slot| {
            let guard = slot.load();
            match &*guard {
                Some(value) if value.name() == name => {
                    return slot
                        .compare_and_swap(&*guard, None)
                        .as_ref()
                        .filter(|v| v.name() == name)
                        .map(Arc::clone);
                }
                _ => None,
            }
        })
    }

    #[inline]
    pub fn get_by_name(&self, name: &str) -> Arc<T> {
        self.get_by_name_opt(name)
            .unwrap_or_else(|| unreachable!("There is no entry for the given name={name:?}"))
    }

    pub fn get_by_name_opt(&self, name: &str) -> Option<Arc<T>> {
        self.data.iter().find_map(|slot| {
            let guard = slot.load();
            match &*guard {
                Some(value) if value.name() == name => Some(value.to_owned()),
                _ => None,
            }
        })
    }
}

impl<I: Indexer, T> UniversalArcHolder<I, T> {
    #[inline]
    pub fn has_not(&self, index: I) -> bool {
        !self.has(index)
    }

    pub fn has(&self, index: I) -> bool {
        self.data
            .get(index.index())
            .filter(|s| s.load().is_some())
            .is_some()
    }
}
