use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};

use crate::command::id::S2C_SESSION_EXCEPTION;
use crate::io::BinaryReader;
use crate::packet::Packet;

const MAX_IDS: usize = 254;
const ID_OFFSET: usize = 1;

type ResultSender = Sender<Result<Packet, RequestError>>;
type ResultReceiver = Receiver<Result<Packet, RequestError>>;

pub struct Requests {
    ids: Vec<Option<ResultSender>>,
    last_index: usize,
}

impl Default for Requests {
    fn default() -> Self {
        Self::new()
    }
}

impl Requests {
    pub fn new() -> Self {
        Self {
            ids: vec_of_none!(MAX_IDS),
            last_index: 0,
        }
    }

    pub fn enqueue(&mut self, packet: &mut Packet) -> Option<ResultReceiver> {
        let (sender, receiver) = oneshot::channel();
        self.enqueue_with(packet, sender).map(|_| receiver)
    }

    pub fn enqueue_with(&mut self, packet: &mut Packet, sender: ResultSender) -> Option<()> {
        let len = self.ids.len();
        for i in 0..len {
            let index = (i + self.last_index) % len;
            if self.ids[index].is_none() {
                self.ids[index] = Some(sender);
                self.last_index = index + 1;
                packet.session = Some((index + ID_OFFSET) as u8);
                return Some(());
            }
        }
        None
    }

    pub fn maybe_respond(&mut self, packet: Packet) -> Option<Packet> {
        if let Some(session) = packet.session {
            if let Some(Some(sender)) = self.take(session) {
                if packet.command == S2C_SESSION_EXCEPTION {
                    debug!(
                        "Server responded with error message for session {}",
                        session
                    );
                    let error = Requests::parse_request_error(&packet);
                    if let Err(Err(err)) = sender.send(Err(error)) {
                        error!("   » {}", err.general());
                        error!("     {}", err.message());
                    }
                } else if let Err(Ok(packet)) = sender.send(Ok(packet)) {
                    warn!("Failed to notify session {} about {:?}", session, packet);
                } else {
                    debug!("Notified session {}", session);
                }
            }
            None
        } else {
            Some(packet)
        }
    }

    fn take(&mut self, session: u8) -> Option<Option<ResultSender>> {
        self.ids
            .get_mut(usize::from(session) - ID_OFFSET)
            .map(Option::take)
    }

    fn parse_request_error(packet: &Packet) -> RequestError {
        match packet.helper {
            0x10_u8 => RequestError::JoinRefused(packet.sub_address),
            0x11_u8 => RequestError::PartRefused(packet.sub_address),
            0xFF_u8 => {
                let reader = &mut packet.payload() as &mut dyn BinaryReader;
                RequestError::ServerException(format!(
                    "\
                                 The server has caught a '{:?}' and forwarded this to you.\n\
                                 The exception has the following message: \n\n {:?}\n\n\
                                 The exception has the following stack trace: \n\n {:?}\n\n\
                                 Please contact your teacher if you are in the Flattiverse course at the HS-Esslingen",
                    reader.read_string(), reader.read_string(), reader.read_string()
                ))
            }
            code => RequestError::UnknownErrorCode(code, format!("{}", code)),
        }
    }
}

#[derive(Debug)]
pub enum RequestError {
    JoinRefused(u8),
    PartRefused(u8),
    ServerException(String),
    UnknownErrorCode(u8, String),
}

impl RequestError {
    pub fn general(&self) -> &str {
        match self {
            RequestError::JoinRefused(_) => "Join refused",
            RequestError::PartRefused(_) => "Part refused",
            RequestError::ServerException(_) => "Server exception",
            RequestError::UnknownErrorCode(..) => "Unknown error code",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            RequestError::JoinRefused(reason) => match reason {
                0x01 => "You are already assigned to an universe. Pleas part first",
                0x02 => "You specified an invalid team",
                0x03 => "Universe is full (maximum players reached)",
                0x04 => "Selected team is full (maximum players for this team reached)",
                0x05 => "Access denied (You don't have the necessary privileges or are banned from this universe)",
                0x06 => "Access denied (Your join configuration doesn't match the tournament configuration)",
                0x07 => "Universe not ready (e.g. offline)",
                _ => "Denied, but Matthias does not know why :'(",
            },
            RequestError::PartRefused(reason) => match reason {
                0x01 => "You are on no universe",
                0x02 => "You are on another universe",
                _ => "Denied, but Matthias does not know why :'(",
            },
            RequestError::ServerException(msg) => msg.as_str(),
            RequestError::UnknownErrorCode(_, msg) => msg.as_str(),
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}: {}", self.general(), self.message())
    }
}

impl Error for RequestError {}
