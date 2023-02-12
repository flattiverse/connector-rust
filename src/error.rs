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
        } else {
            for char in name.chars() {
                match char {
                    'a'..='z' | 'A'..='Z' | '0'..='9' => continue,
                    ' ' | '.' | '_' | '-' => continue,
                    c if matches!(c as u32, 192..=214 | 216..=246 | 248..=687 ) => continue,
                    _ => return Err(GameError::NameContainsInvalidCharacters),
                }
            }
            Ok(name)
        }
    }
}
