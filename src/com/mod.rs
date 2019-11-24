#[cfg(not(feature = "wasm"))]
pub mod tokio;

#[cfg(not(feature = "wasm"))]
pub use self::tokio::*;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "wasm")]
pub use self::wasm::*;
