use crate::events::added_unit_event::AddedUnitEvent;
use crate::events::chat_multicast_event::ChatMulticastEvent;
use crate::events::chat_teamcast_event::ChatTeamcastEvent;
use crate::events::chat_unicast_event::ChatUnicastEvent;
use crate::events::controllable_unregistered::ControllableUnregistered;
use crate::events::death_controllable_event::DeathControllableEvent;
use crate::events::depleted_resource_event::DepletedResourceEvent;
use crate::events::full_update_player_event::FullUpdatePlayerEvent;
use crate::events::partial_update_player_event::PartialUpdatePlayerEvent;
use crate::events::removed_player_event::RemovedPlayerEvent;
use crate::events::removed_unit_event::RemovedUnitEvent;
use crate::events::tick_processed_event::TickProcessedEvent;
use crate::events::universe_group_info_event::UniverseGroupInfoEvent;
use crate::events::updated_controllable_event::UpdatedControllableEvent;
use crate::events::updated_unit_event::UpdatedUnitEvent;
use crate::events::FailureEvent;
use crate::network::query::{QueryId, QueryResponse};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod connection;
pub mod connection_handle;
pub mod query;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum ServerMessage {
    #[serde(rename = "success")]
    Success {
        id: QueryId,
        result: Option<QueryResponse>,
    },
    #[serde(rename = "failure")]
    Failure { id: QueryId, code: i32 },
    #[serde(rename = "events")]
    Events { events: Vec<ServerEvent> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    #[serde(rename = "chatMulticast")]
    ChatMulticast(ChatMulticastEvent),
    #[serde(rename = "chatTeamcast")]
    ChatTeamcastEvent(ChatTeamcastEvent),
    #[serde(rename = "chatUnicast")]
    ChatUnicastEvent(ChatUnicastEvent),
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
    #[serde(rename = "unitUpdated")]
    UnitUpdated(UpdatedUnitEvent),
    #[serde(rename = "tickProcessed")]
    TickProcessed(TickProcessedEvent),
    #[serde(rename = "universeGroupInfo")]
    UniverseGroupInfo(UniverseGroupInfoEvent),
    #[serde(rename = "controllableUpdated")]
    ControllableUpdated(Box<UpdatedControllableEvent>),
    #[serde(rename = "controllableDeath")]
    ControllableDeath(DeathControllableEvent),
    #[serde(rename = "controllableUnregistered")]
    ControllableUnregistered(ControllableUnregistered),
    #[serde(rename = "resourceDepleted")]
    ResourceDeplete(DepletedResourceEvent),
}
