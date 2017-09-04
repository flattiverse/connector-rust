
use std::ops::Index;
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

        for i in 0..self.list.capacity() {
            match self.list[i] {
                Some(arc) => list.push(arc.clone()),
                _ => {}
            }
        }

        list
    }

    pub fn get(&self, name: &str) -> Option<Arc<RwLock<T>>> {
        for i in 0..self.list.capacity() {
            match list[i] {
                None => {},
                Some(arc) => {
                    if arc.read().unwrap().name().eq(&name) {
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

impl<T: UniversalEnumerable> Index<Idx=usize> for UniversalHolder<T> {
    type Output = Option<Arc<RwLock<T>>>;

    fn index(&self, index: Idx) -> &Self::Output {
        self.list[index]
    }
}