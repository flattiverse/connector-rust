use crate::error::GameError;
use crate::network::{ConnectionHandle, Packet};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

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
        let (sender, receiver) = tokio::sync::mpsc::channel(124);
        let handle = self.handle.clone();
        crate::network::spawn(self.run(sender));
        (handle, receiver)
    }

    async fn run(mut self, sender: Sender<ConnectionEvent>) {
        loop {
            match self.receiver.recv().await {
                None => break,
                Some(ConnectionEvent::Packet(packet)) => {
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
                Some(event) => {
                    if sender.send(event).await.is_err() {
                        break;
                    }
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
