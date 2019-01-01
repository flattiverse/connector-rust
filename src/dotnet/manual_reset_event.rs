#![allow(clippy::mutex_atomic)]

use std::sync::Mutex;
use std::sync::Condvar;

use crate::Error;

pub struct ManualResetEvent {
    mutex: Mutex<bool>,
    condition: Condvar,
}

impl ManualResetEvent {
    pub fn new(initial_state: bool) -> ManualResetEvent {
        ManualResetEvent {
            mutex:      Mutex::new(initial_state),
            condition:  Condvar::new(),
        }
    }

    pub fn set(&self) -> Result<(), Error> {
        *self.mutex.lock()? = true;
        self.condition.notify_one();
        Ok(())
    }

    pub fn reset(&self) -> Result<(), Error> {
        *self.mutex.lock()? = false;
        self.condition.notify_one();
        Ok(())
    }

    pub fn wait_one(&self) -> Result<(), Error> {
        let mut lock = self.mutex.lock()?;
        while !*lock {
            lock = self.condition.wait(lock)?;
        }
        Ok(())
    }

    pub fn close(self) -> Result<(), Error> {
        self.reset()
    }
}