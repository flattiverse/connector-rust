use crate::con::ServerMessage;
use crate::packet::Command;
use std::future::Future;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

pub struct ConnectionHandle {
    pub(crate) sender: mpsc::UnboundedSender<ConnectionCommand>,
    pub(crate) handle: JoinHandle<()>,
}

impl ConnectionHandle {
    pub fn send_block_command(
        &self,
        command: impl Into<Command>,
    ) -> Result<oneshot::Receiver<ServerMessage>, ConnectionHandleError> {
        let (sender, receiver) = oneshot::channel();
        match self.sender.send(ConnectionCommand::SendBlockCommand {
            command: command.into(),
            block_consumer: sender,
        }) {
            Err(_) => Err(ConnectionHandleError::ConnectionGone),
            Ok(_) => Ok(receiver),
        }
    }

    #[inline]
    pub fn send_block_command_mapped(
        &self,
        command: impl Into<Command>,
    ) -> Result<impl Future<Output=Result<(), ConnectionHandleError>>, ConnectionHandleError>
    {
        Ok(Self::mapped_response_future(
            self.send_block_command(command)?,
        ))
    }

    #[inline]
    async fn mapped_response_future(
        receiver: oneshot::Receiver<ServerMessage>,
    ) -> Result<(), ConnectionHandleError> {
        Self::map_response(receiver.await)
    }

    fn map_response(
        response: Result<ServerMessage, oneshot::error::RecvError>,
    ) -> Result<(), ConnectionHandleError> {
        match response {
            Err(_) => Err(ConnectionHandleError::ConnectionGone),
            Ok(ServerMessage::Error { result, .. }) => {
                Err(ConnectionHandleError::ServerError(result))
            }
            Ok(ServerMessage::Success { .. }) => Ok(()),
            Ok(events @ ServerMessage::Events(..)) => {
                panic!("Unexpected server response: {events:?}")
            }
        }
    }
}

pub enum ConnectionCommand {
    SendBlockCommand {
        command: Command,
        block_consumer: oneshot::Sender<ServerMessage>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionHandleError {
    #[error("The connection is no longer reachable, your request could not be transmitted")]
    ConnectionGone,
    #[error("The server encountered an error processing your request: {0}")]
    ServerError(String),
}
