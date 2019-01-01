
use std::sync::Arc;
use std::ops::Deref;

pub(crate) mod prelude {
    pub(crate) use crate::message::any_flattiverse_message::prelude::*;
}

use self::prelude::*;

#[derive(Clone)]
pub enum AnyGameMessage {
    GateSwitchedMessage                  (Arc<GateSwitchedMessage>),
    MissionTargetAvailableMessage        (Arc<MissionTargetAvailableMessage>),
    PlayerDroppedFromUniverseGroupMessage(Arc<PlayerDroppedFromUniverseGroupMessage>),
    PlayerJoinedUniverseGroupMessage     (Arc<PlayerJoinedUniverseGroupMessage>),
    PlayerKickedFromUniverseGroupMessage (Arc<PlayerKickedFromUniverseGroupMessage>),
    PlayerPartedUniverseGroupMessage     (Arc<PlayerPartedUniverseGroupMessage>),
    PlayerUnitBuildMessage               (AnyPlayerUnitBuildMessage),
    PlayerUnitContinuedMessage           (Arc<PlayerUnitContinuedMessage>),
    PlayerUnitDeceasedMessage           (AnyPlayerUnitDeceasedMessage),
    PlayerUnitHitEnemyTargetMessage      (Arc<PlayerUnitHitEnemyTargetMessage>),
    PlayerUnitHitMissionTargetMessage    (Arc<PlayerUnitHitMissionTargetMessage>),
    PlayerUnitHitOwnTargetMessage        (Arc<PlayerUnitHitOwnTargetMessage>),
    PlayerUnitJumpedMessage              (Arc<PlayerUnitJumpedMessage>),
    TargetDedominationStartedMessage     (Arc<TargetDedominationStartedMessage>),
    TargetDominationFinishedMessage      (Arc<TargetDominationFinishedMessage>),
    TargetDominationScoredMessage        (Arc<TargetDominationScoredMessage>),
    TargetDominationStartedMessage       (Arc<TargetDominationStartedMessage>),
    TournamentStatusMessage              (Arc<TournamentStatusMessage>),
    UniverseGroupResetMessage            (Arc<UniverseGroupResetMessage>),
    UniverseGroupResetPendingMessage     (Arc<UniverseGroupResetPendingMessage>),
}

impl Deref for AnyGameMessage {
    type Target = GameMessage;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyGameMessage::GateSwitchedMessage                  (ref message) => message.deref(),
            &AnyGameMessage::MissionTargetAvailableMessage        (ref message) => message.deref(),
            &AnyGameMessage::PlayerDroppedFromUniverseGroupMessage(ref message) => message.deref(),
            &AnyGameMessage::PlayerJoinedUniverseGroupMessage     (ref message) => message.deref(),
            &AnyGameMessage::PlayerKickedFromUniverseGroupMessage (ref message) => message.deref(),
            &AnyGameMessage::PlayerPartedUniverseGroupMessage     (ref message) => message.deref(),
            &AnyGameMessage::PlayerUnitBuildMessage               (ref message) => match message {
                &AnyPlayerUnitBuildMessage::PlayerUnitBuildCancelledMessage(ref message) => message.deref(),
                &AnyPlayerUnitBuildMessage::PlayerUnitBuildFinishedMessage (ref message) => message.deref(),
                &AnyPlayerUnitBuildMessage::PlayerUnitBuildStartMessage    (ref message) => message.deref(),
            },
            &AnyGameMessage::PlayerUnitContinuedMessage           (ref message) => message.deref(),
            &AnyGameMessage::PlayerUnitDeceasedMessage            (ref message) => match message {
                &AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithPlayerUnitMessage(ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithUnitMessage      (ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitCommittedSuicideMessage      (ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage(ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByPolicyMessage      (ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitLoggedOffMessage             (ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitResetMessage                 (ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitShotByPlayerUnitMessage      (ref message) => message.deref(),
                &AnyPlayerUnitDeceasedMessage::PlayerUnitShotByUnitMessage            (ref message) => message.deref(),
            },
            &AnyGameMessage::PlayerUnitHitEnemyTargetMessage      (ref message) => message.deref(),
            &AnyGameMessage::PlayerUnitHitMissionTargetMessage    (ref message) => message.deref(),
            &AnyGameMessage::PlayerUnitHitOwnTargetMessage        (ref message) => message.deref(),
            &AnyGameMessage::PlayerUnitJumpedMessage              (ref message) => message.deref(),
            &AnyGameMessage::TargetDedominationStartedMessage     (ref message) => message.deref(),
            &AnyGameMessage::TargetDominationFinishedMessage      (ref message) => message.deref(),
            &AnyGameMessage::TargetDominationScoredMessage        (ref message) => message.deref(),
            &AnyGameMessage::TargetDominationStartedMessage       (ref message) => message.deref(),
            &AnyGameMessage::TournamentStatusMessage              (ref message) => message.deref(),
            &AnyGameMessage::UniverseGroupResetMessage            (ref message) => message.deref(),
            &AnyGameMessage::UniverseGroupResetPendingMessage     (ref message) => message.deref(),
        }
    }
}