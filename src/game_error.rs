use crate::account::AccountStatus;
use crate::galaxy_hierarchy::{PlayerKind, SubsystemComponentKind};
use crate::network::{InvalidArgumentKind, Packet, PacketReader, Session};
use num_enum::{FromPrimitive, TryFromPrimitive, TryFromPrimitiveError};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub struct GameError {
    kind: Box<GameErrorKind>,
}

impl GameError {
    #[inline]
    pub fn kind(&self) -> &GameErrorKind {
        &self.kind
    }

    pub(crate) async fn check_response_ok(session: Session) -> Result<(), GameError> {
        GameError::check_ok(session.response().await?)
    }

    #[inline]
    pub(crate) fn check_ok(packet: Packet) -> Result<(), GameError> {
        Self::check(packet, |_| Ok(()))
    }

    #[inline]
    pub(crate) fn check<T>(
        mut packet: Packet,
        f: impl FnOnce(Packet) -> Result<T, GameError>,
    ) -> Result<T, GameError> {
        if packet.header().command() == 0xFF {
            debug!("GameError, Packet={packet:?}");
            packet.read(|reader| Err(GameError::from(GameErrorKind::from(reader))))
        } else {
            f(packet)
        }
    }
}

impl From<GameErrorKind> for GameError {
    fn from(kind: GameErrorKind) -> Self {
        Self {
            kind: Box::new(kind),
        }
    }
}

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for GameError
where
    <T as TryFromPrimitive>::Primitive: ToString,
{
    fn from(value: TryFromPrimitiveError<T>) -> Self {
        GameErrorKind::InvalidPrimitiveValue {
            value: value.number.to_string(),
            r#type: std::any::type_name::<T>(),
        }
        .into()
    }
}

impl Display for GameError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameErrorKind {
    Unknown(u8),
    CantConnect,
    InvalidProtocolVersion,
    AuthFailed,
    WrongAccountState(Option<AccountStatus>),
    TeamSelectionFailed,
    SelfDisclosureRequired,
    /// Persistent storage is currently unavailable for this login attempt.
    PersistenceUnavailable,
    ServerFullOfPlayerKind(Option<PlayerKind>),
    AccountAlreadyLoggedIn,
    SessionsExhausted,
    InvalidData {
        message: Option<String>,
    },
    ConnectionTerminated {
        reason: Option<Arc<str>>,
    },
    SpecifiedElementNotFound,
    CantCallThisConcurrent,
    InvalidArgument {
        reason: InvalidArgumentKind,
        parameter: String,
    },
    /// Thrown, if you try to call a command where you don't have access to.
    PermissionFailed,
    /// Thrown, if you didn't honor the flood control.
    FloodcontrolTriggered,
    /// Thrown, if you try to register too many units.
    UnitConstraintViolation,
    /// Thrown, if a specific XML node or attribute has an invalid value.
    InvalidXmlNodeValue {
        /// Validation reason that caused the error.
        reason: InvalidArgumentKind,
        /// XML node/attribute path, for example: Galaxy>Team.ColorR.
        node_path: String,
        /// Human-readable hin in English.
        hint: String,
    },
    /// The requested continue action targets a controllable that is already closing.
    ControllableIsClosing,
    /// The requested player currently has no avatar available.
    AvatarNotAvailable,
    /// Thrown, if the controllable you want to control is dead.
    YouNeedToContinueFirst,
    /// Thrown, if you try to do something which requires that your controllable is dead, like
    /// `continue()`.
    YouNeedToDieFirst,
    /// Thrown, if a call to `continue()` fails, because there is no space for you.
    AllStartLocationsAreOvercrowded,
    /// Thrown, if you try to shoo too often.
    CanOnlyShootOncePerTick,

