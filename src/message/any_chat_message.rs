
use std::sync::Arc;
use std::ops::Deref;

pub(crate) mod prelude {
    pub use crate::Player;

    pub(crate) use crate::message::any_flattiverse_message::prelude::*;
}

use self::prelude::*;

#[derive(Clone)]
pub enum AnyChatMessage {
    BinaryChatMessage   (Arc<BinaryChatMessage>),
    BroadCastChatMessage(Arc<BroadCastChatMessage>),
    TeamCastChatMessage (Arc<TeamCastChatMessage>),
    UniCastChatMessage  (Arc<UnicastChatMessage>)
}

impl Deref for AnyChatMessage {
    type Target = ChatMessage;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyChatMessage::BinaryChatMessage   (ref message) => message.deref(),
            &AnyChatMessage::BroadCastChatMessage(ref message) => message.deref(),
            &AnyChatMessage::TeamCastChatMessage (ref message) => message.deref(),
            &AnyChatMessage::UniCastChatMessage  (ref message) => message.deref(),
        }
    }
}