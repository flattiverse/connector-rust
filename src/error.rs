use crate::network::connection_handle::SendQueryError;
use crate::network::query::QueryError;

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("You need to die before you can continue")]
    ControllableMustBeDeadToContinue,
    #[error("You have to live (or call continue()) before you can be suicide")]
    ControllableMustLiveToBeKilled,

    #[error("The length of the name is invalid (too short or too long)")]
    NameLengthInvalid,
    #[error("The name starts or ends with invalid whitespace characters")]
    NameNotTrimmed,
    #[error("The name contains invalid characters")]
    NameContainsInvalidCharacters,
    #[error("You exceeded the amount of allowed ships per player for this UniverseGroup")]
    ExceededShipsPerPlayer,
    #[error("You exceeded the amount of on built units for this UniverseGroup")]
    ExceededNonBuiltUnits,
    #[error("You exceeded the amount of allowed ships per team for this UniverseGroup")]
    ExceededShipsPerTeam,
    #[error("The definition for the unit is empty")]
    UnitDefinitionEmpty,
    #[error("The definition for the unit is too long")]
    UnitDefinitionTooLong,
    #[error("The length of the message is invalid (too short or too long)")]
    MessageLengthInvalid,
    #[error("The messages starts or ends with invalid whitespace characters")]
    MessageNotTrimmed,
    #[error("The message contains invalid characters")]
    MessageContainsInvalidCharacters,

    // -------- from impls
    #[error("Unable to send your request to the server")]
    SendQueryError(#[from] SendQueryError),
    #[error("Your request was denied, reason {0:?}")]
    QueryError(#[from] QueryError),
}

impl GameError {
    pub fn checked_name(name: String) -> Result<String, GameError> {
        if name.is_empty() || name.len() < 2 || name.len() > 32 {
            Err(GameError::NameLengthInvalid)
        } else if name.starts_with(' ') || name.ends_with(' ') {
            Err(GameError::NameNotTrimmed)
        } else if name.chars().all(|char| match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            ' ' | '.' | '_' | '-' => true,
            c if matches!(c as u32, 192..=214 | 216..=246 | 248..=687 ) => true,
            _ => false,
        }) {
            Ok(name)
        } else {
            Err(GameError::NameContainsInvalidCharacters)
        }
    }

    pub fn checked_message(message: String) -> Result<String, GameError> {
        if message.is_empty() || message.len() > 256 {
            Err(GameError::MessageLengthInvalid)
        } else if message.starts_with(' ') || message.ends_with(' ') {
            Err(GameError::MessageNotTrimmed)
        } else if message.chars().all(|char| match char {
            ' '..='~' => true,
            c if matches!(c as u32, 192..=214 | 216..=246 | 248..=687 ) => true,
            '€' | '‚' | '„' | '…' | '‰' | '‹' | '›' | '™' | '•' | '¢' | '£' | '¡' | '¤' | '¥'
            | '©' | '®' | '±' | '²' | '³' | 'µ' | '¿' | '«' | '»' => true,
            _ => false,
        }) {
            Ok(message)
        } else {
            Err(GameError::MessageContainsInvalidCharacters)
        }
    }
}
