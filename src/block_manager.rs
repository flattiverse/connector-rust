use std::ops::Sub;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::time::Instant;

use crate::Error;
use crate::net::Packet;
use crate::net::BinaryReader;

pub const BLOCK_COUNT      : usize    = 255;
pub const BLOCK_OFFSET     : usize    =   1;
pub const WAIT_TIME_MILLIS : Duration = Duration::from_secs(3);


pub struct BlockManager {
    blocks: Vec<Arc<BlockInner>>
}

impl BlockManager {
    pub fn new() -> BlockManager {
        BlockManager {
            blocks: {
                let mut vec = Vec::with_capacity(BLOCK_COUNT);
                for id in 0..BLOCK_COUNT as u8 {
                    vec.push(Arc::new({
                        let mut inner = BlockInner::default();
                        inner.id = id + BLOCK_OFFSET as u8;
                        inner
                    }));
                }
                vec
            }
        }
    }

    pub fn block(&self) -> Result<Block, Error> {
        let inner = self.blocks
            .iter()
            .find_map(|inner|
                if inner.try_activate() {
                    Some(inner.clone())
                } else {
                    None
                }
            )
            .ok_or(Error::NoFreeSlots)?;
        Ok(Block {
            inner,
            timestamp: Instant::now(),
        })
    }

    pub fn answer(&self, response: Packet) {
        let id = response.session() as usize;
        self.blocks[id - BLOCK_OFFSET].answer(response);
    }
}


pub struct Block {
    inner: Arc<BlockInner>,
    timestamp: Instant,
}

impl Block {

    pub fn id(&self) -> u8 {
        self.inner.id
    }

    pub fn wait(&mut self) -> Result<Packet, Error> {
        let packet : Packet = match self.inner.r#await(self.timestamp, WAIT_TIME_MILLIS) {
            Some(packet) => packet,
            None => return Err(Error::Timeout)
        };

        if packet.command() == 0xFF { // error
            if packet.path_sub() == 0xFF { // server error
                let reader : &mut BinaryReader = &mut packet.read();
                return Err(Error::ServerError {
                    exception_type: reader.read_string()?,
                    message: reader.read_string()?,
                    stack_trace: reader.read_string()?
                });

            } else {
                return Err(Error::error_code(packet.path_sub()))
            }
        }

        Ok(packet)
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        self.inner.deactivate();
    }
}


#[derive(Default)]
struct BlockInner {
    id: u8,
    active: AtomicBool,
    data: Mutex<Option<Packet>>,
    cond: Condvar,
}

impl BlockInner {
    /// Tries to activate this block and returns whether the operation was successful
    fn try_activate(&self) -> bool {
        !self.active.swap(true, Ordering::Relaxed)
    }

    fn deactivate(&self) {
        {
            let mut lock = self.data.lock().expect("Failed to acquire lock for BlockInner::data");
            *lock = None;
        }
        self.active.store(false, Ordering::Relaxed);
    }

    fn answer(&self, packet: Packet) {
        {
            let mut lock = self.data.lock().expect("Failed to acquire lock for BlockInner::data");
            *lock = Some(packet);
        }
        self.cond.notify_all();
    }

    fn r#await(&self, time: Instant, timeout: Duration) -> Option<Packet> {
        let mut lock = self.data.lock().expect("Failed to acquire lock for BlockInner::data");
        let mut elapsed = time.elapsed();
        while elapsed < timeout {
            let (returned_lock, _timeout_result) = self.cond.wait_timeout(lock, timeout.sub(elapsed)).expect("wait_timeout failed");
            lock = returned_lock;
            if let Some(response) = lock.take() {
                return Some(response);
            }
            elapsed = time.elapsed();
        }
        None
    }
}