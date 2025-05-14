use crate::galaxy_hierarchy::{Identifiable, Indexer};
use arc_swap::ArcSwapOption;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct UniversalArcHolder<I, T> {
    data: Vec<ArcSwapOption<T>>,
    elements: AtomicU32,
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
            elements: AtomicU32::default(),
            _i: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Arc<T>> + '_ {
        self.data
            .iter()
            .flat_map(|arc| arc.load_full())
            .take(self.elements.load(Ordering::Relaxed) as usize)
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

    pub fn remove_opt(&self, index: I) -> Option<Arc<T>> {
        let result = self.data[index.index()].swap(None);
        if result.is_some() {
            self.elements.fetch_sub(1, Ordering::Relaxed);
        }
        result
    }

    #[inline]
    pub fn populate(&self, value: impl Into<T>) -> Arc<T>
    where
        T: Identifiable<I>,
    {
        let value = Arc::new(value.into());
        let index = value.id();
        self.set(index, Some(Arc::clone(&value)));
        self.elements.fetch_add(1, Ordering::Relaxed);
        value
    }

    pub fn set(&self, index: I, value: impl Into<Option<Arc<T>>>) {
        if self.data[index.index()].swap(value.into()).is_none() {
            self.elements.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline]
    pub fn get(&self, index: I) -> Arc<T>
    where
        I: Debug + Copy,
    {
        self.data
            .get(index.index())
            .and_then(|v| v.load_full())
            .unwrap_or_else(|| unreachable!("There is no entry for the given Index={index:?}"))
    }

    #[inline]
    pub fn get_opt(&self, index: I) -> Option<Arc<T>> {
        self.data[index.index()].load_full()
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
