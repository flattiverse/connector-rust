
use std;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EmailAndOrPasswordInvalid,
    RequestedPacketSizeIsInvalid{max: u32, was: u32},
    NoFreeSlots,
    FailedToFetchBlock,
    Timeout(::std::sync::mpsc::RecvTimeoutError),
    ErrorCode(u8),
    ServerError {
        exception_type: String,
        message: String,
        stack_trace: String
    },
    UnknownMessageType,
    InvalidHash,
    InvalidHostState,
    YouBrokeSomethingBro,
    InvalidChatMessage,
    CannotSendMessageIntoAnotherUniverseGroup,
    InvalidControllableInfo(u8),
    InvalidCargoItem(u8),
    InvalidCrystalKind(u8),
    CannotRenameCrystalKind(super::item::CrystalKind),
    YouCanOnlyRenameCrystalsNotInUse(String),
    YouAreNotTheCrystalMaster(String),
    UnknownUnitType(u8)
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IoError(ioe)
    }
}