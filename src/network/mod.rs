pub const PROTOCOL_VERSION: &'static str = "4";

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

mod fetch;

mod connection_handle;

use async_channel::Receiver;
pub use connection_handle::*;
use std::sync::Arc;

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

use crate::error::GameError;
use crate::hierarchy::Galaxy;
use crate::FlattiverseEvent;

pub(crate) async fn connect(
    uri: &str,
    auth: &str,
    team: u8,
    f: impl FnOnce(ConnectionHandle, Receiver<FlattiverseEvent>) -> Arc<Galaxy>,
) -> Result<Arc<Galaxy>, ConnectError> {
    let team = Some(team).filter(|t| *t < 32);
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

#[inline]
pub async fn get_text(uri: &str) -> Result<String, ConnectError> {
    fetch::get_text(uri).await
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
        match code {
            502 | 504 => 0xF2,
            400 => 0xF3,
            401 => 0xF4,
            409 => 0xF6,
            412 => 0xF7,
            415 => 0xF8,
            _ => 0xF1,
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
