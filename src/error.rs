
use std;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EmailAndOrPasswordInvalid,
    RequestedPacketSizeIsInvalid{max: u32, was: u32}
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IoError(ioe)
    }
}