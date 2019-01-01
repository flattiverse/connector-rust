
use std::sync::Arc;
use std::ops::Deref;

pub(crate) mod prelude {
    pub use crate::Player;
    pub use crate::unit::ControllableInfo;

    pub(crate) use crate::message::any_game_message::prelude::*;
}

use self::prelude::*;

#[derive(Clone)]
pub enum AnyPlayerUnitDeceasedMessage {
    PlayerUnitCollidedWithPlayerUnitMessage (Arc<PlayerUnitCollidedWithPlayerUnitMessage>),
    PlayerUnitCollidedWithUnitMessage       (Arc<PlayerUnitCollidedWithUnitMessage>),
    PlayerUnitCommittedSuicideMessage       (Arc<PlayerUnitCommittedSuicideMessage>),
    PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage(Arc<PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage>),
    PlayerUnitDeceasedByPolicyMessage(Arc<PlayerUnitDeceasedByPolicyMessage>),
    PlayerUnitLoggedOffMessage              (Arc<PlayerUnitLoggedOffMessage>),
    PlayerUnitResetMessage                  (Arc<PlayerUnitResetMessage>),
    PlayerUnitShotByPlayerUnitMessage       (Arc<PlayerUnitShotByPlayerUnitMessage>),
    PlayerUnitShotByUnitMessage             (Arc<PlayerUnitShotByUnitMessage>),
}

impl Deref for AnyPlayerUnitDeceasedMessage {
    type Target = PlayerUnitDeceasedMessage;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithPlayerUnitMessage  (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithUnitMessage        (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitCommittedSuicideMessage        (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage(ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByPolicyMessage        (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitLoggedOffMessage               (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitResetMessage                   (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitShotByPlayerUnitMessage        (ref message) => message.deref(),
            &AnyPlayerUnitDeceasedMessage::PlayerUnitShotByUnitMessage              (ref message) => message.deref(),
        }
    }
}