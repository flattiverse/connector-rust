#[derive(Debug, Clone, Deserialize)]
pub struct PlayerInfo {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "hasAvatar")]
    pub has_avatar: bool,
    #[serde(rename = "team")]
    pub team: i32,
    #[serde(rename = "kills")]
    pub kills: i32,
    #[serde(rename = "deaths")]
    pub deaths: i32,
    #[serde(rename = "collisions")]
    pub collisions: i32,
    #[serde(rename = "sessionKills")]
    pub session_kills: i32,
    #[serde(rename = "sessionDeaths")]
    pub session_deaths: i32,
    #[serde(rename = "sessionCollisions")]
    pub session_collisions: i32,
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "pvPScore")]
    pub pvp_score: i32,
    #[serde(rename = "datePlayedStart")]
    pub date_played_start: i32,
    #[serde(rename = "datePlayedEnd")]
    pub date_played_end: i32,
}
