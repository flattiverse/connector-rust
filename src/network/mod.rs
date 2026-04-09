pub const PROTOCOL_VERSION: &str = "29";

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

mod invalid_argument_kind;
pub use invalid_argument_kind::*;

mod chunked_transfer;
pub use chunked_transfer::*;

use crate::galaxy_hierarchy::{BuildDisclosure, Galaxy, RuntimeDisclosure};
use crate::game_error::GameError;
use crate::{FlattiverseEvent, GameErrorKind};
use async_channel::Receiver;
use std::fmt::Write;
use std::sync::Arc;

#[instrument(level = "trace", skip(f))]
pub(crate) async fn connect(
    uri: &str,
    auth: &str,
    team: Option<&str>,
    runtime_disclosure: Option<RuntimeDisclosure>,
    build_disclosure: Option<BuildDisclosure>,
    f: impl FnOnce(ConnectionHandle, Receiver<FlattiverseEvent>) -> Arc<Galaxy>,
) -> Result<Arc<Galaxy>, ConnectError> {
    let url = {
        let mut url = String::new();

        write!(
            &mut url,
            "{uri}?auth={auth}&version={PROTOCOL_VERSION}&impl=rust&impl-version={}&impl-target={}&impl-arch={}&impl-family={}&impl-os={}",
            env!("CARGO_PKG_VERSION"),
            if cfg!(feature = "desktop") {
                "desktop"
            } else if cfg!(feature = "wasm") {
                "wasm"
            } else {
                "unknown"
            },
            std::env::consts::ARCH,
            std::env::consts::FAMILY,
            std::env::consts::OS,
        ).unwrap();

        if let Some(team) = team {
            write!(&mut url, "&team={team}").unwrap();
        }

        if let Some(runtime_disclosure) = runtime_disclosure {
            write!(&mut url, "&runtimeDisclosure={runtime_disclosure}").unwrap();
        }

        if let Some(build_disclosure) = build_disclosure {
            write!(&mut url, "&buildDisclosure={build_disclosure}").unwrap();
        }

        url
    };

    debug!("Connecting to {}", url);

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
    pub fn game_error_from_http_status_code(code: u16) -> ConnectError {
        debug!("Trying to map HTTP status code {code} to GameErrorKind");
        match code {
            400 | 502 | 504 => ConnectError::from(GameErrorKind::CantConnect),
            401 => ConnectError::from(GameErrorKind::AuthFailed),
            409 => ConnectError::from(GameErrorKind::InvalidProtocolVersion),
            _ => {
                #[cfg(all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_os = "unknown"
                ))]
                {
                    ConnectError::Unknown(format!("Unexpected StatusCode {code:?}"))
                }
                #[cfg(not(all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_os = "unknown"
                )))]
                {
                    let status_code = reqwest::StatusCode::from_u16(code);
                    ConnectError::Unknown(format!(
                        "Unexpected StatusCode {:?}",
                        match &status_code {
                            Ok(code) => code as &dyn std::fmt::Debug,
                            Err(_) => &code as &dyn std::fmt::Debug,
                        }
                    ))
                }
            }
        }
    }
}

impl From<GameErrorKind> for ConnectError {
    #[inline]
    fn from(kind: GameErrorKind) -> Self {
        Self::GameError(GameError::from(kind))
    }
}

#[cfg(feature = "desktop")]
impl From<tokio_tungstenite::tungstenite::Error> for ConnectError {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        if let tokio_tungstenite::tungstenite::Error::Http(response) = value {
            Self::game_error_from_http_status_code(response.status().as_u16())
        } else {
            Self::IoError(value)
        }
    }
}
