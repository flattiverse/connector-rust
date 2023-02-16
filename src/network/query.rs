use crate::controllable::ControllableId;
use crate::players::PlayerId;
use crate::region::GameRegionId;
use crate::team::TeamId;
use crate::universe::UniverseId;
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryId(String);

impl QueryId {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Query {
    pub id: QueryId,
    #[serde(flatten)]
    pub command: QueryCommand,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "command")]
pub enum QueryCommand {
    #[serde(rename = "whoami")]
    WhoAmI,
    #[serde(rename = "controllableContinue")]
    ContinueControllable { controllable: ControllableId },
    #[serde(rename = "controllableKill")]
    KillControllable { controllable: ControllableId },
    #[serde(rename = "controllableNew")]
    NewControllable {
        controllable: ControllableId,
        name: String,
    },
    #[serde(rename = "controllableNozzle")]
    SetControllableNozzle {
        controllable: ControllableId,
        nozzle: f64,
    },
    #[serde(rename = "controllableThruster")]
    SetControllableThruster {
        controllable: ControllableId,
        thrust: f64,
    },
    #[serde(rename = "controllableScanner")]
    SetControllableScanner {
        controllable: ControllableId,
        direction: f64,
        length: f64,
        width: f64,
        enabled: bool,
    },
    #[serde(rename = "controllableShoot")]
    ControllableShoot {
        direction: Vector,
        load: f64,
        damage: f64,
        time: u16,
    },
    #[serde(rename = "unitSet")]
    SetUnit { universe: UniverseId, unit: String },
    #[serde(rename = "unitGet")]
    GetUnit { universe: UniverseId, unit: String },
    #[serde(rename = "unitRemove")]
    RemoveUnit { universe: UniverseId, unit: String },
    #[serde(rename = "regionList")]
    ListRegion { universe: UniverseId },
    #[serde(rename = "regionSetUnnamed")]
    SetRegionUnnamed {
        universe: UniverseId,
        #[serde(rename = "regionId")]
        region: GameRegionId,
        teams: i32,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        #[serde(rename = "startLocation")]
        start_location: bool,
        #[serde(rename = "safeZone")]
        safe_zone: bool,
        #[serde(rename = "slowRestore")]
        slow_restore: bool,
    },
    #[serde(rename = "regionSet")]
    SetRegion {
        universe: UniverseId,
        #[serde(rename = "regionId")]
        region: GameRegionId,
        teams: i32,
        name: String,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        #[serde(rename = "startLocation")]
        start_location: bool,
        #[serde(rename = "safeZone")]
        safe_zone: bool,
        #[serde(rename = "slowRestore")]
        slow_restore: bool,
    },
    #[serde(rename = "regionRemove")]
    RemoveRegion {
        universe: UniverseId,
        #[serde(rename = "regionId")]
        region: GameRegionId,
    },
    #[serde(rename = "chatTeamcast")]
    ChatTeam { team: TeamId, message: String },
    #[serde(rename = "chatUnicast")]
    ChatPlayer { player: PlayerId, message: String },
    #[serde(rename = "chatMulticast")]
    ChatUniverseGroup { message: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum QueryResponse {
    Integer(i32),
    Double(f64),
    String(String),
    Empty,
}

impl QueryResponse {
    #[inline]
    pub fn get_double(&self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn get_integer(&self) -> Option<i32> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn get_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_str()),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum QueryError {
    #[error("[0x02] The specified unit doesn't exist.")]
    UnitDoesNotExist,
    #[error("[0x03] The specified Player doesn't exist.")]
    PlayerDoesNotExist,
    #[error("[0x05] The command you tried to access can't be access with your player kind. (Tried to access admin commands as player or vice versa, etc.)")]
    InvalidPlayerKind,
    #[error("[0x10] You exceeded the amount of allowed ships per player for this UniverseGroup.")]
    ExceededShipsPerPlayer,
    #[error("[0x11] You exceeded the amount of non built units for this UniverseGroup.")]
    ExceededNonBuiltUnits,
    #[error("[0x12] You exceeded the amount of allowed ships per team for this UniverseGroup.")]
    ExceededShipsPerTeam,
    #[error("[0x20] You need to die before you can use Continue().")]
    MustBeDeadForThisAction,
    #[error("[0x21] All start locations are currently overcrowded. Try again, later.")]
    SpawnLocationsOvercrowded,
    #[error("[0x22] You need to Continue() before doing this.")]
    MustBeAliveForThisAction,
    #[error("[0x23] This system doesn't support those values.")]
    UnsupportedValues,
    #[error(
        "[0xA0] Your JSON definition is missing some mandatory base value like name or radius."
    )]
    MissingBaseMandatoryValues,
    #[error("[0xA1] Your JSON definition is missing some mandatory non-base value.")]
    MissingExtendedMandatoryValues,
    #[error("[0xA2] At least one required JSON property doesn't exist.")]
    PropertyDoesNotExist,
    #[error("[0xA3] At least one required JSON property doesn't have the required kind.")]
    PropertyHasWrongKind,
    #[error("[0xA4] At least one required JSON property has an invalid value.")]
    PropertyHasInvalidValue,
    #[error("[0xA8] \"kind\" is missing.")]
    MissingKindValue,
    #[error("[0xA9] \"kind\" couldn't be resolved to a valid unit kind. (Can't resolve \"playerUnit\", \"shot\" or \"explosion\".)")]
    InvalidKindValue,
    #[error(
        "[0xAA] Can't replace a non editable unit like \"playerunit\", \"shot\" or \"explosion\"."
    )]
    UnitCannotBeEdited,
    #[error("[0xAB] Tried to access an upgradepath or system which does not exist.")]
    UpgradePathOrSystemDoesNoteExist,
    #[error(
        "[0xB0] The parameter you did specify was either null or didn't contain proper content."
    )]
    ParameterEitherNullOrInvalidContent,
    #[error("[0xB1] The parameter you did specify exceeded the maximum size.")]
    ParameterExceededMaximumSize,
    #[error("[0xB2] You dishonored the naming criteria of units. Only allowed characters are: space, dot, minus, underscore, a-z, A-Z and unicode characters between 192-214, 216-246 and 248-687.")]
    InvalidUnitName,
    #[error("[0xB3] The requested slot is not free.")]
    SlotAlreadyInUse,
    #[error("[0xB4] The name is already in use, either by another player's ship or by an unit in one of the universes.")]
    NameAlreadyInUse,
    #[error("[0xB5] Messages must be between 1 and 256 characters long and can't contain control characters.")]
    InvalidMessage,
    #[error("[0xB6] doubles can't be NaN or positive/negative infinity.")]
    InvalidFloatingPointValue,
    #[error("[0xB7] You triggered the flood control. Please wait a little bit before sending the next message.")]
    FloodControl,
    #[error("[0xC0] We couldn't connect to the specified endpoint. Maybe a typo?")]
    EndpointUnreachable,
    #[error("[0xC1] The specified auth key has been declined.")]
    AuthKeyDeclined,
    #[error("[0xC2] You are currently online. (You can only logon once with each account.) Please note: If your game just crashed: In such a case your account and ships are still lingering around so you have to wait round about 30 seconds before retrying.")]
    AlreadyOnline,
    #[error("[0xC3] This UniverseGroup seems to be full. Try another one.")]
    UniverseGroupIsFull,
    #[error("[0xC4] The specified team doesn't exist.")]
    TeamNotFound,
    #[error("[0xC5] Your connector is outdated and incompatible: Please update the connector.")]
    ConnectorIncompatible,
    #[error("[0xC6] The universe group is currently offline.")]
    UniverseGroupIsCurrentlyOffline,
    #[error("[0xCF] Something went wrong while connecting but we don't know what and don't have any more infomration. You may try your luck with the inner exception.")]
    ConnectionError,
    #[error("[0xF0] The web socket got terminated while waiting for the completion of the command. This usually indicates that you have a network connectivity issue somewhere between you and the server or that the server has been rebooted to reload some level settings.")]
    WebSocketTerminated,
    #[error("[0xFF] Some fatal error occurred and the server closed the connection. You may give this information to a flattiverse admin because this shouldn't happen")]
    AB,

    #[error("[0x{0:02x}] Unknown GameException code 0x{0:02x} received.")]
    Other(i32),
    #[error("Unable to receive a response because the connection to the server is no more")]
    ConnectionGone,

    #[error("Failed to parse the response: {0}")]
    ResponseMalformed(String),
}

