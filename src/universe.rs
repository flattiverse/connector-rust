use crate::hierarchy::Galaxy;
use crate::GameError;

pub struct Universe;

impl Universe {
    pub async fn join(uri: &str, auth: &str, team: u8) -> Result<Galaxy, GameError> {
        let mut galaxy = Galaxy::join(uri, auth, team).await?;
        galaxy.wait_login_completed().await?;
        Ok(galaxy)
    }
}
