use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// When the user releases a key, the plugin will receive the `keyUp` event.
pub type KeyUp = ActionEvent<KeyUpPayload>;

/// Payload for [`KeyUp`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyUpPayload {
    /// This JSON object contains data that you can set and are stored persistently.
    pub settings: Map<String, Value>,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// Only set when the action has multiple states defined in its manifest.json. The 0-based value
    /// contains the current state of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<u32>,
    /// Only set when the action is triggered with a specific value from a Multi-Action. For
    /// example, if the user sets the Game Capture Record action to be disabled in a Multi-Action,
    /// you would see the value 1. 0 and 1 are valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_desired_state: Option<u8>,
    /// Boolean indicating if the action is inside a Multi-Action.
    pub is_in_multi_action: bool,
}
