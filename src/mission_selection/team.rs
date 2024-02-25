#[derive(Debug, Clone, Deserialize)]
pub struct TeamInfo {
    #[serde(rename = "id")]
    pub id: u8,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "galaxy")]
    pub galaxy: i32,
    #[serde(rename = "red")]
    pub red: i32,
    #[serde(rename = "green")]
    pub green: i32,
    #[serde(rename = "blue")]
    pub blue: i32,
}
