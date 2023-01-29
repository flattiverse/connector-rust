use serde_derive::{Deserialize, Serialize};

pub mod universe_group_info_event;

/// This event indicates some critical out-of-game failure like a problem with the
/// data-transport, etc.. Consider upgrading the connector if this happens and it
/// is not due to a lost connection.
#[derive(Debug, Serialize, Deserialize)]
pub struct FailureEvent {
    /// The message which indicates the issue that happened.
    pub message: String,
}
