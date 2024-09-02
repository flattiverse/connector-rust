use crate::account::AccountStatus;
use crate::galaxy_hierarchy::PlayerKind;
use crate::network::{InvalidArgumentKind, Packet, PacketReader};
use num_enum::{FromPrimitive, TryFromPrimitive, TryFromPrimitiveError};
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub struct GameError {
    kind: Box<GameErrorKind>,
}

impl GameError {
    #[inline]
    pub fn kind(&self) -> &GameErrorKind {
        &self.kind
    }

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
    InvalidOrMissingTeam,
    ServerFullOfPlayerKind(Option<PlayerKind>),
    SessionsExhausted,
    ConnectionTerminated,
    SpecifiedElementNotFound,
    CantCallThisConcurrent,
    InvalidArgument {
        reason: InvalidArgumentKind,
        parameter: String,
    },
    PermissionFailed,

    // TODO local only
    ParameterNotWithinSpecification,
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
            GameErrorKind::InvalidOrMissingTeam => "[0x05] No or non-existent team specified.",
            GameErrorKind::ServerFullOfPlayerKind(None) => "[0x08] Server is full of unknown things.",
            GameErrorKind::ServerFullOfPlayerKind(Some(kind)) => match kind {
                PlayerKind::Admin => "[0x08] Server is full of admins. (Too many admins connected to the galaxy server.)",
                PlayerKind::Spectator => "[0x08] Server is full of spectators. (Too many spectators connected to the galaxy server.)",
                PlayerKind::Player => "[0x08] All player slots are taken. Please wait until players leave the galaxy.",
                PlayerKind::Unknown(id) => return write!(f, "[0x08] Server is full of things with code {:#02x}.", id)
            },
            GameErrorKind::SessionsExhausted => "[0x0C] Sessions exhausted: You cannot have more than 255 calls in progress.",
            GameErrorKind::ConnectionTerminated => "[0x0F] Connection has been terminated for unknown reason.",
            GameErrorKind::SpecifiedElementNotFound => "[0x05] No or non-existent team specified.",
            GameErrorKind::InvalidArgument {
                reason,
                parameter
            } => return write!(f, "[0x12] Parameter {parameter:?} {}", match reason {
                InvalidArgumentKind::TooSmall => "is wrong due to an too small value.",
                InvalidArgumentKind::TooLarge => "is wrong due to an too large value.",
                InvalidArgumentKind::NameConstraint => "doesn't match the name constraint.",
                InvalidArgumentKind::ChatConstraint => "doesn't match the chat constraint.",
                InvalidArgumentKind::EntityNotFound => "doesn't point to an existing entity.",
                InvalidArgumentKind::ConstrainedNaN => "contained a \"Not a Number\" value.",
                InvalidArgumentKind::ConstrainedInfinity => "contained a \"Infinity\" value.",
                InvalidArgumentKind::Unknown(..) => "is wrong due to an invalid value."
            }),
            GameErrorKind::PermissionFailed => "[0x13] Permission denied. Did you try to call a command where you don't have access to?",
            GameErrorKind::CantCallThisConcurrent => "[0x11] This method cannot be called concurrently.",
            GameErrorKind::ParameterNotWithinSpecification => "[0x??] Parameters are not within specification.",
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
            0x05 => GameErrorKind::InvalidOrMissingTeam,
            0x08 => GameErrorKind::ServerFullOfPlayerKind(
                reader.opt_read_byte().map(PlayerKind::from_primitive),
            ),
            0x0C => GameErrorKind::SessionsExhausted,
            0x0F => GameErrorKind::ConnectionTerminated,
            0x10 => GameErrorKind::SpecifiedElementNotFound,
            0x11 => GameErrorKind::CantCallThisConcurrent,
            0x12 => GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::from_primitive(reader.read_byte()),
                parameter: reader.read_string(),
            },
            0x13 => GameErrorKind::PermissionFailed,
            code => GameErrorKind::Unknown(code),
        }
    }
}
