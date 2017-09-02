

mod crypt;
mod packet;
mod binary_reader;
mod binary_writer;
mod connection;

pub use self::crypt::*;
pub use self::packet::*;
pub use self::binary_reader::*;
pub use self::binary_writer::*;
pub use self::connection::*;


pub fn is_set_u8(val: u8, mask: u8) -> bool {
    (val & mask) == mask
}