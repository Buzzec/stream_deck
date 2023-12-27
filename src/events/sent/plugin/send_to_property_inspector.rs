use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The plugin can send a payload to the Property Inspector using the `sendToPropertyInspector` event.
///
/// The Property Inspector will receive asynchronously an event `sendToPropertyInspector` containing the payload.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SendToPropertyInspector {
    /// The action's unique identifier.
    pub action: String,
    /// A value to identify the instance's action.
    pub context: Value,
    /// A JSON object that will be received by the Property Inspector.
    pub payload: Map<String, Value>,
}
impl SendableEvent for SendToPropertyInspector {
    const EVENT: &'static str = "sendToPropertyInspector";
}
