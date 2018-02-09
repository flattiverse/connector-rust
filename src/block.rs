

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::time::Instant;
use std::time::Duration;

use std::ops::Sub;

use Error;
use net::Packet;
use net::BinaryReader;

use block_manager::BLOCK_OFFSET;

const WAIT_TIME_MILLIS : u64 = 3 * 1000;

pub struct Block {
    pub(crate) blocks: Arc<Mutex<Vec<Option<Sender<Box<Packet>>>>>>,
    pub(crate) id: u8,
    pub(crate) receiver: Receiver<Box<Packet>>,
    pub(crate) timestamp: Instant
}

impl Block {

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn wait(&mut self) -> Result<Box<Packet>, Error> {
        let time_passed = Instant::now().duration_since(self.timestamp);
        let max_time_wait = Duration::from_millis(WAIT_TIME_MILLIS);

        let time_wait = if time_passed > max_time_wait {
            Duration::from_millis(0)
        } else {
            max_time_wait.sub(time_passed)
        };

        let packet : Box<Packet> = match self.receiver.recv_timeout(time_wait) {
            Ok(packet) => packet,
            Err(e) => return Err(Error::Timeout(e))
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

    fn close(&mut self) {
        let mut lock = self.blocks.lock().expect("Failed to acquire lock");
        lock[self.id as usize-BLOCK_OFFSET] = None;
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        self.close()
    }
}