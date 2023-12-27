use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This function can be used by a plugin to dynamically change the state of an action supporting
/// multiple states.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetState {
    /// A value to identify the instance's action.
    pub context: Value,
    /// A JSON object
    pub payload: SetStatePayload,
}
impl SendableEvent for SetState {
    const EVENT: &'static str = "setState";
}

/// Payload for [`SetState`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SetStatePayload {
    /// A 0-based integer value representing the state requested.
    pub state: u32,
}
