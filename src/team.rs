use serde_derive::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct TeamId(pub(crate) usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    /// The id of the team.
    pub id: TeamId,
    /// The name of the team.
    pub name: String,
    /// The red value of the team's color.
    pub r: f64,
    /// The green value of the team's color.
    pub g: f64,
    /// The blue value of the team's color.
    pub b: f64,
}

impl Team {
    /// The team's color in a three-dimensional color array (RGB)
    pub fn rgb(&self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }
}
