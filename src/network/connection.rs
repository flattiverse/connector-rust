use crate::galaxy_hierarchy::{
    ClusterId, ControllableId, ControllableInfoId, Galaxy, GameMode, PlayerId, PlayerKind, TeamId,
};
use crate::game_error::GameError;
use crate::network::{ConnectionHandle, Packet, SessionId};
use crate::unit::UnitKind;
use crate::utils::Readable;
use crate::{FlattiverseEvent, FlattiverseEventKind, GameErrorKind, PlayerUnitDestroyedReason};
use async_channel::Sender;
use num_enum::FromPrimitive;
use std::sync::{Arc, Weak};
use std::time::Duration;

pub struct Connection {
    pub(crate) handle: ConnectionHandle,
    pub(crate) galaxy: Weak<Galaxy>,
    pub(crate) sender: Sender<FlattiverseEvent>,
}

impl Connection {
    #[inline]
    pub(crate) fn on_close(&self, reason: Option<Arc<str>>) {
        if let Some(reason) = &reason {
            warn!("Closing connection: {reason}");
        }
        self.sender.close();
        self.handle.sessions.close_all(reason);
    }

    #[cfg_attr(
        all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_os = "unknown"
        ),
        allow(unused)
    )] // TODO JS-WebSockets do not support Ping/Pong atm
    pub(crate) fn on_ping_measured(&self, duration: Duration) -> Result<(), GameError> {
        self.sender
            .try_send(FlattiverseEventKind::PingMeasured(duration).into())
            .map_err(|_| {
                GameError::from(GameErrorKind::ConnectionTerminated {
                    reason: Some(Arc::from("Failed to send ping")),
                })
            })
    }

    pub(crate) fn handle(&self, packet: Packet) -> Result<(), GameError> {
        let mut events = Vec::with_capacity(2);
        if let Some(galaxy) = self.galaxy.upgrade() {
            if packet.header().session() != 0 {
                self.handle
                    .sessions
                    .resolve(SessionId(packet.header().session()), packet);
                Ok(())
            } else {
                match self.on_packet(packet, &galaxy, &mut events) {
                    Ok(()) => {
                        for event in events.drain(..) {
                            if self.sender.try_send(event).is_err() {
                                error!("Event-Receiver gone, shutting down connection!");
                                return Err(GameErrorKind::ConnectionTerminated {
                                    reason: Some(Arc::from("Event-Receiver gone")),
                                }
                                .into());
                            }
                        }
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to process packet: {e:?}");
                        Err(e)
                    }
                }
            }
        } else {
            error!("Galaxy gone, shutting down connection!");
            Err(GameErrorKind::ConnectionTerminated {
                reason: Some(Arc::from("Galaxy gone")),
            }
            .into())
        }
    }

    #[instrument(level = "debug", skip_all, fields(command = packet.header().command_hex()))]
    pub(crate) fn on_packet(
        &self,
        mut packet: Packet,
        galaxy: &Arc<Galaxy>,
        events: &mut Vec<FlattiverseEvent>,
    ) -> Result<(), GameError> {
        let command = packet.header().command();
        packet.read(|reader| match command {
            0x00 => galaxy.ping_pong(events, reader.read_uint16()),
            0x01 => galaxy.update_galaxy(
                events,
                GameMode::from_primitive(reader.read_byte()),
                reader.read_string(),
                reader.read_string(),
                reader.read_byte(),
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_string(),
            ),
            0x02 => galaxy.update_team(
                events,
                TeamId(reader.read_byte()),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_string(),
            ),
            0x03 => galaxy.deactivate_team(events, TeamId(reader.read_byte())),
            0x04 => galaxy.update_team_score(
                events,
                TeamId(reader.read_byte()),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_int32(),
            ),
            0x06 => galaxy.update_cluster(
                events,
                ClusterId(reader.read_byte()),
                reader.read_string(),
                reader.read_byte(),
            ),
            0x07 => galaxy.deactivate_cluster(events, ClusterId(reader.read_byte())),
            0x10 => galaxy.create_player(
                events,
                PlayerId(reader.read_byte()),
                PlayerKind::from_primitive(reader.read_byte()),
                TeamId(reader.read_byte()),
                reader.read_string(),
                reader.read_f32(),
                reader.read_byte() != 0,
                reader.read_byte(),
                reader.read_int32(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_byte() != 0,
                reader,
            ),
            0x11 => galaxy.update_player(
                events,
                PlayerId(reader.read_byte()),
                reader.read_f32(),
                reader.read_byte() != 0,
                reader.read_byte(),
                reader.read_int32(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
                reader.read_int64(),
            ),
            0x12 => galaxy.update_player_score(
                events,
                PlayerId(reader.read_byte()),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_int32(),
            ),
            0x1F => galaxy.deactivate_player(events, PlayerId(reader.read_byte())),
            0x20 => galaxy.controllable_info_new(
                events,
                PlayerId(reader.read_byte()),
                UnitKind::read(reader),
                ControllableInfoId(reader.read_byte()),
                reader.read_string(),
                reader.read_boolean(),
            ),
            0x21 => galaxy.controllable_info_alive(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
            ),
            0x22 => galaxy.controllable_info_dead_by_reason(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                PlayerUnitDestroyedReason::from_primitive(reader.read_byte()),
            ),
            0x23 => galaxy.controllable_info_dead_by_neutral_collision(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                UnitKind::read(reader),
                reader.read_string(),
            ),
            0x24 => galaxy.controllable_info_dead_by_player_unit(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                PlayerUnitDestroyedReason::from_primitive(reader.read_byte()),
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
            ),
            0x25 => galaxy.controllable_info_score_updated(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_uint32(),
                reader.read_int32(),
            ),
            0x2F => galaxy.controllable_info_removed(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
            ),
            0x80 => galaxy.controllable_new(
                events,
                UnitKind::read(reader),
                ControllableId(reader.read_byte()),
                ClusterId(reader.read_byte()),
                reader.read_string(),
                reader,
            ),
            0x81 => galaxy.controllable_deceased(events, ControllableId(reader.read_byte())),
            0x82 => galaxy.controllable_updated(
                events,
                ControllableId(reader.read_byte()),
                ClusterId(reader.read_byte()),
                reader,
            ),
            0x8F => galaxy.controllable_removed(events, ControllableId(reader.read_byte())),
            0x8E => galaxy.power_up_collected(
                events,
                ControllableId(reader.read_byte()),
                UnitKind::read(reader),
                reader.read_string(),
                reader.read_f32(),
                reader.read_f32(),
            ),
            0x30 => galaxy.unit_new(
                events,
                ClusterId(reader.read_byte()),
                reader.read_string(),
                UnitKind::read(reader),
                reader,
            ),
            0x31 => galaxy.unit_updated_movement(
                events,
                ClusterId(reader.read_byte()),
                reader.read_string(),
                reader,
            ),
            0x32 => galaxy.unit_updated_state(
                events,
                ClusterId(reader.read_byte()),
                reader.read_string(),
                reader,
            ),
            0x3E => galaxy.unit_updated_by_admin(
                events,
                ClusterId(reader.read_byte()),
                reader.read_string(),
            ),
            0x3F => {
                galaxy.unit_removed(events, ClusterId(reader.read_byte()), reader.read_string())
            }
            0x0B => galaxy.compiled_with(events, reader.read_byte(), reader.read_string()),
            0xC0 => galaxy.universe_tick(
                events,
                reader.read_uint32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_int32(),
            ),
            0xC1 => galaxy.flag_scored_chat(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                TeamId(reader.read_byte()),
                reader.read_string(),
            ),
            0xC2 => galaxy.domination_point_scored_chat(
                events,
                TeamId(reader.read_byte()),
                reader.read_string(),
            ),
            0xC3 => galaxy.own_flag_hit(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                TeamId(reader.read_byte()),
                reader.read_string(),
            ),
            0xC4 => galaxy.chat_galaxy(events, PlayerId(reader.read_byte()), reader.read_string()),
            0xC5 => galaxy.chat_team(events, PlayerId(reader.read_byte()), reader.read_string()),
            0xC6 => galaxy.chat_player(events, PlayerId(reader.read_byte()), reader.read_string()),
            0xC7 => galaxy.mission_target_hit_chat(
                events,
                PlayerId(reader.read_byte()),
                ControllableInfoId(reader.read_byte()),
                reader.read_uint16(),
            ),
            0xC8 => galaxy.system_message(events, reader.read_string()),
            0xC9 => galaxy.flag_reactivated_chat(
                events,
                TeamId(reader.read_byte()),
                reader.read_string(),
            ),
            0xCA => galaxy.gate_switched(events, ClusterId(reader.read_byte()), reader),
            0xCB => galaxy.gate_restored(
                events,
                ClusterId(reader.read_byte()),
                reader.read_string(),
                reader.read_byte(),
            ),
            0xCC => galaxy.binary_chat_player(events, PlayerId(reader.read_byte()), reader),
            0xD0 => galaxy.tournament_upsert(events, reader),
            0xD1 => galaxy.tournament_removed(events),
            0xD2 => galaxy.tournament_message(events, reader.read_string()),
            _ => {
                warn!("Received packet with unknown command={command:#02x}",);
                Ok(())
            }
        })
    }
}
