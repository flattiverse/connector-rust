use crate::error::GameError;
use crate::hierarchy::Galaxy;
use crate::network::{ConnectionHandle, Packet, SessionId};
use crate::{FlattiverseEvent, GameErrorKind};
use async_channel::Sender;
use std::sync::Weak;
use std::time::Duration;

pub struct Connection {
    pub(crate) handle: ConnectionHandle,
    pub(crate) galaxy: Weak<Galaxy>,
    pub(crate) sender: Sender<FlattiverseEvent>,
}

impl Connection {
    pub(crate) fn on_close(&self) {
        let _ = self.sender.try_send(FlattiverseEvent::ConnectionClosed);
        self.sender.close();
    }

    #[allow(unused)] // TODO JS-WebSockets do not support Ping/Pong
    pub(crate) fn on_ping_measured(&self, duration: Duration) -> Result<(), GameError> {
        self.sender
            .try_send(FlattiverseEvent::PingMeasured(duration))
            .map_err(|_| GameError::from(GameErrorKind::ConnectionClosed))
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
                            Err(GameErrorKind::ConnectionClosed.into())
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
            Err(GameErrorKind::ConnectionClosed.into())
        }
    }
}
