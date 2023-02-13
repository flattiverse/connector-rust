use serde_derive::{Deserialize, Serialize};

/// A unit that has not been properly identified yet. Use the analyzer system to find out the actual
/// kind.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reduced {
    #[serde(rename = "probableKind")]
    pub probable_kind: String,
}
