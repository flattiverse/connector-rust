
mod motd_message;
mod game_message;
mod chat_message;
mod system_message;
mod binary_chat_message;
mod unicast_chat_message;
mod gate_switched_message;
mod team_cast_chat_message;
mod broad_cast_chat_message;
mod player_unit_build_message;
mod player_unit_reset_message;
mod tournament_status_message;
mod player_unit_jumped_message;
mod player_unit_deceased_message;
mod universe_group_reset_message;
mod player_unit_continued_message;
mod player_unit_logged_off_message;
mod player_unit_build_start_message;
mod player_unit_build_cancel_message;
mod mission_target_available_message;
mod player_unit_shot_by_unit_message;
mod target_domination_scored_message;
mod target_domination_started_message;
mod target_domination_finished_message;
mod player_unit_shot_by_player_message;
mod player_unit_hit_own_target_message;
mod player_unit_build_finished_message;
mod target_dedomination_started_message;
mod player_joined_universe_group_message;
mod player_parted_universe_group_message;
mod player_unit_hit_enemy_target_message;
mod universe_group_reset_pending_message;
mod player_unit_committed_suicide_message;
mod player_dropped_universe_group_message;
mod player_unit_hit_mission_target_message;
mod player_unit_collided_with_unit_message;
mod player_unit_collided_with_player_message;
mod player_kicked_from_universe_group_message;
mod player_unit_deceased_by_bad_hull_refreshing_power_up_message;

pub use self::motd_message::*;
pub use self::game_message::*;
pub use self::chat_message::*;
pub use self::system_message::*;
pub use self::binary_chat_message::*;
pub use self::unicast_chat_message::*;
pub use self::gate_switched_message::*;
pub use self::team_cast_chat_message::*;
pub use self::broad_cast_chat_message::*;
pub use self::player_unit_build_message::*;
pub use self::player_unit_reset_message::*;
pub use self::tournament_status_message::*;
pub use self::player_unit_jumped_message::*;
pub use self::player_unit_deceased_message::*;
pub use self::universe_group_reset_message::*;
pub use self::player_unit_continued_message::*;
pub use self::player_unit_logged_off_message::*;
pub use self::player_unit_build_start_message::*;
pub use self::player_unit_build_cancel_message::*;
pub use self::mission_target_available_message::*;
pub use self::player_unit_shot_by_unit_message::*;
pub use self::target_domination_scored_message::*;
pub use self::target_domination_started_message::*;
pub use self::target_domination_finished_message::*;
pub use self::player_unit_shot_by_player_message::*;
pub use self::player_unit_hit_own_target_message::*;
pub use self::player_unit_build_finished_message::*;
pub use self::target_dedomination_started_message::*;
pub use self::player_joined_universe_group_message::*;
pub use self::player_parted_universe_group_message::*;
pub use self::player_unit_hit_enemy_target_message::*;
pub use self::universe_group_reset_pending_message::*;
pub use self::player_unit_committed_suicide_message::*;
pub use self::player_dropped_universe_group_message::*;
pub use self::player_unit_hit_mission_target_message::*;
pub use self::player_unit_collided_with_unit_message::*;
pub use self::player_unit_collided_with_player_message::*;
pub use self::player_kicked_from_universe_group_message::*;
pub use self::player_unit_deceased_by_bad_hull_refreshing_power_up_message::*;



use std::fmt;
use std::fmt::Display;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use net::Packet;
use net::BinaryReader;
use dotnet::DateTime;

use downcast::Any;


downcast!(FlattiverseMessage);
pub trait FlattiverseMessage : Any + Display + Send + Sync {
    fn timestamp(&self) -> &DateTime;
}

pub struct FlattiverseMessageData {
    timestamp: DateTime,
}

impl FlattiverseMessageData {
    fn from_packet(_: &Arc<Connector>, _: &Packet, reader: &mut BinaryReader) -> Result<FlattiverseMessageData, Error> {
        Ok(FlattiverseMessageData {
            timestamp: DateTime::from_ticks(reader.read_i64()?),
        })
    }
}

impl<T: 'static + Borrow<FlattiverseMessageData> + BorrowMut<FlattiverseMessageData> + Display + Send + Sync> FlattiverseMessage for T {
    fn timestamp(&self) -> &DateTime {
        &self.borrow().timestamp
    }
}

impl fmt::Display for FlattiverseMessageData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}



