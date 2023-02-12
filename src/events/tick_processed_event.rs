use serde_derive::{Deserialize, Serialize};
use std::time::Duration;

/// This event informs of the completion of a tick in the [`UniverseGroup`].
///
/// [`UniverseGroup`]: crate::universe_group::UniverseGroup
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickProcessedEvent {
    #[serde(rename = "processingTime")]
    pub(crate) processing_time: f64,
}

impl TickProcessedEvent {
    #[inline]
    pub fn processing_time(&self) -> Duration {
        Duration::from_secs_f64(self.processing_time)
    }
}