    /// Thrown when a tournament-specific action is requested although no tournament is currently
    /// configured.
    TournamentNotConfigured,
    /// Thrown when an admin tries to configure a tournament although one already exists.
    TournamentAlreadyConfigured,
    /// Thrown when a tournament lifecycle action is requested in a stage that does not permit it.
    TournamentWrongStage,
    /// Thrown when galaxy, region, or editable-unit map editing is attempted while any tournament
    /// is configured.
    TournamentMapEditingLocked,
    /// Thrown when ship registration or respawn is attempted while the current tournament stage has
    /// closed registration.
    TournamentRegistrationClosed,
    /// Thrown when a login, registration, or respawn requires tournament participation but the
    /// current account is not part of the configured tournament lists.
    TournamentParticipantRequired,
    /// Thrown when a spectator login is attempted in a tournament stage that forbids spectating.
    TournamentSpectatingForbidden,
    /// Thrown when the chosen login or controllable team conflicts with the account's configured
    /// tournament team.
    TournamentTeamMismatch,
    /// Thrown when tournament configuration is attempted in a galaxy game mode that does not allow
    /// tournaments.
    TournamentModeNotAllowed,
    /// Thrown when a normal player login is denied by the galaxy player ACL.
    PlayerAccessRestricted,
    /// Thrown when an admin login is denied by the galaxy admin ACL.
    AdminAccessRestricted,
    /// The galaxy is currently rebuilding its static segment data.
    StaticMapRebuildInProgress,
    /// Static segment rebuilding is currently blocked by tournament state.
    StaticMapRebuildLocked,
    /// Thrown when one subsystem-metadata usage evaluation receives the same component kind more
    /// than once.
    DuplicateSubsystemComponentValue {
        /// The duplicated component kind.
        component_kind: SubsystemComponentKind,
    },

    // TODO local only
    InvalidPrimitiveValue {
        value: String,
        r#type: &'static str,
    },
}

