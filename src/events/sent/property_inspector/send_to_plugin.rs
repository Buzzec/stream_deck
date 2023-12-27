use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The plugin can send a payload to the Property Inspector using the `sendToPropertyInspector` event.
///
/// The Property Inspector will receive asynchronously an event `sendToPropertyInspector` containing the payload.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SendToPlugin {
    /// The action's unique identifier. If your plugin supports multiple actions, you should use
    /// this value to find out which action was triggered.
    pub action: String,
    /// A value to Identify the Property Inspector. This value is received by the Property Inspector
    /// as parameter of the `connectElgatoStreamDeckSocket` function.
    pub context: Value,
    /// A JSON object that will be received by the plugin.
    pub payload: Map<String, Value>,
}
impl SendableEvent for SendToPlugin {
    const EVENT: &'static str = "sendToPlugin";
}
