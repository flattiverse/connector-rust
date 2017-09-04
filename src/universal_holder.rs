
use std::sync::Arc;
use std::sync::RwLock;

use IndexList;

pub trait UniversalEnumerable {
    fn name(&self) -> &str;
}

pub struct UniversalHolder<T: UniversalEnumerable>  {
    list: IndexList<RwLock<T>>
}

impl<T: UniversalEnumerable> UniversalHolder<T> {
    pub fn new(list: IndexList<RwLock<T>>) -> UniversalHolder<T> {
        UniversalHolder {
            list: list
        }
    }

    pub fn list(&self) -> Vec<Arc<RwLock<T>>> {
        let mut list = Vec::new();

        for i in 0..self.list.len() {
            match self.list.get(i) {
                Some(arc) => list.push(arc),
                _ => {}
            }
        }

        list
    }

    pub fn get_for_index(&self, index: u8) -> Option<Arc<RwLock<T>>> {
        self.list.get(index as usize)
    }

    pub fn get_for_name(&self, name: &str) -> Option<Arc<RwLock<T>>> {
        for i in 0..self.list.len() {
            match self.list.get(i) {
                None => {},
                Some(arc) => {
                    if arc.read().unwrap().name().eq(name) {
                        return Some(arc)
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