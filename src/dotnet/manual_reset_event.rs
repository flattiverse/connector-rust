
use std::sync::Mutex;
use std::sync::Condvar;

use Error;


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

    pub fn set(&mut self) -> Result<(), Error> {
        *self.mutex.lock()? = true;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        *self.mutex.lock()? = false;
        Ok(())
    }

    pub fn wait_one(&self) -> Result<(), Error> {
        let mut lock = self.mutex.lock()?;
        while !*lock {
            lock = self.condition.wait(lock)?;
        }
        Ok(())
    }

    pub fn close(mut self) -> Result<(), Error> {
        self.reset()
    }
}