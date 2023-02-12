use crate::network::connection::SenderData;
use crate::network::query::{Query, QueryCommand, QueryError, QueryKeeper, QueryResult};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::task::JoinHandle;

pub struct ConnectionHandle {
    pub(crate) sender: mpsc::UnboundedSender<SenderData>,
    pub(crate) queries: Arc<Mutex<QueryKeeper>>,
    #[allow(unused)]
    pub(crate) handle: JoinHandle<()>,
}

impl ConnectionHandle {
    pub async fn send_query(
        &self,
        command: impl Into<QueryCommand>,
    ) -> Result<impl Future<Output = QueryResult> + 'static, SendQueryError> {
        let (sender, receiver) = oneshot::channel();
        let id = self
            .queries
            .lock()
            .await
            .register_new_for(sender)
            .ok_or(SendQueryError::QueryIdsExhausted)?;
        match self.sender.send(SenderData::Query(Query {
            id,
            command: command.into(),
        })) {
            Ok(_) => Ok(async move {
                match receiver.await {
                    Err(_) => Err(QueryError::ConnectionGone),
                    Ok(Err(e)) => Err(e),
                    Ok(Ok(r)) => Ok(r),
                }
            }),
            Err(_) => Err(SendQueryError::ConnectionGone),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SendQueryError {
    #[error("The connection is no longer reachable, your request could not be transmitted")]
    ConnectionGone,
    #[error("Cannot issue more query ids: all possible query ids are in use")]
    QueryIdsExhausted,
}
