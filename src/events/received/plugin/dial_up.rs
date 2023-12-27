use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When the user releases a pressed encoder, the plugin will receive the `dialUp` event (SD+).
pub type DialUp = ActionEvent<DialUpPayload>;

/// Payload for [`DialUp`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DialUpPayload {
    /// This JSON object contains data that you can set and are stored persistently.
    pub settings: Value,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// Encoder
    pub controller: String,
}
