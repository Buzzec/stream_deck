use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Event received after calling the `getGlobalSettings` API to retrieve the global persistent data.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveGlobalSettings {
    /// didReceiveGlobalSettings
    pub event: String,
    /// A JSON object
    pub payload: DidReceiveGlobalSettingsPayload,
}

/// Payload for [`DidReceiveGlobalSettings`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveGlobalSettingsPayload {
    /// This JSON object contains persistently stored data.
    pub settings: Map<String, Value>,
}
