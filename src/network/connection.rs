use crate::error::{GameError, GameErrorKind};
use crate::events::FlattiverseEvent;
use crate::network::{ConnectionHandle, Packet};
use async_channel::{Receiver, Sender};
use std::time::Duration;

pub struct Connection {
    handle: ConnectionHandle,
    receiver: Receiver<ConnectionEvent>,
}

impl Connection {
    #[inline]
    pub fn from_existing(handle: ConnectionHandle, receiver: Receiver<ConnectionEvent>) -> Self {
        Self { handle, receiver }
    }

    pub fn spawn(
        self,
    ) -> (
        ConnectionHandle,
        Receiver<Result<FlattiverseEvent, GameError>>,
    ) {
        let (sender, receiver) = async_channel::unbounded();
        let handle = self.handle.clone();
        crate::network::spawn(self.run(sender));
        (handle, receiver)
    }

    async fn run(self, sender: Sender<Result<FlattiverseEvent, GameError>>) {
        loop {
            match self.receiver.recv().await {
                Err(_empty_and_closed) => break,
                Ok(ConnectionEvent::Closed(msg)) => {
                    let err = GameError::from(GameErrorKind::ConnectionClosed);
                    let err = if let Some(msg) = msg {
                        err.with_info(msg)
                    } else {
                        err
                    };
                    let _ = sender.send(Err(err)).await;
                    break;
                }
                Ok(ConnectionEvent::PingMeasured(ping)) => {
                    if sender
                        .send(Ok(FlattiverseEvent::PingMeasured(ping)))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Ok(ConnectionEvent::Packet(packet)) => {
                    self.handle
                        .sessions
                        .lock()
                        .await
                        .resolve(packet.header().session(), packet);
                }
                Ok(ConnectionEvent::GameError(e)) => {
                    if sender.send(Err(e)).await.is_err() {
                        break;
                    }
                }
            }
        }
        // TODO cleanup
    }
}

#[derive(Debug)]
pub enum ConnectionEvent {
    PingMeasured(Duration),
    Packet(Packet),
    GameError(GameError),
    Closed(Option<String>),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Received an unexpected command code: 0x{0:02x}")]
    UnexpectedCommand(u8),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError {
    #[error("The underlying connection no longer exists")]
    ConnectionGone,
    #[error("{0}")]
    GameError(GameError),
}
