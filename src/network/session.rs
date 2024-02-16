use crate::network::Packet;
use async_channel::{unbounded, Receiver, Sender};

pub type SessionId = u8;

pub struct SessionHandler {
    sessions: [Option<Sender<Packet>>; 256],
}

impl Default for SessionHandler {
    fn default() -> Self {
        Self {
            sessions: core::array::from_fn(|_| None),
        }
    }
}

impl SessionHandler {
    pub fn get(&mut self) -> Option<Session> {
        self.sessions
            .iter_mut()
            .enumerate()
            .skip(1) // TODo session id of 0 is not allowed
            .filter(|(_, s)| s.is_none())
            .find_map(|(id, slot)| {
                let (sender, receiver) = unbounded();
                let session = Session {
                    id: id as SessionId,
                    receiver,
                };
                *slot = Some(sender);
                Some(session)
            })
    }

    pub fn resolve(&mut self, id: SessionId, packet: Packet) {
        if let Some(session) = core::mem::take(&mut self.sessions[usize::from(id)]) {
            let _ = session.try_send(packet);
        } else {
            error!("Did not find Session for id={id}")
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
}
