
use std::sync::Arc;
use std::sync::Weak;

use IndexList;

pub trait UniversalEnumerable {
    fn name(&self) -> &str;
}

pub struct UniversalHolder<T: UniversalEnumerable>  {
    list: IndexList<T>
}

impl<T: UniversalEnumerable> UniversalHolder<T> {
    pub fn new(list: IndexList<T>) -> UniversalHolder<T> {
        UniversalHolder {
            list
        }
    }

    pub(crate) fn set(&mut self, index: usize, value: Option<Arc<T>>) {
        self.list.set(index, value);
    }

    pub fn list(&self) -> Vec<Arc<T>> {
        let mut list = Vec::new();

        for i in 0..self.list.len() {
            match self.list.get(i) {
                Some(arc) => list.push(arc.clone()),
                _ => {}
            }
        }

        list
    }

    pub fn get_for_index(&self, index: usize) -> Option<Arc<T>> {
        self.list.get(index)
    }

    pub fn get_for_index_weak(&self, index: usize) -> Weak<T> {
        self.list.get_weak(index)
    }

    pub fn get_for_name(&self, name: &str) -> Option<Arc<T>> {
        for i in 0..self.list.len() {
            match self.list.get(i) {
                None => {},
                Some(ref arc) => {
                    if arc.name().eq(name) {
                        return Some(arc.clone())
                    }
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl<T: UniversalEnumerable> UniversalEnumerable for Box<T> {
    fn name(&self) -> &str {
        self.as_ref().name()
    }
}