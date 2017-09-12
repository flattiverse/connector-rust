
use std::sync::Weak;
use std::sync::RwLock;

use Error;
use DateTime;
use Connector;
use ManualResetEvent;

use std::thread;
use std::thread::ThreadId;


pub struct UniverseGroupFlowControl {
    connector: Weak<Connector>,
    pre_wait:  ManualResetEvent,
    wait_ev:   ManualResetEvent,

    limit_stamp: RwLock<DateTime>,

    tick:   RwLock<u16>,
    ready:  RwLock<bool>,
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
            ready:      RwLock::new(true),

            // defaults
            tick:           RwLock::new(0u16),
            limit_stamp:    RwLock::new(DateTime::from_ticks(0_i64)),
        }
    }

    pub fn setup(&self) -> Result<(), Error> {
        self.pre_wait.wait_one()?;
        Ok(())
    }

    pub fn set_pre_wait(&self, limit_stamp: DateTime, tick: u16) -> Result<(), Error> {
        *self.limit_stamp.write()? = limit_stamp;


        if *self.ready.read()? {
            *self.tick .write()? = tick;
            *self.ready.write()? = false;
        }

        self.pre_wait.set()?;
        Ok(())
    }

    pub fn set_wait(&self, limit_stamp: DateTime) -> Result<(), Error> {
        *self.limit_stamp.write()? = limit_stamp;
        self.wait_ev.set()?;
        Ok(())
    }

    /// Returns when the pre-wait-phase has ended
    pub fn pre_wait(&self) -> Result<i64, Error> {
        Self::check_thread(self.thread)?;
        self.pre_wait.wait_one()?;
        Ok(self.limit_stamp.read()?.elapsed_millis())
    }

    pub fn wait(&self) -> Result<i64, Error> {
        Self::check_thread(self.thread)?;
        self.wait_ev.wait_one()?;
        Ok(self.limit_stamp.read()?.elapsed_millis())
    }

    pub fn ready(&self) -> Result<bool, Error> {
        Ok(*self.ready.read()?)
    }

    /// Commits this instance. The return
    /// value indicates whether the commit
    /// was in time
    pub fn commit(&self) -> Result<bool, Error> {
        Self::check_thread(self.thread)?;
        self.wait_ev .reset()?;
        self.pre_wait.reset()?;
        *self.ready.write()? = true;
        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let tick = *self.tick.read()?;
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