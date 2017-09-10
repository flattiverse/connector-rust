
use std;
use std::sync::PoisonError;

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
    UnknownUnitType(u8),
    PoisonError,
    ConnectorNotAvailable,
    ControllableNotAvailable,
    PlayerNotAvailable,
    PlayerNotInUniverseGroup,
    ControllableInfoNotAvailable,
    InvalidMessage,
    InvalidMessageList,
    InvalidMessageAtIndex(u8),
    CantSendMessageToInactivePlayer,
    MissingPlayer(u16),
    InvalidFromDegree(f32),
    InvalidToDegree(f32),
    InvalidRange(f32),
    InvalidEvent(u8),
    InvalidDifficulty(u8),
    InvalidPerformanceRequirement(u8),
    InvalidTournamentStage(u8),
    InvalidTournamentSet(u8),
    InvalidControllable(u8),
    InvalidName,
    InvalidClass,
    InvalidDirection,
    InvalidValue(f32),
    AccessFromWrongThreadAllowedOnly(std::thread::ThreadId),
    TickIsGone,
    VectorIsDamaged,
    ScanRequestExceedsScannerCount{got: u8, max: u8},
    TooManySubDirections(usize),
    InvalidDestination,
    InvalidEnergyValue(f32),
    InvalidParticlesValue(f32),
    InvalidIonsValue(f32),
    InvalidDirectionValue(f32),
    InvalidRangeValue(f32),
    InvalidForceValue(f32),
    InvalidCrystalName(String),
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IoError(ioe)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::PoisonError
    }
}