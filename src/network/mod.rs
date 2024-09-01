pub const PROTOCOL_VERSION: &str = "0";

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
mod driver_wasm;

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
mod driver;

mod connection_handle;
pub use connection_handle::*;

mod packet_header;
pub use packet_header::PacketHeader;

mod packet;
pub use packet::Packet;

mod packet_reader;
pub use packet_reader::PacketReader;

mod packet_writer;
pub use packet_writer::PacketWriter;

mod connection;
pub use connection::*;

mod session;
pub use session::*;

use async_channel::Receiver;
use std::sync::Arc;
use crate::galaxy_hierarchy::Galaxy;
use crate::game_error::GameError;
use crate::{FlattiverseEvent, GameErrorKind};

pub(crate) async fn connect(
    uri: &str,
    auth: &str,
    team: Option<&str>,
    f: impl FnOnce(ConnectionHandle, Receiver<FlattiverseEvent>) -> Arc<Galaxy>,
) -> Result<Arc<Galaxy>, ConnectError> {
    let url = format!(
        "{uri}?auth={auth}&version={}{}{}&impl=rust&impl-version={}",
        PROTOCOL_VERSION,
        team.map(|_| "&team=").unwrap_or_default(),
        team.unwrap_or_default(),
        env!("CARGO_PKG_VERSION"),
    );

    #[cfg(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    ))]
    return driver_wasm::connect(&url, f).await;

    #[cfg(not(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    )))]
    return driver::connect(&url, f).await;
}


#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("Unknown error: {0}")]
    Unknown(String),

    #[cfg_attr(feature = "desktop", error("Underlying connection error"))]
    #[cfg(feature = "desktop")]
    IoError(tokio_tungstenite::tungstenite::Error),
    #[cfg_attr(feature = "desktop", error("The provided url is malformed: {0}"))]
    #[cfg(feature = "desktop")]
    MalformedHostUrl(url::ParseError),
    #[cfg_attr(
        feature = "desktop",
        error("The url to the proxy server is malformed: {0}")
    )]
    #[cfg(feature = "desktop")]
    MalformedProxyUrl(url::ParseError),
    #[cfg_attr(
        feature = "desktop",
        error("Failed to connect to the proxy server: {0}")
    )]
    #[cfg(feature = "desktop")]
    ProxyConnectionError(std::io::Error),
    #[cfg_attr(
        feature = "desktop",
        error("The proxy server sent and unexpected response: {0}")
    )]
    #[cfg(feature = "desktop")]
    ProxyResponseError(#[from] async_http_proxy::HttpError),

    #[error("{0}")]
    GameError(GameError),
}

impl ConnectError {
    pub fn game_error_from_http_status_code(code: u16) -> GameError {
        debug!("Trying to map HTTP status code {code} to GameErrorKind");
        match code {
            400 | 502 | 504 => GameErrorKind::CantConnect,
            401 => GameErrorKind::AuthFailed,
            409 => GameErrorKind::InvalidProtocolVersion,
            _ => GameErrorKind::CantConnect,
        }
        .into()
    }
}

#[cfg(feature = "desktop")]
impl From<tokio_tungstenite::tungstenite::Error> for ConnectError {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        if let tokio_tungstenite::tungstenite::Error::Http(response) = value {
            Self::GameError(Self::game_error_from_http_status_code(
                response.status().as_u16(),
            ))
        } else {
            Self::IoError(value)
        }
    }
}
