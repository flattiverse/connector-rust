use crate::galaxy_hierarchy::{ClusterId, Galaxy, GameMode, PlayerId, PlayerKind, TeamId};
use crate::game_error::GameError;
use crate::network::{ConnectionHandle, Packet, SessionId};
use crate::{FlattiverseEvent, FlattiverseEventKind, GameErrorKind};
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
    pub(crate) fn on_close(&self) {
        self.sender.close();
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
            .map_err(|_| GameError::from(GameErrorKind::ConnectionTerminated))
    }

    pub(crate) fn handle(&self, packet: Packet) -> Result<(), GameError> {
        if let Some(galaxy) = self.galaxy.upgrade() {
            if packet.header().session() != 0 {
                self.handle
                    .sessions
                    .resolve(SessionId(packet.header().session()), packet);
                Ok(())
            } else {
                match self.on_packet(packet, &galaxy) {
                    Ok(None) => Ok(()),
                    Ok(Some(event)) => {
                        if self.sender.try_send(event).is_err() {
                            error!("Event-Receiver gone, shutting down connection!");
                            Err(GameErrorKind::ConnectionTerminated.into())
                        } else {
                            Ok(())
                        }
                    }
                    Err(e) => {
                        error!("Failed to process packet: {e:?}");
                        Err(e)
                    }
                }
            }
        } else {
            error!("Galaxy gone, shutting down connection!");
            Err(GameErrorKind::ConnectionTerminated.into())
        }
    }

    pub(crate) fn on_packet(
        &self,
        mut packet: Packet,
        galaxy: &Arc<Galaxy>,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!(
            "Processing packet with command=0x{:02x}",
            packet.header().command()
        );

        let command = packet.header().command();
        packet.read(|reader| match command {
            0x00 => galaxy.ping_pong(reader.read_uint16()),
            0x01 => galaxy.update_galaxy(
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
                reader.read_uint16(),
                reader.read_uint16(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
            ),
            0x02 => galaxy.update_team(
                TeamId(reader.read_byte()),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_byte(),
                reader.read_string(),
            ),
            0x03 => galaxy.deactivate_team(TeamId(reader.read_byte())),
            0x06 => galaxy.update_cluster(ClusterId(reader.read_byte()), reader.read_string()),
            0x07 => galaxy.deactivate_cluster(ClusterId(reader.read_byte())),
            0x10 => galaxy.create_player(
                PlayerId(reader.read_byte()),
                PlayerKind::from_primitive(reader.read_byte()),
                TeamId(reader.read_byte()),
                reader.read_string(),
                reader.read_f32(),
            ),
            0x11 => galaxy.update_player(PlayerId(reader.read_byte()), reader.read_f32()),
            0x13 => galaxy.deactivate_player(PlayerId(reader.read_byte())),
            _ => {
                warn!("Received packet with unknown command={command:#02x}",);
                Ok(None)
            }
        })
    }
}
