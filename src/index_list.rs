
use std::sync::Arc;
use std::sync::Weak;


pub struct IndexList<T> {
    index: usize,
    values: Vec<Option<Arc<T>>>,
    value: Option<Arc<T>>,
    avoid_zero: bool,
}

impl<T> IndexList<T> {
    pub fn new(avoid_zero: bool, size: usize) -> IndexList<T> {
        let mut list = IndexList {
            index: if avoid_zero {1} else {0},
            values: Vec::with_capacity(size),
            value: None,
            avoid_zero: avoid_zero
        };

        // fill the whole Vec
        for _ in 0..size {
            list.values.push(None);
        }

        list
    }

    pub fn get(&self, index: usize) -> Option<Arc<T>> {
        match self.values.get(index) {
            None => None,
            Some(ref option) => match option {
                &&Some(ref arc) => Some(arc.clone()),
                &&None => None
            }
        }
    }

    pub fn get_weak(&self, index: usize) -> Weak<T> {
        match self.values.get(index) {
            None => Weak::new(),
            Some(ref option) => match option {
                &&Some(ref arc) => Arc::downgrade(arc),
                &&None => Weak::new()
            }
        }
    }

    pub fn set(&mut self, index: usize, value: Option<Arc<T>>) {
        self.values[index] = value;
    }

    pub fn wipe(&mut self, index: usize) {
        self.values[index] = None;
    }

    pub fn wipe_all(&mut self) {
        for i in 0..self.values.len() {
            self.values[i] = None
        }
    }

    pub fn count(&mut self) -> usize {
        let mut counter = 0;
        for i in 0..self.values.len() {
            if self.values[i].is_some() {
                counter += 1;
            }
        }
        counter
    }

    pub fn len(&self) -> usize {
        self.values.capacity()
    }
}