impl Display for GameErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GameErrorKind::Unknown(code) => return write!(f, "[{:#02x}] Unknown error code.", code),
            GameErrorKind::CantConnect => "[0x01] Couldn't connect to the flattiverse galaxy.",
            GameErrorKind::InvalidProtocolVersion => "[0x02] Invalid protocol version. Consider up(- or down)grading the connector.",
            GameErrorKind::AuthFailed => "[0x03] Authentication failed: Missing, wrong or unused API key.",
            GameErrorKind::WrongAccountState(None) => "[0x04] Your account is in the wrong state - however, this connector version doesn't understand the state submitted.",
            GameErrorKind::WrongAccountState(Some(state)) => match state {
                AccountStatus::Unknown => "[0x04] Your account is in the wrong state - however, this connector version doesn't understand the state submitted.",
                AccountStatus::OptIn => "[0x04] You need to opt-in first to use the game.",
                AccountStatus::ReOptIn => "[0x04] You need to re-opt-in first to use the game.",
                AccountStatus::User => "[0x04] Well, the game server should have you let in. Please report this issue to info@flattiverse.com.",
                AccountStatus::Banned => "[0x04] Your account has been banned from using the game.",
                AccountStatus::Deleted => "[0x04] Your account is deleted."
            },
            GameErrorKind::TeamSelectionFailed => "[0x05] Invalid team specified or no team available for auto-selection.",
            GameErrorKind::SelfDisclosureRequired => "[0x06] Galaxy requires self-disclosure for this login.",
            GameErrorKind::PersistenceUnavailable => "[0x07] Persistent storage is currently unavailable. Please try again later.",
            GameErrorKind::ServerFullOfPlayerKind(None) => "[0x08] Server is full of unknown things.",
            GameErrorKind::AccountAlreadyLoggedIn => "[0x09] Account already has an active galaxy session.",
            GameErrorKind::ServerFullOfPlayerKind(Some(kind)) => match kind {
                PlayerKind::Admin => "[0x08] Server is full of admins. (Too many admins connected to the galaxy server.)",
                PlayerKind::Spectator => "[0x08] Server is full of spectators. (Too many spectators connected to the galaxy server.)",
                PlayerKind::Player => "[0x08] All player slots are taken. Please wait until players leave the galaxy.",
                PlayerKind::Unknown(id) => return write!(f, "[0x08] Server is full of things with code {:#02x}.", id)
            },
            GameErrorKind::SessionsExhausted => "[0x0C] Sessions exhausted: You cannot have more than 255 calls in progress.",
            GameErrorKind::InvalidData {
                message,
            } => match message {
                None => "[0x0D] Invalid data received, protocol mismatch: Terminating connection.",
                Some(message) => message.as_str(),
            },
            GameErrorKind::ConnectionTerminated { reason } => if let Some(reason) = reason {
                return write!(f, "[0x0E] Connection has been terminated with reason: {reason}.");
            } else {
                "[0x0F] Connection has been terminated for unknown reason."
            },
            GameErrorKind::SpecifiedElementNotFound => "[0x10] Specified element not found.",
            GameErrorKind::CantCallThisConcurrent => "[0x11] This method cannot be called concurrently.",
            GameErrorKind::PermissionFailed => "[0x13] Permission denied. Did you try to call a command where you don't have access to?",
            GameErrorKind::FloodcontrolTriggered => "[0x14] You probably type too fast: Don't flood the chat.",
            GameErrorKind::UnitConstraintViolation => "[0x15] You tried to register too much units of a specific kind.",
            GameErrorKind::InvalidXmlNodeValue { reason, node_path, hint } => return write!(f, "[0x16] XML node {node_path:?} is invalid ({reason:?}): {hint}."),
            GameErrorKind::InvalidArgument {
                reason,
                parameter
            } => return write!(f, "[0x12] Parameter {parameter:?} {}", match reason {
                InvalidArgumentKind::TooSmall => "is wrong due to an too small value.",
                InvalidArgumentKind::TooLarge => "is wrong due to an too large value.",
                InvalidArgumentKind::NameConstraint => "doesn't match the name constraint.",
                InvalidArgumentKind::ChatConstraint => "doesn't match the chat constraint.",
                InvalidArgumentKind::AmbiguousXmlData => "contains ambiguous XML data.",
                InvalidArgumentKind::EntityNotFound => "doesn't point to an existing entity.",
                InvalidArgumentKind::NameInUse => "references a name which is already in use.",
                InvalidArgumentKind::ContainedNaN => "contained a \"Not a Number\" value.",
                InvalidArgumentKind::ConstrainedInfinity => "contained a \"Infinity\" value.",
                InvalidArgumentKind::Unknown(..) => "is wrong due to an invalid value."
            }),
            GameErrorKind::ControllableIsClosing => "[0x17] Can't continue a controllable that is already closing.",
            GameErrorKind::AvatarNotAvailable => "[0x18] This player has no avatar.",
            GameErrorKind::YouNeedToContinueFirst => "[0x20] This controllable is dead. You need to Continue() first.",
            GameErrorKind::YouNeedToDieFirst => "[0x21] This controllable is alive. The controllable needs to die first.",
            GameErrorKind::AllStartLocationsAreOvercrowded => "[0x22] All start locations are currently overcrowded.",
            GameErrorKind::CanOnlyShootOncePerTick =>  "[0x30] Please, only shoot once a tick with the same unit.",
            GameErrorKind::TournamentNotConfigured => "[0x31] No tournament is configured.",
            GameErrorKind::TournamentAlreadyConfigured => "[0x32] A tournament is already configured.",
            GameErrorKind::TournamentWrongStage => "[0x33] This tournament action is not allowed in the current stage.",
            GameErrorKind::TournamentMapEditingLocked => "[0x34] Map editing is locked while a tournament exists.",
            GameErrorKind::TournamentRegistrationClosed => "[0x35] Ship registration is closed in the current tournament stage.",
            GameErrorKind::TournamentParticipantRequired => "[0x36] This account does not participate in the configured tournament.",
            GameErrorKind::TournamentSpectatingForbidden => "[0x37] Spectating is forbidden in the current tournament stage.",
            GameErrorKind::TournamentTeamMismatch => "[0x38] This account is assigned to a different tournament team.",
            GameErrorKind::TournamentModeNotAllowed => "[0x39] Tournaments are not allowed for the current galaxy game mode.",
            GameErrorKind::PlayerAccessRestricted =>  "[0x3A] Player access to this galaxy is restricted by ACL.",
            GameErrorKind::AdminAccessRestricted =>  "[0x3B] Admin access to this galaxy is restricted by ACL.",
            GameErrorKind::StaticMapRebuildInProgress =>  "[0x3C] The galaxy is currently rebuilding its static map data.",
            GameErrorKind::StaticMapRebuildLocked =>  "[0x3D] Static map rebuilding is currently blocked by the tournament state.",
            GameErrorKind::DuplicateSubsystemComponentValue {component_kind} => return write!(f, "[0x40] The subsystem component \"{component_kind:?}\" was supplied more than once."),
            GameErrorKind::InvalidPrimitiveValue { value, r#type } => return write!(f, "[0x??] Value {value:?} not expected for  {type:?}"),
        })
    }
}

