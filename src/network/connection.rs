use crate::game_error::GameError;
use crate::galaxy_hierarchy::Galaxy;
use crate::network::{ConnectionHandle, Packet, SessionId};
use crate::{FlattiverseEvent, FlattiverseEventKind, GameErrorKind};
use async_channel::Sender;
use std::sync::Weak;
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


    #[cfg_attr(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    ), allow(unused))] // TODO JS-WebSockets do not support Ping/Pong atm
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
                match galaxy.on_packet(packet) {
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
}
