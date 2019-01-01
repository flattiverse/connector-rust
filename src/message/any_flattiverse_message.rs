
use std::sync::Arc;
use std::ops::Deref;

use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

pub(crate) mod prelude {
    pub use crate::dotnet::DateTime;

    pub(crate) use crate::message::*;
}

use self::prelude::*;

#[derive(Clone)]
pub enum AnyMessage {
    ChatMessage  (AnyChatMessage),
    GameMessage  (AnyGameMessage),
    SystemMessage(AnySystemMessage),
}

impl AnyMessage {

    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<AnyMessage, Error> {
        Ok(match packet.path_sub() {
            0x00 => AnyMessage::SystemMessage(AnySystemMessage   ::SystemMessage         (Arc::new(SystemMessageData     ::from_packet(connector, packet, reader)?))),
            0x01 => AnyMessage::ChatMessage  (AnyChatMessage     ::UniCastChatMessage    (Arc::new(UnicastChatMessage    ::from_packet(connector, packet, reader)?))),
            0x02 => AnyMessage::ChatMessage  (AnyChatMessage     ::TeamCastChatMessage   (Arc::new(TeamCastChatMessage   ::from_packet(connector, packet, reader)?))),
            0x03 => AnyMessage::ChatMessage  (AnyChatMessage     ::BroadCastChatMessage  (Arc::new(BroadCastChatMessage  ::from_packet(connector, packet, reader)?))),
            0x04 => AnyMessage::ChatMessage  (AnyChatMessage     ::BinaryChatMessage     (Arc::new(BinaryChatMessage     ::from_packet(connector, packet, reader)?))),
            0x08 => AnyMessage::SystemMessage(AnySystemMessage   ::MOTDMessage           (Arc::new(MOTDMessage           ::from_packet(connector, packet, reader)?))),
            0x10 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitCommittedSuicideMessage      (Arc::new(PlayerUnitCommittedSuicideMessage      ::from_packet(connector, packet, reader)?)))),
            0x11 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithUnitMessage      (Arc::new(PlayerUnitCollidedWithUnitMessage      ::from_packet(connector, packet, reader)?)))),
            0x12 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithPlayerUnitMessage(Arc::new(PlayerUnitCollidedWithPlayerUnitMessage::from_packet(connector, packet, reader)?)))),
            0x13 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitShotByUnitMessage            (Arc::new(PlayerUnitShotByUnitMessage            ::from_packet(connector, packet, reader)?)))),
            0x14 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitShotByPlayerUnitMessage      (Arc::new(PlayerUnitShotByPlayerUnitMessage      ::from_packet(connector, packet, reader)?)))),
            0x15 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitLoggedOffMessage             (Arc::new(PlayerUnitLoggedOffMessage             ::from_packet(connector, packet, reader)?)))),
            0x16 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitResetMessage                 (Arc::new(PlayerUnitResetMessage                 ::from_packet(connector, packet, reader)?)))),
            0x17 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage(Arc::new(PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage::from_packet(connector, packet, reader)?)))),
            0x18 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitDeceasedMessage(AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByPolicyMessage(Arc::new(PlayerUnitDeceasedByPolicyMessage::from_packet(connector, packet, reader)?)))),
            0x20 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitContinuedMessage           (Arc::new(PlayerUnitContinuedMessage            ::from_packet(connector, packet, reader)?))),
            0x30 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitHitMissionTargetMessage    (Arc::new(PlayerUnitHitMissionTargetMessage     ::from_packet(connector, packet, reader)?))),
            0x31 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitHitOwnTargetMessage        (Arc::new(PlayerUnitHitOwnTargetMessage         ::from_packet(connector, packet, reader)?))),
            0x32 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitHitEnemyTargetMessage      (Arc::new(PlayerUnitHitEnemyTargetMessage       ::from_packet(connector, packet, reader)?))),
            0x33 => AnyMessage::GameMessage(AnyGameMessage::MissionTargetAvailableMessage        (Arc::new(MissionTargetAvailableMessage         ::from_packet(connector, packet, reader)?))),
            0x34 => AnyMessage::GameMessage(AnyGameMessage::TargetDominationStartedMessage       (Arc::new(TargetDominationStartedMessage        ::from_packet(connector, packet, reader)?))),
            0x35 => AnyMessage::GameMessage(AnyGameMessage::TargetDominationFinishedMessage      (Arc::new(TargetDominationFinishedMessage       ::from_packet(connector, packet, reader)?))),
            0x36 => AnyMessage::GameMessage(AnyGameMessage::TargetDominationScoredMessage        (Arc::new(TargetDominationScoredMessage         ::from_packet(connector, packet, reader)?))),
            0x37 => AnyMessage::GameMessage(AnyGameMessage::TargetDedominationStartedMessage     (Arc::new(TargetDedominationStartedMessage      ::from_packet(connector, packet, reader)?))),
            0x38 => AnyMessage::GameMessage(AnyGameMessage::GateSwitchedMessage                  (Arc::new(GateSwitchedMessage                   ::from_packet(connector, packet, reader)?))),
            0x40 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitJumpedMessage              (Arc::new(PlayerUnitJumpedMessage               ::from_packet(connector, packet, reader)?))),
            0x50 => AnyMessage::GameMessage(AnyGameMessage::PlayerJoinedUniverseGroupMessage     (Arc::new(PlayerJoinedUniverseGroupMessage      ::from_packet(connector, packet, reader)?))),
            0x51 => AnyMessage::GameMessage(AnyGameMessage::PlayerPartedUniverseGroupMessage     (Arc::new(PlayerPartedUniverseGroupMessage      ::from_packet(connector, packet, reader)?))),
            0x52 => AnyMessage::GameMessage(AnyGameMessage::PlayerDroppedFromUniverseGroupMessage(Arc::new(PlayerDroppedFromUniverseGroupMessage ::from_packet(connector, packet, reader)?))),
            0x53 => AnyMessage::GameMessage(AnyGameMessage::PlayerKickedFromUniverseGroupMessage (Arc::new(PlayerKickedFromUniverseGroupMessage  ::from_packet(connector, packet, reader)?))),
            0x60 => AnyMessage::GameMessage(AnyGameMessage::UniverseGroupResetPendingMessage     (Arc::new(UniverseGroupResetPendingMessage      ::from_packet(connector, packet, reader)?))),
            0x61 => AnyMessage::GameMessage(AnyGameMessage::UniverseGroupResetMessage            (Arc::new(UniverseGroupResetMessage             ::from_packet(connector, packet, reader)?))),
            0x62 => AnyMessage::GameMessage(AnyGameMessage::TournamentStatusMessage              (Arc::new(TournamentStatusMessage               ::from_packet(connector, packet, reader)?))),
            0x70 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitBuildMessage(AnyPlayerUnitBuildMessage::PlayerUnitBuildStartMessage    (Arc::new(PlayerUnitBuildStartMessage    ::from_packet(connector, packet, reader)?)))),
            0x71 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitBuildMessage(AnyPlayerUnitBuildMessage::PlayerUnitBuildCancelledMessage(Arc::new(PlayerUnitBuildCancelledMessage::from_packet(connector, packet, reader)?)))),
            0x72 => AnyMessage::GameMessage(AnyGameMessage::PlayerUnitBuildMessage(AnyPlayerUnitBuildMessage::PlayerUnitBuildFinishedMessage (Arc::new(PlayerUnitBuildFinishedMessage ::from_packet(connector, packet, reader)?)))),
            _ => return Err(Error::UnknownMessageType),
        })
    }
}

