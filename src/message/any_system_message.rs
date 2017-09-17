
use std::sync::Arc;
use std::ops::Deref;

pub(crate) mod prelude {
    pub(crate) use message::any_flattiverse_message::prelude::*;
}

use self::prelude::*;

#[derive(Clone)]
pub enum AnySystemMessage {
    SystemMessage(Arc<SystemMessageData>),
    MOTDMessage  (Arc<MOTDMessage>),
}

impl Deref for AnySystemMessage {
    type Target = SystemMessage;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnySystemMessage::SystemMessage(ref message) => message.deref(),
            &AnySystemMessage::MOTDMessage  (ref message) => message.deref(),
        }
    }
}