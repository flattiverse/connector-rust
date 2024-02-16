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
    #[error("An unknown error occurred while connecting to the flattiverse server")]
    UnspecifiedConnectionIssue = 0xF0,
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
    #[error("The connector you are using is outdated.")]
    ConnectorOutdated = 0xF5,
    #[error("Login failed because you are already online.")]
    AlreadyOnline = 0xF6,
    #[error("Specified team doesn't exist or can't be selected.")]
    TeamNotFound = 0xF7,
    #[error("The network connection has been closed.")]
    ConnectionClosed = 0xFE,
    #[error("Generic exception thrown.")]
    GenericException = 0xFF,
    #[num_enum(catch_all)]
    #[error("Unspecified GameError code received. Outdated connector somehow?!")]
    Unspecified(u8) = 0x00,
}