pub fn from_reader(connector: &Arc<Connector>, packet: &Packet) -> Result<Box<FlattiverseMessage>, Error> {
    let path_sub = packet.path_sub();
    let reader = &mut packet.read() as &mut BinaryReader;

    match path_sub {
        0x00 => Ok(Box::new(SystemMessageData                           ::from_packet(connector, packet, reader)?)),
        0x01 => Ok(Box::new(UnicastChatMessageData                      ::from_packet(connector, packet, reader)?)),
        0x02 => Ok(Box::new(TeamCastChatMessageData                     ::from_packet(connector, packet, reader)?)),
        0x03 => Ok(Box::new(BroadCastChatMessageData                    ::from_packet(connector, packet, reader)?)),
        0x04 => Ok(Box::new(BinaryChatMessageData                       ::from_packet(connector, packet, reader)?)),
        0x08 => Ok(Box::new(MOTDMessageData                             ::from_packet(connector, packet, reader)?)),
        0x10 => Ok(Box::new(PlayerUnitCommittedSuicideMessageData       ::from_packet(connector, packet, reader)?)),
        0x11 => Ok(Box::new(PlayerUnitCollidedWithUnitMessageData       ::from_packet(connector, packet, reader)?)),
        0x12 => Ok(Box::new(PlayerUnitCollidedWithPlayerUnitMessageData ::from_packet(connector, packet, reader)?)),
        0x13 => Ok(Box::new(PlayerUnitShotByUnitMessageData             ::from_packet(connector, packet, reader)?)),
        0x14 => Ok(Box::new(PlayerUnitShotByPlayerUnitMessageData       ::from_packet(connector, packet, reader)?)),
        0x15 => Ok(Box::new(PlayerUnitLoggedOffMessageData              ::from_packet(connector, packet, reader)?)),
        0x16 => Ok(Box::new(PlayerUnitResetMessageData                  ::from_packet(connector, packet, reader)?)),
        0x17 => Ok(Box::new(PlayerUnitDeceasedByBadHullRefreshingPowerUpMessageData ::from_packet(connector, packet, reader)?)),
        0x20 => Ok(Box::new(PlayerUnitContinuedMessageData              ::from_packet(connector, packet, reader)?)),
        0x30 => Ok(Box::new(PlayerUnitHitMissionTargetMessageData       ::from_packet(connector, packet, reader)?)),
        0x31 => Ok(Box::new(PlayerUnitHitOwnTargetMessageData           ::from_packet(connector, packet, reader)?)),
        0x32 => Ok(Box::new(PlayerUnitHitEnemyTargetMessageData         ::from_packet(connector, packet, reader)?)),
        0x33 => Ok(Box::new(MissionTargetAvailableMessageData           ::from_packet(connector, packet, reader)?)),
        0x34 => Ok(Box::new(TargetDominationStartedMessageData          ::from_packet(connector, packet, reader)?)),
        0x35 => Ok(Box::new(TargetDominationFinishedMessageData         ::from_packet(connector, packet, reader)?)),
        0x36 => Ok(Box::new(TargetDominationScoredMessageData           ::from_packet(connector, packet, reader)?)),
        0x37 => Ok(Box::new(TargetDedominationStartedMessageData        ::from_packet(connector, packet, reader)?)),
        0x38 => Ok(Box::new(GateSwitchedMessageData                     ::from_packet(connector, packet, reader)?)),
        0x40 => Ok(Box::new(PlayerUnitJumpedMessageData                 ::from_packet(connector, packet, reader)?)),
        0x50 => Ok(Box::new(PlayerJoinedUniverseGroupMessageData        ::from_packet(connector, packet, reader)?)),
        0x51 => Ok(Box::new(PlayerPartedUniverseGroupMessageData        ::from_packet(connector, packet, reader)?)),
        0x52 => Ok(Box::new(PlayerDroppedFromUniverseGroupMessageData   ::from_packet(connector, packet, reader)?)),
        0x53 => Ok(Box::new(PlayerKickedFromUniverseGroupMessageData    ::from_packet(connector, packet, reader)?)),
        0x60 => Ok(Box::new(UniverseGroupResetPendingMessageData        ::from_packet(connector, packet, reader)?)),
        0x61 => Ok(Box::new(UniverseGroupResetMessageData               ::from_packet(connector, packet, reader)?)),
        0x62 => Ok(Box::new(TournamentStatusMessageData                 ::from_packet(connector, packet, reader)?)),
        0x70 => Ok(Box::new(PlayerUnitBuildStartMessageData             ::from_packet(connector, packet, reader)?)),
        0x71 => Ok(Box::new(PlayerUnitBuildCancelledMessageData         ::from_packet(connector, packet, reader)?)),
        0x72 => Ok(Box::new(PlayerUnitBuildFinishedMessageData          ::from_packet(connector, packet, reader)?)),
        _ => Err(Error::UnknownMessageType)
    }
}