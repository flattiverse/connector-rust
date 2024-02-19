use crate::network::Packet;
use num_enum::FromPrimitive;
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub struct GameError {
    code: GameErrorKind,
    info: Option<String>,
}

impl GameError {
    #[inline]
    pub fn with_info(mut self, info: impl Into<String>) -> Self {
        self.info = Some(info.into());
        self
    }

    #[inline]
    pub fn with_info_opt(mut self, info: Option<String>) -> Self {
        self.info = info;
        self
    }

    #[inline]
    pub fn kind(&self) -> GameErrorKind {
        self.code
    }

    pub(crate) fn check<T>(
        mut packet: Packet,
        f: impl FnOnce(Packet) -> Result<T, GameError>,
    ) -> Result<T, GameError> {
        if packet.header().command() == 0xFF {
            Err(
                GameError::from(GameErrorKind::from_primitive(packet.header().command()))
                    .with_info_opt({
                        if packet.header().size() > 0 {
                            Some(packet.read(|reader| reader.read_string()))
                        } else {
                            None
                        }
                    }),
            )
        } else {
            f(packet)
        }
    }
}

impl From<GameErrorKind> for GameError {
    fn from(code: GameErrorKind) -> Self {
        Self { code, info: None }
    }
}

impl From<u8> for GameError {
    #[inline]
    fn from(value: u8) -> Self {
        Self::from(GameErrorKind::from(value))
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[0x{:02X}] {}", u8::from(self.code), self.code)?;
        if let Some(info) = self.info.as_deref() {
            write!(f, " {info}")?;
        }
        Ok(())
    }
}

#[repr(u8)]
#[derive(
    Debug,
    thiserror::Error,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
)]
pub enum GameErrorKind {
    #[error("The requested element doesn't exist or can't be accessed.")]
    ElementDoesntExist = 0x30,
    #[error("The parameter doesn't match the specification.")]
    ParameterNotWithinSpecification = 0x31,
    #[error("The object can't have more of these. Object full?")]
    CannotAddAlreayFull = 0x32,
    #[error("There is no compatible or available kind.")]
    ThereIsNoSuchKind = 0x33,
    #[error("You don't have permission to alter this element.")]
    NotConfigurable = 0x34,
    #[error("Unit has been created but wasn't there when the session returned.")]
    CreatedButMissing = 0x35,
    #[error("Unauthorized request. You prorbaly aren't the right kind of client: Player, Spectator or Admin.")]
    CommandDoesntExist = 0xE0,
    #[error("An unknown error occurred.")]
    UnknownError = 0xF0,
    #[error(
        "Couldn't connect to the universe server. Are you online? Is flattiverse still online?"
    )]
    CouldntEstablishConnection = 0xF1,
    #[error("The reverse proxy of the flattiverse universe is online but the corresponding galaxy is offline. This may be due to maintenance reasons or the galaxy software version is being upgraded.")]
    ReverseProxyCouldntEstablishConnection = 0xF2,
    #[error("The call could not be processed. Either you didn't make a WebSocket call or the database is not available.")]
    CallCouldNotBeProcessed = 0xF3,
    #[error("Authorization failed, possibly because one of these reasons: auth parameter, ambiguous or unknown, or no spectators allowed.")]
    AuthorizatoinFailed = 0xF4,
    #[error("Forbidden. You are not allowed to perform this action.")]
    Forbidden = 0xF5,
    #[error("The connector you are using is outdated.")]
    ConnectorOutdated = 0xF6,
    #[error("Login failed because you are already online.")]
    AlreadyOnline = 0xF7,
    #[error("Specified element doesn't exist.")]
    ElementNotFound = 0xF8,
    #[error("Command didn't affect any database rows.")]
    CommandHadNoEffect = 0xF9,
    #[error("Given name is invalid.")]
    NameIsInvalid = 0xFA,
    #[error("Given name already exists.")]
    NameAlreadyExists = 0xFB,
    #[error("Can't create element because maximum for this kind is reached.")]
    MaximumReached = 0xFC,
    #[error("The network connection has been closed.")]
    ConnectionClosed = 0xFE,
    #[error("Generic exception thrown.")]
    GenericException = 0xFF,
    #[num_enum(catch_all)]
    #[error("Unspecified GameError code received. Outdated connector somehow?!")]
    Unspecified(u8) = 0x00,
}
