use serde_derive::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub id: QueryId,
    #[serde(flatten)]
    pub command: QueryCommand,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum QueryCommand {
    #[serde(rename = "whoami")]
    WhoAmI,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryResponse {
    Double(f64),
    Integer(i32),
    String(String),
}

impl QueryResponse {
    #[inline]
    pub fn get_double(&self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn get_integer(&self) -> Option<i32> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn get_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_str()),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("Unknown error code: {0}")]
    Other(i32),
}

impl From<i32> for QueryError {
    fn from(value: i32) -> Self {
        match value {
            _ => Self::Other(value),
        }
    }
}

pub type QueryResult = Result<QueryResponse, QueryError>;

#[derive(Default)]
pub struct QueryKeeper {
    queries: Vec<(QueryId, Sender<QueryResult>)>,
}

impl QueryKeeper {
    pub fn register_new_for(&mut self, target: Sender<QueryResult>) -> QueryId {
        let id = QueryId(Uuid::new_v4().to_string());
        self.queries.push((id.clone(), target));
        id
    }

    pub fn answer(&mut self, id: &QueryId, result: QueryResult) -> Option<QueryResult> {
        match self.unblock(id) {
            Some(target) => target.send(result).err(),
            None => Some(result),
        }
    }

    pub fn unblock(&mut self, id: &QueryId) -> Option<Sender<QueryResult>> {
        for i in 0..self.queries.len() {
            if self.queries[i].0 == *id {
                return Some(self.queries.swap_remove(i).1);
            }
        }
        None
    }
}
