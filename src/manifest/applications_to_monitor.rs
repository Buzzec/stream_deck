use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApplicationsToMonitor {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mac: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub windows: Vec<String>,
}
