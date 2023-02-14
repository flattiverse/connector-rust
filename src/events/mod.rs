use serde_derive::{Deserialize, Serialize};

pub mod added_unit_event;
pub mod chat_multicast_event;
pub mod chat_teamcast_event;
pub mod chat_unicast_event;
pub mod controllable_unregistered;
pub mod death_controllable_event;
pub mod full_update_player_event;
pub mod partial_update_player_event;
pub mod removed_player_event;
pub mod removed_unit_event;
pub mod tick_processed_event;
pub mod universe_group_info_event;
pub mod updated_controllable_event;
pub mod updated_unit_event;

pub trait ApplicableEvent<T> {
    fn apply(self, target: &mut T)
    where
        Self: Sized;
}

/// For events that need to load additional data from `T`.
pub trait Completable<T> {
    fn complete(&mut self, source: &T);
}

impl<T, C: Completable<T>> Completable<T> for Option<C> {
    fn complete(&mut self, source: &T) {
        if let Some(this) = self {
            this.complete(source);
        }
    }
}

/// This event indicates some critical out-of-game failure like a problem with the
/// data-transport, etc.. Consider upgrading the connector if this happens and it
/// is not due to a lost connection.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailureEvent {
    /// The message which indicates the issue that happened.
    pub message: String,
}
