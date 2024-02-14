use crate::error::GameError;
use crate::network::{ConnectionHandle, PacketReader};
use async_channel::Receiver;
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

    #[inline]
    pub fn handle(&self) -> &ConnectionHandle {
        &self.handle
    }

    pub fn try_receive(&mut self) -> Option<Result<ConnectionEvent, ReceiveError>> {
        match self.receiver.try_recv() {
            Ok(event) => match event {
                ConnectionEvent::GameError(error) => Some(Err(ReceiveError::GameError(error))),
                event => Some(Ok(event)),
            },
            Err(e) if e.is_empty() => None,
            Err(_) => Some(Err(ReceiveError::ConnectionGone)),
        }
    }

    #[inline]
    pub async fn receive(&mut self) -> Result<ConnectionEvent, ReceiveError> {
        self.receiver
            .recv()
            .await
            .map_err(|_| ReceiveError::ConnectionGone)
            .and_then(|event| match event {
                ConnectionEvent::GameError(error) => Err(ReceiveError::GameError(error)),
                event => Ok(event),
            })
    }

    #[inline]
    pub fn receiver_queue_len(&self) -> usize {
        self.receiver.len()
    }
}

#[derive(Debug)]
pub enum ConnectionEvent {
    PingMeasured(Duration),
    ReceivedMessage { player: u8, message: String },
    GameError(GameError),
    Closed(Option<String>),
}

impl<'a> TryFrom<PacketReader<'a>> for ConnectionEvent {
    type Error = ParseError;

    fn try_from(mut reader: PacketReader<'a>) -> Result<Self, Self::Error> {
        match reader.header().command() {
            0x30 => Ok(ConnectionEvent::ReceivedMessage {
                player: reader.header().player(),
                message: reader.read_string(usize::from(reader.header().size())),
            }),
            c => Err(ParseError::UnexpectedCommand(c)),
        }
    }
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
