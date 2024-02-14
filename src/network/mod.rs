pub const PROTOCOL_VERSION: &'static str = "0";

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

pub async fn connect(uri: &str, auth: &str, team: u8) -> Result<Connection, ConnectError> {
    let team = Some(team).filter(|t| *t > 31);
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
    return driver_wasm::connect(&url).await;

    #[cfg(not(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    )))]
    return driver::connect(&url).await;
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

    // --- parsed from status code
    #[error("No auth parameter was given, or a malformed or non-existing auth key was given. A proper auth parameter consists of string of 64 characters representing hex values. A connection as a spectator was attempted, but the UniverseGroup does not allow spectators")]
    MissingAuthOr(Option<String>),
    #[error("A connection with a wrong connector version was attempted.")]
    WrongConnectorVersion(Option<String>),
    #[error("A connection as a player or admin was attempted, but the associated account is still online with another connection. As disconnecting players will linger for a while, a connection may not be possible for a short time even if a previous connection has been closed or severed")]
    StillOnline(Option<String>),
    #[error("A connection with a wrong team was attempted")]
    WrongTeam(Option<String>),
    #[error("The UniverseGroup is currently at capacity and no further connections are possible.")]
    UniverseFull(Option<String>),
    #[error("The UniverseGroup is currently offline.")]
    UniverseOffline(Option<String>),
}

#[cfg(feature = "desktop")]
impl From<tokio_tungstenite::tungstenite::Error> for ConnectError {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        if let tokio_tungstenite::tungstenite::Error::Http(response) = value {
            fn into_msg(
                response: tokio_tungstenite::tungstenite::http::Response<Option<Vec<u8>>>,
            ) -> Option<String> {
                response.into_body().and_then(|b| String::from_utf8(b).ok())
            }

            match response.status().as_u16() {
                401 => Self::MissingAuthOr(into_msg(response)),
                409 => Self::WrongConnectorVersion(into_msg(response)),
                412 => Self::StillOnline(into_msg(response)),
                415 => Self::WrongTeam(into_msg(response)),
                417 => Self::UniverseFull(into_msg(response)),
                502 => Self::UniverseOffline(into_msg(response)),
                _ => Self::IoError(tokio_tungstenite::tungstenite::Error::Http(response)),
            }
        } else {
            Self::IoError(value)
        }
    }
}
