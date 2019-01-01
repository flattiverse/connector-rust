
use std::sync::Weak;

use crate::Error;
use crate::DateTime;
use crate::Connector;
use crate::ManualResetEvent;

use atomic::Atomic;
use atomic::Ordering;

use std::thread;
use std::thread::ThreadId;


pub struct UniverseGroupFlowControl {
    connector: Weak<Connector>,
    pre_wait:  ManualResetEvent,
    wait_ev:   ManualResetEvent,

    limit_stamp: Atomic<DateTime>,

    tick:   Atomic<u16>,
    ready:  Atomic<bool>,
    thread: ThreadId,
}

impl PartialEq for UniverseGroupFlowControl {
    fn eq(&self, other: &UniverseGroupFlowControl) -> bool {
        (self as *const UniverseGroupFlowControl) == (other as *const UniverseGroupFlowControl)
    }
}

impl UniverseGroupFlowControl {
    pub fn new(connector: Weak<Connector>) -> UniverseGroupFlowControl {
        UniverseGroupFlowControl {
            connector,
            thread:     thread::current().id(),
            wait_ev:    ManualResetEvent::new(false),
            pre_wait:   ManualResetEvent::new(false),
            ready:      Atomic::new(true),

            // defaults
            tick:           Atomic::new(0u16),
            limit_stamp:    Atomic::new(DateTime::from_ticks(0_i64)),
        }
    }

    pub fn setup(&self) -> Result<(), Error> {
        self.pre_wait.wait_one()?;
        Ok(())
    }

    pub fn set_pre_wait(&self, limit_stamp: DateTime, tick: u16) -> Result<(), Error> {
        self.limit_stamp.store(limit_stamp, Ordering::Relaxed);


        if self.ready.load(Ordering::Relaxed) {
            self.tick.store(tick, Ordering::Relaxed);
            self.ready.store(false, Ordering::Relaxed);
        }

        self.pre_wait.set()?;
        Ok(())
    }

    pub fn set_wait(&self, limit_stamp: DateTime) -> Result<(), Error> {
        self.limit_stamp.store(limit_stamp, Ordering::Relaxed);
        self.wait_ev.set()?;
        Ok(())
    }

    /// Returns when the pre-wait-phase has ended
    pub fn pre_wait(&self) -> Result<i64, Error> {
        Self::check_thread(self.thread)?;
        self.pre_wait.wait_one()?;
        Ok(self.limit_stamp.load(Ordering::Relaxed).elapsed_millis())
    }

    pub fn wait(&self) -> Result<i64, Error> {
        Self::check_thread(self.thread)?;
        self.wait_ev.wait_one()?;
        Ok(self.limit_stamp.load(Ordering::Relaxed).elapsed_millis())
    }

    pub fn ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    /// Commits this instance. The return
    /// value indicates whether the commit
    /// was in time
    pub fn commit(&self) -> Result<bool, Error> {
        Self::check_thread(self.thread)?;
        self.wait_ev .reset()?;
        self.pre_wait.reset()?;
        self.ready.store(true, Ordering::Relaxed);
        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let tick = self.tick.load(Ordering::Relaxed);
                connector.flow_control_check(tick)
            }
        }
    }

    fn check_thread(allowed: ThreadId) -> Result<(), Error> {
        if thread::current().id() != allowed {
            return Err(Error::AccessFromWrongThreadAllowedOnly(allowed));
        }
        Ok(())
    }
}