impl From<&mut dyn PacketReader> for GameErrorKind {
    fn from(reader: &mut dyn PacketReader) -> Self {
        match reader.read_byte() {
            0x01 => GameErrorKind::CantConnect,
            0x02 => GameErrorKind::InvalidProtocolVersion,
            0x03 => GameErrorKind::AuthFailed,
            0x04 => GameErrorKind::WrongAccountState(
                reader.opt_read_byte().map(AccountStatus::from_primitive),
            ),
            0x05 => GameErrorKind::TeamSelectionFailed,
            0x06 => GameErrorKind::SelfDisclosureRequired,
            0x07 => GameErrorKind::PersistenceUnavailable,
            0x08 => GameErrorKind::ServerFullOfPlayerKind(
                reader.opt_read_byte().map(PlayerKind::from_primitive),
            ),
            0x09 => GameErrorKind::AccountAlreadyLoggedIn,
            0x0C => GameErrorKind::SessionsExhausted,
            0x0D => GameErrorKind::InvalidData { message: None },
            0x0F => GameErrorKind::ConnectionTerminated { reason: None },
            0x10 => GameErrorKind::SpecifiedElementNotFound,
            0x11 => GameErrorKind::CantCallThisConcurrent,
            0x12 => GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::from_primitive(reader.read_byte()),
                parameter: reader.read_string(),
            },
            0x13 => GameErrorKind::PermissionFailed,
            0x14 => GameErrorKind::FloodcontrolTriggered,
            0x15 => GameErrorKind::UnitConstraintViolation,
            0x16 => GameErrorKind::InvalidXmlNodeValue {
                reason: InvalidArgumentKind::from_primitive(reader.read_byte()),
                node_path: reader.read_string(),
                hint: reader.read_string(),
            },
            0x17 => GameErrorKind::ControllableIsClosing,
            0x18 => GameErrorKind::AvatarNotAvailable,
            0x20 => GameErrorKind::YouNeedToContinueFirst,
            0x21 => GameErrorKind::YouNeedToDieFirst,
            0x22 => GameErrorKind::AllStartLocationsAreOvercrowded,
            0x30 => GameErrorKind::CanOnlyShootOncePerTick,
            0x31 => GameErrorKind::TournamentNotConfigured,
            0x32 => GameErrorKind::TournamentAlreadyConfigured,
            0x33 => GameErrorKind::TournamentWrongStage,
            0x34 => GameErrorKind::TournamentMapEditingLocked,
            0x35 => GameErrorKind::TournamentRegistrationClosed,
            0x36 => GameErrorKind::TournamentParticipantRequired,
            0x37 => GameErrorKind::TournamentSpectatingForbidden,
            0x38 => GameErrorKind::TournamentTeamMismatch,
            0x39 => GameErrorKind::TournamentModeNotAllowed,
            0x3A => GameErrorKind::PlayerAccessRestricted,
            0x3B => GameErrorKind::AdminAccessRestricted,
            0x3C => GameErrorKind::StaticMapRebuildInProgress,
            0x3D => GameErrorKind::StaticMapRebuildLocked,
            0x40 => GameErrorKind::DuplicateSubsystemComponentValue {
                component_kind: SubsystemComponentKind::from_primitive(reader.read_byte()),
            },
            code => GameErrorKind::Unknown(code),
        }
    }
}
