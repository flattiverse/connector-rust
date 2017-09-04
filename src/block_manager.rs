
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::time::Instant;

use Error;
use Block;
use net::Packet;

pub const BLOCK_COUNT  : usize = 255;
pub const BLOCK_OFFSET : usize =   1;

pub struct BlockManager {
    blocks: Arc<Mutex<Vec<Option<Sender<Box<Packet>>>>>>
}

impl BlockManager {
    pub fn new() -> BlockManager {
        let mut vec = Vec::with_capacity(BLOCK_COUNT);
        for _ in 0..BLOCK_COUNT {
            vec.push(None);
        }

        BlockManager {
            blocks: Arc::new(Mutex::new(vec))
        }
    }

    pub fn block(&self) -> Result<Arc<Mutex<Box<Block>>>, Error> {
        let (tx, rx) = channel();
        let mut lock = self.blocks.lock().expect("Failed to acquire lock");
        let index = Self::find_next_free(&lock)?;
        lock[index] = Some(tx);
        Ok(Arc::new(Mutex::new(Box::new(Block {
            blocks: self.blocks.clone(),
            receiver: rx,
            id: (index + BLOCK_OFFSET) as u8,
            timestamp: Instant::now(),
        }))))
    }

    fn find_next_free(vec: &Vec<Option<Sender<Box<Packet>>>>) -> Result<usize, Error> {
        for i in 0..BLOCK_COUNT {
            if vec[i].is_none() {
                return Ok(i);
            }
        }
        Err(Error::NoFreeSlots)
    }

    pub fn answer(&self, response: Box<Packet>) {
        let mut lock = self.blocks.lock().expect("Failed to acquire lock");
        let id = response.session() as usize;
        match lock[id-BLOCK_OFFSET] {
            Some(ref mut sender) => {
                println!("sending to {:?}", sender);
                let _ = sender.send(response);
            },
            _ => {
                println!("no sender found");
            }
        };
    }
}