impl Deref for AnyMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        match self {
            AnyMessage::ChatMessage  (ref message) => match message {
                AnyChatMessage::BinaryChatMessage   (ref message) => message.deref(),
                AnyChatMessage::BroadCastChatMessage(ref message) => message.deref(),
                AnyChatMessage::TeamCastChatMessage (ref message) => message.deref(),
                AnyChatMessage::UniCastChatMessage  (ref message) => message.deref(),
            },
            AnyMessage::GameMessage  (ref message) => match message {
                AnyGameMessage::GateSwitchedMessage                  (ref message) => message.deref(),
                AnyGameMessage::MissionTargetAvailableMessage        (ref message) => message.deref(),
                AnyGameMessage::PlayerDroppedFromUniverseGroupMessage(ref message) => message.deref(),
                AnyGameMessage::PlayerJoinedUniverseGroupMessage     (ref message) => message.deref(),
                AnyGameMessage::PlayerKickedFromUniverseGroupMessage (ref message) => message.deref(),
                AnyGameMessage::PlayerPartedUniverseGroupMessage     (ref message) => message.deref(),
                AnyGameMessage::PlayerUnitBuildMessage               (ref message) => match message {
                    AnyPlayerUnitBuildMessage::PlayerUnitBuildCancelledMessage(ref message) => message.deref(),
                    AnyPlayerUnitBuildMessage::PlayerUnitBuildFinishedMessage (ref message) => message.deref(),
                    AnyPlayerUnitBuildMessage::PlayerUnitBuildStartMessage    (ref message) => message.deref(),
                },
                AnyGameMessage::PlayerUnitContinuedMessage           (ref message) => message.deref(),
                AnyGameMessage::PlayerUnitDeceasedMessage            (ref message) => match message {
                    AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithPlayerUnitMessage(ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitCollidedWithUnitMessage      (ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitCommittedSuicideMessage      (ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByBadHullRefreshingPowerUpMessage(ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitDeceasedByPolicyMessage      (ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitLoggedOffMessage             (ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitResetMessage                 (ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitShotByPlayerUnitMessage      (ref message) => message.deref(),
                    AnyPlayerUnitDeceasedMessage::PlayerUnitShotByUnitMessage            (ref message) => message.deref(),
                },
                AnyGameMessage::PlayerUnitHitEnemyTargetMessage      (ref message) => message.deref(),
                AnyGameMessage::PlayerUnitHitMissionTargetMessage    (ref message) => message.deref(),
                AnyGameMessage::PlayerUnitHitOwnTargetMessage        (ref message) => message.deref(),
                AnyGameMessage::PlayerUnitJumpedMessage              (ref message) => message.deref(),
                AnyGameMessage::TargetDedominationStartedMessage     (ref message) => message.deref(),
                AnyGameMessage::TargetDominationFinishedMessage      (ref message) => message.deref(),
                AnyGameMessage::TargetDominationScoredMessage        (ref message) => message.deref(),
                AnyGameMessage::TargetDominationStartedMessage       (ref message) => message.deref(),
                AnyGameMessage::TournamentStatusMessage              (ref message) => message.deref(),
                AnyGameMessage::UniverseGroupResetMessage            (ref message) => message.deref(),
                AnyGameMessage::UniverseGroupResetPendingMessage     (ref message) => message.deref(),
            },
            AnyMessage::SystemMessage(ref message) => match message {
                AnySystemMessage::SystemMessage(ref message) => message.deref(),
                AnySystemMessage::MOTDMessage  (ref message) => message.deref(),
            },
        }
    }
}