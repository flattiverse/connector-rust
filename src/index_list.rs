
use std::rc::Rc;

pub struct IndexList<T> {
    index: usize,
    values: Vec<Option<Rc<T>>>,
    value: Option<Rc<T>>,
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

    pub fn get(&self, index: usize) -> Option<Rc<T>> {
        match self.values.get(index) {
            None => None,
            Some(rc) => match rc {
                &None => None,
                &Some(ref v) => Some(v.clone())
            }
        }
    }

    pub fn set(&mut self, index: usize, value: Option<Rc<T>>) {
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

    pub fn insert(&mut self, val: Rc<T>) -> Result<usize, &'static str> {
        for _ in 0..self.values.len() {
            if self.avoid_zero && self.index == 0 {
                self.index += 1;
            }

            if self.values[self.index].is_none() {
                let ret = self.index;
                self.index += 1;
                return Ok(ret);
            }

            self.index = (self.index+1) % self.values.len();
        }
        Err("No free slot")
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
}