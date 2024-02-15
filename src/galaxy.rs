use crate::error::{GameError, GameErrorKind};
use crate::events::FlattiverseEvent;
use crate::network::{ConnectError, ConnectionHandle};
use async_channel::Receiver;

pub struct Galaxy {
    connection: ConnectionHandle,
    receiver: Receiver<Result<FlattiverseEvent, GameError>>,
}

impl Galaxy {
    pub async fn join(uri: &str, auth: &str, team: u8) -> Result<Self, GameError> {
        let connection = crate::network::connect(uri, auth, team)
            .await
            .map_err(|e| match e {
                ConnectError::GameError(error) => error,
                e => GameError::from(GameErrorKind::GenericException)
                    .with_info(format!("Failed to connect due to local issues: {e}")),
            })?;
        let (handle, receiver) = connection.spawn();
        Ok(Self {
            connection: handle,
            receiver,
        })
    }

    pub async fn receive(&mut self) -> Result<FlattiverseEvent, GameError> {
        match self.receiver.recv().await {
            Ok(result) => result,
            Err(_) => Err(GameErrorKind::ConnectionClosed.into()),
        }
    }

    pub fn poll_receive(&mut self) -> Option<Result<FlattiverseEvent, GameError>> {
        match self.receiver.try_recv() {
            Ok(result) => Some(result),
            Err(e) if e.is_closed() => Some(Err(GameErrorKind::ConnectionClosed.into())),
            _ => None,
        }
    }

    #[inline]
    pub fn receiver_queue_len(&self) -> usize {
        self.receiver.len()
    }
}
