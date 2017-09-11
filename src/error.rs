
use std;
use std::sync::PoisonError;
use std::sync::mpsc::SendError;

use backtrace::Backtrace;

#[derive(Debug)]
pub enum Error {
    DowncastError(Backtrace),
    IoError(Backtrace, std::io::Error),
    SendError,
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
    TournamentNotAvailable,
    ScoresNotAvailable,
    PlayerNotAvailable,
    PlayerNotInUniverseGroup,
    VectorNotAvailable,
    TeamNotAvailable,
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
    InvalidUniverseGroup(u16),
    InvalidTeam(u8),
    PendingShutdown,
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IoError(Backtrace::new(), ioe)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::PoisonError
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::SendError
    }
}

use downcast::DowncastError;
impl<T> From<DowncastError<T>> for Error {
    fn from(e: DowncastError<T>) -> Self {
        Error::DowncastError(Backtrace::new())
    }
}