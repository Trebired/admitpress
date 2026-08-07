use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackupInfo {
    pub file_name: String,
    pub path: String,
    pub created_at: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub format: String,
    pub version: u32,
    pub created_at: String,
}
