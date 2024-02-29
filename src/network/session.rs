use crate::network::Packet;
use arc_swap::ArcSwapOption;
use async_channel::{Receiver, RecvError, Sender};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct SessionId(pub(crate) u8);

pub struct SessionHandler {
    sessions: [ArcSwapOption<Sender<Packet>>; 256],
}

impl Default for SessionHandler {
    fn default() -> Self {
        Self {
            sessions: core::array::from_fn(|_| ArcSwapOption::default()),
        }
    }
}

impl SessionHandler {
    pub fn get(&self) -> Option<Session> {
        let (sender, receiver) = async_channel::unbounded();
        let sender = Arc::new(sender);
        let id = self
            .sessions
            .iter()
            .enumerate()
            .skip(1) // TODo session id of 0 is not allowed
            .find_map(|(id, slot)| {
                if slot
                    .compare_and_swap(&None::<Arc<Sender<Packet>>>, Some(Arc::clone(&sender)))
                    .is_none()
                {
                    return Some(id);
                } else {
                    None
                }
            })
            .expect("Ids exhausted");

        let session = Session {
            id: SessionId(id as _),
            receiver,
        };
        Some(session)
    }

    pub fn resolve(&self, id: SessionId, packet: Packet) {
        if let Some(session) = self.sessions[usize::from(id.0)].swap(None) {
            if let Err(e) = session.try_send(packet) {
                error!("Failed to resolve {id:?}: {e:?}");
            }
        } else {
            error!("Did not find Session for {id:?}")
        }
    }
}

pub struct Session {
    pub(crate) id: SessionId,
    pub(crate) receiver: Receiver<Packet>,
}

impl Session {
    #[inline]
    pub fn id(&self) -> SessionId {
        self.id
    }

    #[inline]
    pub async fn response(self) -> Result<Packet, RecvError> {
        self.receiver.recv().await
    }
}
