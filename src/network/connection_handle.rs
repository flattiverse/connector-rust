use crate::network::connection::SenderData;
use crate::network::query::{QueryCommand, QueryResult};
use std::future::Future;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

pub struct ConnectionHandle {
    pub(crate) sender: mpsc::UnboundedSender<SenderData>,
    pub(crate) handle: JoinHandle<()>,
}

impl ConnectionHandle {
    pub fn send_query(
        &self,
        command: impl Into<QueryCommand>,
    ) -> Result<impl Future<Output = Result<QueryResult, SendQueryError>> + 'static, SendQueryError>
    {
        let (sender, receiver) = oneshot::channel();
        match self.sender.send(SenderData::Query(command.into(), sender)) {
            Ok(_) => Ok(async move { receiver.await.map_err(|_| SendQueryError::ConnectionGone) }),
            Err(_) => Err(SendQueryError::ConnectionGone),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SendQueryError {
    #[error("The connection is no longer reachable, your request could not be transmitted")]
    ConnectionGone,
}
