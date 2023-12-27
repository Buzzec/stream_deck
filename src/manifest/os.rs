use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OS {
    pub platform: Platform,
    pub minimum_version: String,
}
impl OS {
    pub fn default_windows() -> Self {
        Self {
            platform: Platform::Windows,
            minimum_version: "10".to_string(),
        }
    }

    pub fn default_mac() -> Self {
        Self {
            platform: Platform::Mac,
            minimum_version: "10.11".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Platform {
    Mac,
    Windows,
}
