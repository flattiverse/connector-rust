use crate::packet::Packet;
use tokio::sync::oneshot::Sender;

const MAX_IDS: usize = 254;
const ID_OFFSET: usize = 1;

pub struct Requests {
    ids: Vec<Option<Sender<Packet>>>,
    last_index: usize,
}

impl Requests {
    pub fn new() -> Self {
        Self {
            ids: vec_of_none!(MAX_IDS),
            last_index: 0,
        }
    }

    pub fn enqueue(&mut self, sender: Sender<Packet>) -> Option<usize> {
        let len = self.ids.len();
        for i in 0..len {
            let index = (i + self.last_index) % len;
            if self.ids[index].is_none() {
                self.ids[index] = Some(sender);
                self.last_index = index + 1;
                return Some(index + ID_OFFSET);
            }
        }
        None
    }

    pub fn take(&mut self, id: usize) -> Option<Sender<Packet>> {
        if self.ids.len() < id {
            self.ids[id].take()
        } else {
            None
        }
    }

    pub fn maybe_respond(&mut self, packet: Packet) -> Option<Packet> {
        if packet.command == 0xFF {
            let session = packet.session;
            if let Some(Some(sender)) = self
                .ids
                .get_mut(usize::from(session) - ID_OFFSET)
                .map(Option::take)
            {
                if let Err(packet) = sender.send(packet) {
                    warn!("Failed to notify session: {}", packet.session);
                } else {
                    debug!("Notified session {}", session);
                }
            }
            None
        } else {
            Some(packet)
        }
    }
}
