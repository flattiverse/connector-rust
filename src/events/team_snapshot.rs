use crate::galaxy_hierarchy::{NamedUnit, Team, TeamId};

/// Snapshot of a team state relevant for events.
#[derive(Debug, Clone)]
pub struct TeamSnapshot {
    /// Team id.
    pub id: TeamId,
    /// Team name.
    pub name: String,
    /// Red color component.
    pub red: u8,
    /// Green color component.
    pub green: u8,
    /// Blue color component.
    pub blue: u8,
}

impl From<&Team> for TeamSnapshot {
    fn from(team: &Team) -> Self {
        Self {
            id: team.id,
            name: team.name().to_string(),
            red: team.red(),
            green: team.green(),
            blue: team.blue(),
        }
    }
}
