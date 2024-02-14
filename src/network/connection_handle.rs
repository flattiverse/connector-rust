use async_channel::Sender;

pub struct ConnectionHandle {
    pub(crate) sender: Sender<SenderData>,
}

impl ConnectionHandle {}

pub enum SenderData {
    #[cfg(not(feature = "wasm"))]
    Raw(tokio_tungstenite::tungstenite::Message),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("The connection is no longer reachable, your request could not be transmitted")]
    ConnectionGone,
    #[error("Cannot issue more query ids: all possible query ids are in use")]
    QueryIdsExhausted,
}
