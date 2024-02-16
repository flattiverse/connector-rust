use crate::error::GameError;
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

    pub fn spawn(self) -> (ConnectionHandle, Receiver<ConnectionEvent>) {
        let (sender, receiver) = async_channel::unbounded();
        let handle = self.handle.clone();
        crate::network::spawn(self.run(sender));
        (handle, receiver)
    }

    async fn run(self, sender: Sender<ConnectionEvent>) {
        loop {
            match self.receiver.recv().await {
                Err(_empty_and_closed) => break,
                Ok(ConnectionEvent::Packet(packet)) => {
                    if packet.header().session() != 0 {
                        self.handle
                            .sessions
                            .lock()
                            .await
                            .resolve(packet.header().session(), packet);
                    } else {
                        if sender.send(ConnectionEvent::Packet(packet)).await.is_err() {
                            break;
                        }
                    }
                }
                Ok(event) => {
                    if sender.send(event).await.is_err() {
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
