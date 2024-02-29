use crate::atomics::Atomic;
use crate::error::GameError;
use crate::hierarchy::{Galaxy, GalaxyConfig, GalaxyId};
use crate::network::{ConnectionHandle, Packet};
use crate::{FlattiverseEvent, UniversalArcHolder};
use arc_swap::ArcSwap;
use async_channel::Sender;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;

pub struct Connection {
    handle: ConnectionHandle,
    receiver: Receiver<ConnectionEvent>,
}

impl Connection {
    #[inline]
    pub fn from_existing(handle: ConnectionHandle, receiver: Receiver<ConnectionEvent>) -> Self {
        Self { handle, receiver }
    }

    pub fn spawn(self) -> Arc<Galaxy> {
        let (event_sender, event_receiver) = async_channel::unbounded();
        let handle = self.handle.clone();
        let galaxy = Arc::new(Galaxy {
            id: Atomic::from(GalaxyId(0)),
            config: ArcSwap::new(Arc::new(GalaxyConfig::default())),

            clusters: UniversalArcHolder::with_capacity(256),
            ship_designs: UniversalArcHolder::with_capacity(256),
            teams: UniversalArcHolder::with_capacity(256),
            controllables: UniversalArcHolder::with_capacity(256),
            players: UniversalArcHolder::with_capacity(256),

            connection: handle,
            login_completed: Atomic::from(false),
            events: event_receiver,
        });
        crate::runtime::spawn(self.run(Arc::clone(&galaxy), event_sender));
        galaxy
    }

    async fn run(mut self, galaxy: Arc<Galaxy>, sender: Sender<FlattiverseEvent>) {
        loop {
            match self.receiver.recv().await {
                None => break,
                Some(ConnectionEvent::PingMeasured(duration)) => {
                    if sender
                        .send(FlattiverseEvent::PingMeasured(duration))
                        .await
                        .is_err()
                    {
                        warn!("Galaxy gone, shutting down connection!");
                        break;
                    }
                }
                Some(ConnectionEvent::Packet(packet)) => {
                    if packet.header().session() != 0 {
                        self.handle
                            .sessions
                            .lock()
                            .await
                            .resolve(packet.header().session(), packet);
                    } else {
                        match galaxy.on_packet(packet) {
                            Ok(None) => {}
                            Ok(Some(event)) => {
                                if sender.send(event).await.is_err() {
                                    warn!("Galaxy gone, shutting down connection!");
                                }
                            }
                            Err(e) => {
                                error!("Failed to process packet: {e:?}");
                                break;
                            }
                        }
                    }
                }
                Some(ConnectionEvent::GameError(e)) => {
                    error!("Connection error: {e:?}");
                    break;
                }
                Some(ConnectionEvent::Closed(reason)) => {
                    warn!("Connection closed with reason={reason:?}");
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ConnectionEvent {
    PingMeasured(Duration),
    Packet(Packet),
    GameError(GameError),
    Closed(Option<String>),
}
