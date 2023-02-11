use crate::events::added_unit_event::AddedUnitEvent;
use crate::events::full_update_player_event::FullUpdatePlayerEvent;
use crate::events::partial_update_player_event::PartialUpdatePlayerEvent;
use crate::events::removed_player_event::RemovedPlayerEvent;
use crate::events::removed_unit_event::RemovedUnitEvent;
use crate::events::tick_processed_event::TickProcessedEvent;
use crate::events::universe_group_info_event::UniverseGroupInfoEvent;
use crate::events::FailureEvent;
use crate::network::query::{QueryId, QueryResponse};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod connection;
pub mod connection_handle;
pub mod query;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerMessage {
    #[serde(rename = "success")]
    Success { id: QueryId, result: QueryResponse },
    #[serde(rename = "failure")]
    Failure { id: QueryId, code: i32 },
    #[serde(rename = "events")]
    Events { events: Vec<ServerEvent> },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerEvent {
    /// A Fallback event for debugging purposes, if the event sent from the server is unknown to the
    /// connector.
    #[serde(rename = "raw")]
    Raw(HashMap<String, serde_json::Value>),
    /// This event indicates some critical out-of-game failure like a problem with the
    /// data-transport, etc.. Consider upgrading the connector if this happens and it
    /// is not due to a lost connection.
    #[serde(rename = "failure")]
    Failure(FailureEvent),
    #[serde(rename = "playerFullUpdate")]
    PlayerFullUpdate(FullUpdatePlayerEvent),
    #[serde(rename = "playerPartialUpdate")]
    PlayerPartialUpdate(PartialUpdatePlayerEvent),
    #[serde(rename = "playerRemoved")]
    PlayerRemoved(RemovedPlayerEvent),
    #[serde(rename = "unitRemoved")]
    UnitRemoved(RemovedUnitEvent),
    #[serde(rename = "unitAdded")]
    UnitAdded(AddedUnitEvent),
    #[serde(rename = "tickProcessed")]
    TickProcessed(TickProcessedEvent),
    #[serde(rename = "universeGroupInfo")]
    UniverseGroupInfo(UniverseGroupInfoEvent),
}
