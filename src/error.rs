
use std;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EmailAndOrPasswordInvalid,
    RequestedPacketSizeIsInvalid{max: u32, was: u32},
    NoFreeSlots,
    FailedToFetchBlock,
    Timeout,
    ErrorCode(u8),
    ServerError {
        exception_type: String,
        message: String,
        stack_trace: String
    }
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IoError(ioe)
    }
}