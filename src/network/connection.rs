use crate::network::{ConnectionHandle, PacketReader};
use async_channel::Receiver;

pub struct Connection {
    handle: ConnectionHandle,
    receiver: Receiver<ConnectionEvent>,
}

impl Connection {
    #[inline]
    pub fn from_existing(handle: ConnectionHandle, receiver: Receiver<ConnectionEvent>) -> Self {
        Self { handle, receiver }
    }

    pub async fn receive(&mut self) -> Option<ConnectionEvent> {
        let x = self.receiver.recv().await;
        // TODO
        x.ok()
    }
}

#[derive(Debug)]
pub enum ConnectionEvent {
    ReceivedMessage { player: u8, message: String },
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