impl From<i32> for QueryError {
    fn from(value: i32) -> Self {
        match value {
            0x02 => Self::UnitDoesNotExist,
            0x03 => Self::PlayerDoesNotExist,
            0x05 => Self::InvalidPlayerKind,
            0x10 => Self::ExceededShipsPerPlayer,
            0x11 => Self::ExceededNonBuiltUnits,
            0x12 => Self::ExceededShipsPerTeam,
            0x20 => Self::MustBeDeadForThisAction,
            0x21 => Self::SpawnLocationsOvercrowded,
            0x22 => Self::MustBeAliveForThisAction,
            0x23 => Self::UnsupportedValues,
            0xA0 => Self::MissingBaseMandatoryValues,
            0xA1 => Self::MissingExtendedMandatoryValues,
            0xA2 => Self::PropertyDoesNotExist,
            0xA3 => Self::PropertyHasWrongKind,
            0xA4 => Self::PropertyHasInvalidValue,
            0xA8 => Self::MissingKindValue,
            0xA9 => Self::InvalidKindValue,
            0xAA => Self::UnitCannotBeEdited,
            0xAB => Self::UpgradePathOrSystemDoesNoteExist,
            0xB0 => Self::ParameterEitherNullOrInvalidContent,
            0xB1 => Self::ParameterExceededMaximumSize,
            0xB2 => Self::InvalidUnitName,
            0xB3 => Self::SlotAlreadyInUse,
            0xB4 => Self::NameAlreadyInUse,
            0xB5 => Self::InvalidMessage,
            0xB6 => Self::InvalidFloatingPointValue,
            0xB7 => Self::FloodControl,
            0xC0 => Self::EndpointUnreachable,
            0xC1 => Self::AuthKeyDeclined,
            0xC2 => Self::AlreadyOnline,
            0xC3 => Self::UniverseGroupIsFull,
            0xC4 => Self::TeamNotFound,
            0xC5 => Self::ConnectorIncompatible,
            0xCF => Self::ConnectionError,
            0xF0 => Self::WebSocketTerminated,
            0xFF => Self::AB,
            _ => Self::Other(value),
        }
    }
}

