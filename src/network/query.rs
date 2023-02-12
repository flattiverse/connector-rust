use crate::controllable::ControllableId;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryId(String);

impl QueryId {}

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
    #[serde(rename = "controllableContinue")]
    ContinueControllable { controllable: ControllableId },
    #[serde(rename = "controllableKill")]
    KillControllable { controllable: ControllableId },
    #[serde(rename = "controllableNew")]
    NewControllable {
        controllable: ControllableId,
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryResponse {
    Integer(i32),
    Double(f64),
    String(String),
    Empty,
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
    #[error("[0x05] The command you tried to access can't be accessed with your player kind (you tried to access admin commands as a player or vice versa...).")]
    InvalidPlayerKind,
    #[error("Unknown error code: {0}")]
    Other(i32),
    #[error("Unable to receive a response because the connection to the server is no more")]
    ConnectionGone,
}

impl From<i32> for QueryError {
    fn from(value: i32) -> Self {
        match value {
            0x05 => Self::InvalidPlayerKind,
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
    const ALLOWED_LEN: usize = 2;
    const ALLOWED_CHARS: &'static [char] = &[
        '.', '-', '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
        'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
        'Z',
    ];

    pub fn register_new_for(&mut self, target: Sender<QueryResult>) -> Option<QueryId> {
        let id = self.unused_id()?;
        self.queries.push((id.clone(), target));
        Some(id)
    }

    fn unused_id(&self) -> Option<QueryId> {
        let mut id = String::with_capacity(Self::ALLOWED_LEN);

        for mut counter in 0..Self::ALLOWED_CHARS.len().pow(Self::ALLOWED_LEN as u32) {
            // fill the buffer
            for _ in 0..Self::ALLOWED_LEN {
                id.push(Self::ALLOWED_CHARS[counter % Self::ALLOWED_CHARS.len()]);
                counter /= Self::ALLOWED_CHARS.len();
            }

            if !self.contains(&id) {
                return Some(QueryId(id));
            } else {
                id.clear();
            }
        }
        None
    }

    fn contains(&self, id: &str) -> bool {
        for i in 0..self.queries.len() {
            if self.queries[i].0 .0 == id {
                return true;
            }
        }
        false
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
