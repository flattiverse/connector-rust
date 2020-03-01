use crate::command::id::S2C_SESSION_EXCEPTION;
use crate::entity::Privileges;
use crate::io::BinaryReader;
use crate::packet::Packet;
use futures::channel::oneshot;
use futures::channel::oneshot::{Receiver, Sender};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

const MAX_IDS: usize = u8::max_value() as usize + 1;

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
                packet.session = Some(index as u8);
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
        self.ids.get_mut(usize::from(session)).map(Option::take)
    }

    fn parse_request_error(packet: &Packet) -> RequestError {
        match packet.helper {
            0x01_u8 => RequestError::UniverseServerUnhandled,
            0x05_u8 => RequestError::PermissionDenied(PermissionDenied(Privileges::from(
                packet.sub_address,
            ))),
            0x06_u8 => RequestError::IllegalName(IllegalName),
            0x07_u8 => RequestError::UnitDoesntExist,
            0x10_u8 => RequestError::JoinRefused(packet.sub_address),
            0x11_u8 => RequestError::PartRefused(packet.sub_address),
            0x20_u8 => RequestError::UniverseDoesNotExist,
            0x21_u8 => RequestError::UniverseOffline,
            0x22_u8 => RequestError::UniverseGoneWhileExecutingRequest,
            0x24_u8 => RequestError::GalaxyDoesNotExist,
            0x60_u8 => RequestError::NonEditableUnit,
            0x61_u8 => RequestError::AmbiguousXmlData(AmbiguousXmlData),
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
    UniverseServerUnhandled,
    PermissionDenied(PermissionDenied),
    IllegalName(IllegalName),
    UnitDoesntExist,
    JoinRefused(u8),
    PartRefused(u8),
    UniverseDoesNotExist,
    UniverseOffline,
    UniverseGoneWhileExecutingRequest,
    GalaxyDoesNotExist,
    NonEditableUnit,
    AmbiguousXmlData(AmbiguousXmlData),
    ServerException(String),
    UnknownErrorCode(u8, String),
    InternalIoError(std::io::Error),
}

impl RequestError {
    pub fn general(&self) -> &str {
        match self {
            RequestError::UniverseServerUnhandled => "Universe Server Error",
            RequestError::PermissionDenied(_) => "Permission Denied",
            RequestError::IllegalName(_) => "Illegal Name",
            RequestError::UnitDoesntExist => "Unit doesn't exist",
            RequestError::JoinRefused(_) => "Join refused",
            RequestError::PartRefused(_) => "Part refused",
            RequestError::UniverseDoesNotExist => "Universe does not exist",
            RequestError::UniverseOffline => "Invalid operation",
            RequestError::UniverseGoneWhileExecutingRequest => "Invalid operation",
            RequestError::GalaxyDoesNotExist => "Galaxy does not exist",
            RequestError::NonEditableUnit => "Argument Error",
            RequestError::AmbiguousXmlData(_) => "Ambiguous Xml Data",
            RequestError::ServerException(_) => "Server exception",
            RequestError::UnknownErrorCode(..) => "Unknown error code",
            RequestError::InternalIoError(_) => "Internal IO-Error",
        }
    }

    pub fn message(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            RequestError::UniverseServerUnhandled => "The universe server encountered an exception which has not been handled properly in the code. Additionally the proxy can't understand the exact details of this exception. Please forward this incident to info@flattiverse.com.",
            RequestError::PermissionDenied(pd) => return Cow::Owned(format!("{}", pd.to_string())),
            RequestError::IllegalName(_) => "The given name is illegal",
            RequestError::UnitDoesntExist => "The specified unit doesn't exist or is not available in this context.",
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
            RequestError::UniverseDoesNotExist => "The specified universe doesn't exist",
            RequestError::UniverseOffline => "The universe is offline.",
            RequestError::UniverseGoneWhileExecutingRequest => "The universe disconnected from the proxy while executing your command.",
            RequestError::GalaxyDoesNotExist => "The specified galaxy doesn't exist",
            RequestError::NonEditableUnit => "The unit you specified to update can't be altered: Maybe it's a player unit, shot or some other active and dynamically generated unit.",
            RequestError::AmbiguousXmlData(e) => e.description(),
            RequestError::ServerException(msg) => msg.as_str(),
            RequestError::UnknownErrorCode(_, msg) => msg.as_str(),
            RequestError::InternalIoError(e) => e.description()
        })
    }
}

impl From<std::io::Error> for RequestError {
    fn from(e: std::io::Error) -> Self {
        RequestError::InternalIoError(e)
    }
}

impl From<IllegalName> for RequestError {
    fn from(e: IllegalName) -> Self {
        RequestError::IllegalName(e)
    }
}

impl From<AmbiguousXmlData> for RequestError {
    fn from(e: AmbiguousXmlData) -> Self {
        RequestError::AmbiguousXmlData(e)
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}: {}", self.general(), self.message())
    }
}

impl Error for RequestError {}

#[derive(Debug)]
pub struct IllegalName;

impl Display for IllegalName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "The given name is illegal")
    }
}

impl Error for IllegalName {}

#[derive(Debug)]
pub struct AmbiguousXmlData;

impl Display for AmbiguousXmlData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "The XML specification is invalid. Please check the syntax and/or content."
        )
    }
}

impl Error for AmbiguousXmlData {}

#[derive(Debug)]
pub struct PermissionDenied(Privileges);

impl Display for PermissionDenied {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        let required = self
            .0
            .list()
            .map(|p| format!("{:?}", p))
            .collect::<Vec<String>>();

        if required.is_empty() {
            write!(f, "Access denied. However, it seems like you don't need any privileges for what you tried to do?!")
        } else if required.len() == 1 {
            write!(
                f,
                "Access denied. You require the \"{}\" privilege,",
                required[0]
            )
        } else {
            write!(
                f,
                "Access denied. You require the following privileges for this call: \"{}\" and \"{}\".",
                required.join("\", \""),
                required[required.len() -1 ]
            )
        }
    }
}

impl Error for PermissionDenied {}
