
use std::sync::Arc;
use std::ops::Deref;

pub(crate) mod prelude {
    pub use Player;
    pub use unit::ControllableInfo;

    pub(crate) use message::any_game_message::prelude::*;
}

use self::prelude::*;

#[derive(Clone)]
pub enum AnyPlayerUnitBuildMessage {
    PlayerUnitBuildCancelledMessage (Arc<PlayerUnitBuildCancelledMessage>),
    PlayerUnitBuildFinishedMessage  (Arc<PlayerUnitBuildFinishedMessage>),
    PlayerUnitBuildStartMessage     (Arc<PlayerUnitBuildStartMessage>),
}

impl Deref for AnyPlayerUnitBuildMessage {
    type Target = PlayerUnitBuildMessage;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyPlayerUnitBuildMessage::PlayerUnitBuildCancelledMessage(ref message) => message.deref(),
            &AnyPlayerUnitBuildMessage::PlayerUnitBuildFinishedMessage (ref message) => message.deref(),
            &AnyPlayerUnitBuildMessage::PlayerUnitBuildStartMessage    (ref message) => message.deref(),
        }
    }
}