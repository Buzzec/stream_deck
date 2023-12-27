use serde::{Deserialize, Serialize};

/// When the computer wakes up, the plugin will receive the systemDidWakeUp event.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemDidWakeUp {
    /// systemDidWakeUp
    pub event: String,
}
