
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

    pub fn set(&self) -> Result<(), Error> {
        println!("set being called");
        *self.mutex.lock()? = true;
        self.condition.notify_one();
        println!("set being called: done");
        Ok(())
    }

    pub fn reset(&self) -> Result<(), Error> {
        println!("reset being called");
        *self.mutex.lock()? = false;
        self.condition.notify_one();
        println!("reset being called: done");
        Ok(())
    }

    pub fn wait_one(&self) -> Result<(), Error> {
        println!("wait_one");
        let mut lock = self.mutex.lock()?;
        println!("locked");
        while !*lock {
            println!("condvar_wait");
            lock = self.condition.wait(lock)?;
            println!("condvar_waited");
        }
        println!("wait_one done");
        Ok(())
    }

    pub fn close(mut self) -> Result<(), Error> {
        self.reset()
    }
}