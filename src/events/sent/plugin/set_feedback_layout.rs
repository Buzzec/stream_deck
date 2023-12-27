use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The plugin can send a setFeedbackLayout event to the Stream Deck application to dynamically change the current
/// Stream Deck + touch display layout. setFeedbackLayout can use the id of a built-in layout or a relative path to a
/// custom layout JSON file. See Layouts for more information.
///
/// ## Example
/// The default layout is defined in the manifest and use the following payload to switch to a predefined custom layout.
/// ```json
/// "payload": {
///   "layout": "./path-to-layout/layout.json"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeedbackLayout {
    /// A value to Identify the instance's action you want to modify.
    pub context: Value,
    /// A JSON object
    pub payload: SetFeedbackLayoutPayload,
}
impl SendableEvent for SetFeedbackLayout {
    const EVENT: &'static str = "setFeedbackLayout";
}

/// Payload for [`SetFeedbackLayout`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeedbackLayoutPayload {
    /// A predefined layout identifier or the relative path to a json file that contains a custom layout
    pub layout: String,
}
