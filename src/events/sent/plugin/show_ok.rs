use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The plugin can send a showOk event to the Stream Deck application to temporarily show an OK checkmark icon on the
/// image displayed by an instance of an action.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ShowOk {
    /// A value to identify the instance's action.
    pub context: Value,
}
impl SendableEvent for ShowOk {
    const EVENT: &'static str = "showOk";
}
