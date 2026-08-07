use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReleaseStatus {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub checked_at: Option<String>,
    pub release_url: Option<String>,
}
