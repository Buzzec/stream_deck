use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Event received after calling the `getSettings` API to retrieve the persistent data stored for the action.
pub type DidReceiveSettings = ActionEvent<DidReceiveSettingsPayload>;

/// Payload for [`DidReceiveSettings`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveSettingsPayload {
    /// This JSON object contains persistently stored data.
    pub settings: Map<String, Value>,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// Only set when the action has multiple states defined in its manifest.json. The 0-based value contains the
    /// current state of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<u8>,
    /// Boolean indicating if the action is inside a Multi-Action.
    pub is_in_multi_action: bool,
}