pub type QueryResult = Result<QueryResponse, QueryError>;

#[derive(Default)]
pub struct QueryKeeper {
    queries: Vec<(QueryId, Sender<QueryResult>)>,
}

impl QueryKeeper {
    const ALLOWED_LEN: usize = 2;
    const ALLOWED_CHARS: &'static [char] = &[
        '.', '-', '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
        'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
        'Z',
    ];

    pub fn register_new_for(&mut self, target: Sender<QueryResult>) -> Option<QueryId> {
        let id = self.unused_id()?;
        self.queries.push((id.clone(), target));
        Some(id)
    }

    fn unused_id(&self) -> Option<QueryId> {
        let mut id = String::with_capacity(Self::ALLOWED_LEN);

        for mut counter in 0..Self::ALLOWED_CHARS.len().pow(Self::ALLOWED_LEN as u32) {
            // fill the buffer
            for _ in 0..Self::ALLOWED_LEN {
                id.push(Self::ALLOWED_CHARS[counter % Self::ALLOWED_CHARS.len()]);
                counter /= Self::ALLOWED_CHARS.len();
            }

            if !self.contains(&id) {
                return Some(QueryId(id));
            } else {
                id.clear();
            }
        }
        None
    }

    fn contains(&self, id: &str) -> bool {
        for i in 0..self.queries.len() {
            if self.queries[i].0 .0 == id {
                return true;
            }
        }
        false
    }

    pub fn answer(&mut self, id: &QueryId, result: QueryResult) -> Option<QueryResult> {
        match self.unblock(id) {
            Some(target) => target.send(result).err(),
            None => Some(result),
        }
    }

    pub fn unblock(&mut self, id: &QueryId) -> Option<Sender<QueryResult>> {
        for i in 0..self.queries.len() {
            if self.queries[i].0 == *id {
                return Some(self.queries.swap_remove(i).1);
            }
        }
        None
    }
}
