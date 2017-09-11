
use Error;

pub struct ManagedArray<T: Clone> {
    index:    isize,
    array:    Vec<Option<T>>,
}

impl<T: Clone> ManagedArray<T> {
    pub fn with_capacity(capacity: usize) -> ManagedArray<T> {
        ManagedArray {
            index: -1,
            array: vec!(None; capacity),
        }
    }

    pub fn get(&self, index: usize) -> &Option<T> {
        &self.array[index]
    }

    pub fn set(&mut self, index: usize, value: Option<T>) {
        self.array[index] = value;
    }

    pub fn wipe_all(&mut self) {
        for i in 0..self.array.len() {
            self.array[i] = None;
        }
    }

    pub fn wipe_index(&mut self, index: usize) {
        self.set(index, None);
    }

    /*
    pub fn wipe_value(&mut self, value: &T) {
        for i in 0..self.array.len() {
            if let Some(ref v) = self.array[i] {
                if value.eq(v) {
                    self.array[i] = None;
                    break;
                }
            }
        }
    }*/

    pub fn insert(&mut self, value: T) -> Result<usize, Error> {
        for i in 0..self.array.len() {
            self.index = (self.index+1) % self.array.len() as isize;
            if self.array[self.index as usize].is_none() {
                self.array[self.index as usize] = Some(value);
                return Ok(self.index as usize);
            }
        }
        Err(Error::NoFreeSlots)
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for val in self.array.iter() {
            if val.is_some() {
                count += 1;
            }
        }
        count
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }
}

impl<T: Clone> AsRef<[Option<T>]> for ManagedArray<T> {
    fn as_ref(&self) -> &[Option<T>] {
        self.array.as_ref()
    }
}

impl<T: Clone> AsMut<[Option<T>]> for ManagedArray<T> {
    fn as_mut(&mut self) -> &mut [Option<T>] {
        self.array.as_mut()
    }
}

