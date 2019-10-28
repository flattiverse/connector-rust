use crate::packet::Packet;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};

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

    pub fn enqueue(&mut self, packet: &mut Packet) -> Option<Receiver<Packet>> {
        let len = self.ids.len();
        for i in 0..len {
            let index = (i + self.last_index) % len;
            if self.ids[index].is_none() {
                let (sender, receiver) = oneshot::channel();

                self.ids[index] = Some(sender);
                self.last_index = index + 1;
                packet.session = (index + ID_OFFSET) as u8;

                return Some(receiver);
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
        if packet.session != 0 {
            let session = packet.session;
            if let Some(Some(sender)) = self
                .ids
                .get_mut(usize::from(session) - ID_OFFSET)
                .map(Option::take)
            {
                if packet.command == 0xFF {
                    // error occured
                    error!("Error occured for session {}", session);
                    match packet.helper {
                        0x10_u8 => {
                            error!("   » Join refused");
                            match packet.sub_address {
                                0x01 => error!("     You are already assigned to an universe. Pleas part first"),
                                0x02 => error!("     You specified an invalid team"),
                                0x03 => error!("     Universe is full (maximum players reached)"),
                                0x04 => error!("     Selected team is full (maximum players for this team reached)"),
                                0x05 => error!("     Access denied (You don't have the necessary privileges or are banned from this universe)"),
                                0x06 => error!("     Access denied (Your join configuration doesn't match the tournament configuration)"),
                                0x07 => error!("     Universe not ready (e.g. offline)"),
                                _ => error!("     Denied, but Matthias does not know why :'("),
                            }
                        }
                        0x11_u8 => {
                            error!("   » Part refused");
                            match packet.sub_address {
                                0x01 => error!("     You are on no universe"),
                                0x02 => error!("     You are on another universe"),
                                _ => error!("     Denied, but Matthias does not know why :'("),
                            }
                        }
                        0xFF_u8 => error!("   » invalid exception by server"),
                        code => error!("   » Unknown exception code {}", code),
                    }
                    drop(sender);
                } else if let Err(packet) = sender.send(packet) {
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
