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

pub fn connect(uri: &str, auth: &str, team: u8) -> Result<Connection, ConnectError> {
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
    return driver_wasm::connect(&url);

    #[cfg(not(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    )))]
    return driver::connect(&url);
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
