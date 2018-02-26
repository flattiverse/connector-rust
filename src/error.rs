
use std;
use std::sync::PoisonError;
use std::sync::mpsc::SendError;

use backtrace::Backtrace;

#[derive(Debug)]
pub enum Error {
    IoError(Backtrace, std::io::Error),
    SendError,
    EmailAndOrPasswordInvalid,
    RequestedPacketSizeIsInvalid{max: u32, was: u32},
    NoFreeSlots,
    FailedToFetchBlock,
    Timeout(::std::sync::mpsc::RecvTimeoutError),
    ErrorCode(u8, Option<&'static str>),
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
    MissingPlayer(Backtrace, u16),
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
    InvalidUniverse(u8),
    InvalidUniverseGroup(u16),
    InvalidTeam(u8),
    PendingShutdown,
    PlayerAlreadyInAnotherUniverseGroup(u16),
    StillOpenFlowControlsInUniverseGroup(u16),
    WithReason(String),
    NotCrystalCargoItem(Backtrace),
    NotControllableShip(Backtrace),
    UniverseNotInUniverseGroup,
}

impl Error {
    pub fn missing_player(player: u16) -> Error {
        Error::MissingPlayer(Backtrace::new(), player)
    }

    pub fn not_crystal_cargo_item() -> Error {
        Error::NotCrystalCargoItem(Backtrace::new())
    }

    pub fn not_controllable_ship() -> Error {
        Error::NotControllableShip(Backtrace::new())
    }

    pub fn error_code(code: u8) -> Error {
        Error::ErrorCode(code, Error::error_code_msg(code))
    }

    fn error_code_msg(code: u8) -> Option<&'static str> {
        match code {
            0x00 => Some("Can't connect: check if tcp://galaxy.flattiverse.xxx:22 is available."),
            0x01 => Some("Invalid protocol version: Please update your connector."),
            0x02 => Some("Access denied: Wrong username, password or insufficient privileges."),
            0x03 => Some("User already online: Please wait while your other connection is being logged off, all your lingering ships are being removed and all your remaining actions are being processed."),
            0x04 => Some("Too many players: Try again later."),
            0x05 => Some("Too many players from your address: Close some sessions."),
            0x06 => Some("You have been banned from the flattiverse gameserver."),
            0x10 => Some("Network timeout: Ping too slow? Disconnected?"),
            0x11 => Some("Too many sessions active in the protocol."),
            0x12 => Some("This request cannot be parallel executed."),
            0x20 => Some("Player not online or not available."),
            0x21 => Some("UniverseGroup not online or not available."),
            0x22 => Some("Team not available in this UniverseGroup."),
            0x23 => Some("Please part first."),
            0x24 => Some("Please join a UniverseGroup first."),
            0x25 => Some("Tick is gone."),
            0x26 => Some("Please dispose all FlowControls, before parting the UniverseGroup."),
            0x27 => Some("Your system doesn't meet the performance requirements in order to enter this UniverseGroup."),
            0x28 => Some("Invalid password."),
            0x29 => Some("Your scores don't meet the access requirements in order to enter this UniverseGroup."),
            0x2A => Some("No performance-data available."),
            0x2B => Some("Tournament (already) configured."),
            0x2C => Some("Can't switch TournamentStage."),
            0x2D => Some("This game-type doesn't support tournaments."),
            0x2E => Some("You can't register a ship while current TournamentStage is active. Maybe you are no member of this tournament or your team doesn't match the tournament-setup."),
            0x2F => Some("The level of t he controllable exceeds the maximum allowed level for this universe group."),
            0x30 => Some("Index not found."),
            0x31 => Some("Please don't flood."),
            0x32 => Some("No ack for binary messages."),
            0x40 => Some("You can only destroy crystals not in use."),
            0x41 => Some("This crystal doesn't exist or is in an invalid state."),
            0x42 => Some("Not enough crystal slots."),
            0x43 => Some("You can only rename crystals not in use."),
            0x44 => Some("You can't rename special crystals."),
            0x60 => Some("Movement-vector exceeds extended acceleration-limit."),
            0x61 => Some("Scan-range exceeds extended scanner-range-limit."),
            0x62 => Some("Weapon-load exceeds extended limit."),
            0x63 => Some("Weapon-speed exceeds extended limit."),
            0x64 => Some("Detonation-time exceeds extended limit."),
            0x65 => Some("Hull-damage exceeds extended limit."),
            0x66 => Some("Shield-damage exceeds extended limit."),
            0x67 => Some("Energy-damage exceeds extended limit."),
            0x68 => Some("There is currently no shot available."),
            0x69 => Some("Energy-transfer exceeds extended limit."),
            0x6A => Some("Particle-transfer exceeds extended limit."),
            0x6B => Some("Ion-transfer exceeds extended limit."),
            0x6C => Some("Scan-span exceeds maximum scannable degree per scan."),
            0x6D => Some("This ship can't build that kind of unit."),
            0x6E => Some("Platforms can't move."),
            0x6F => Some("Crystal converter exceeds maximum conversation request."),
            0x70 => Some("Tractor beam request exceeds maximum force."),
            0x80 => Some("Too many controllables of that kind for you."),
            0x81 => Some("Too many controllables of that kind for your team."),
            0x82 => Some("This unit is already shutting down."),
            0x83 => Some("Please close all units before using UniverseGroup::part()."),
            0x84 => Some("Please use Ship::proceed() before that."),
            0x85 => Some("Please die before using that."),
            0x86 => Some("All start-locations are currently overcrowded."),
            0x87 => Some("UniverseGroup reset is pending."),
            0x88 => Some("This Unit is a \"built-unit\" (Platform, Drone, Probe, Base) and can't continue or is a unit which is currently in build progress."),
            0x90 => Some("At least one argument contains NaN or +-Inf."),
            0x91 => Some("At least one argument is wrong."),
            0x92 => Some("Not enough energy for this action."),
            0x93 => Some("Not enough particles for this action."),
            0x94 => Some("Not enough ions for this action."),
            0x95 => Some("The request will result in a packet too big to transfer. Limit or reduce sent data in the call. (Short descriptions or binary data.)"),
            0x98 => Some("No matching ship class found."),
            0x99 => Some("Name is already in use."),
            0x9A => Some("Name doesn't match specification."),
            0x9B => Some("Too many subrequests in request."),
            0x9C => Some("Too many subdirections in request."),
            0x9D => Some("You don't have enough resources (energy, ions, particles) to build this Unit."),
            0x9E => Some("You can only harvest 0.25 nebula per tick."),
            0x9F => Some("Requirements not met."),
            0xA0 => Some("Can't find free name."),
            0xA1 => Some("This unit is moving too fast for repair."),
            0xA2 => Some("Too many crystals. A maximum of 64 is allowed."),
            0xB0 => Some("There is no such Unit in range."),
            0xB1 => Some("The dedicated unit isn't a player-unit."),
            0xE0 => Some("You can only use this object from the thread you created it from."),
            0xF0 => Some("Something on server-side is not well configured. Please report this to the admins."),
            _ => None
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::WithReason(message)
    }
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
