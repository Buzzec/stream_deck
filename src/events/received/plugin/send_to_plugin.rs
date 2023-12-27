use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The plugin will receive a `sendToPlugin` event when the Property Inspector sends a
/// `sendToPlugin` event
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendToPlugin {
    /// The action's unique identifier. If your plugin supports multiple actions, you should use
    /// this value to see which action was triggered.
    pub action: String,
    /// sendToPlugin
    pub event: String,
    /// A value identifying the instance's action. You will need to pass this opaque value to
    /// several APIs like the setTitle API.
    pub context: Value,
    /// Custom value sent.
    pub payload: Map<String, Value>,
}
