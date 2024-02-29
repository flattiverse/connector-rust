use crate::hierarchy::Galaxy;
use crate::mission_selection::GalaxyInfo;
use crate::network::ConnectError;
use crate::GameError;
use std::sync::Arc;

#[derive(Debug)]
pub struct Universe(Vec<GalaxyInfo>);

impl Universe {
    pub const URI_BASE: &'static str = "www.flattiverse.com";

    #[cfg(not(feature = "dev-environment"))]
    pub const URI_GALAXIES_ALL: &'static str = "https://www.flattiverse.com/api/galaxies/all";

    #[cfg(feature = "dev-environment")]
    pub const URI_GALAXIES_ALL: &'static str = "http://localhost:8080/api/galaxies/all";

    pub async fn fetch() -> Result<Self, ConnectError> {
        let text = crate::network::get_text(Self::URI_GALAXIES_ALL).await?;
        let galaxies = serde_json::from_str::<Vec<GalaxyInfo>>(&text)
            .map_err(|e| ConnectError::Unknown(e.to_string()))?;
        Ok(Universe(galaxies))
    }

    /// Don't use this method - only if you a _really_ sure what you are doing.
    pub async fn manual_join(uri: &str, auth: &str, team: u8) -> Result<Arc<Galaxy>, GameError> {
        let galaxy = Galaxy::join(uri, auth, team).await?;
        galaxy.wait_login_completed().await?;
        Ok(galaxy)
    }

    #[inline]
    pub fn iter_galaxies(&self) -> impl Iterator<Item = &GalaxyInfo> {
        self.0.iter()
    }
}
