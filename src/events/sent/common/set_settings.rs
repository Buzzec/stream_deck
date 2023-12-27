use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The plugin and Property Inspector can save data persistently for the action's instance using the `setSettings` event:
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetSettings {
    /// A value to Identify the instance's action or Property Inspector. This value is received by
    /// the Property Inspector as a parameter of the `connectElgatoStreamDeckSocket` function.
    pub context: Value,
    /// A JSON object which is persistently saved for the action's instance.
    pub payload: Map<String, Value>,
}
impl SendableEvent for SetSettings {
    const EVENT: &'static str = "setSettings";
}
