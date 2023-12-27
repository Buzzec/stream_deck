use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The plugin can send a `setFeedback` event to the Stream Deck application to dynamically change properties of items on
/// the Stream Deck + touch display layout.
///
/// ## Examples
/// If your plugin uses the Indicator Layout, you can set the value of the title item by using the title key in the
/// `setFeedback` payload.
/// ```json
/// {
///     "title": "Example Title"
/// }
/// ```
/// You can also set multiple properties at once with a single payload. The Indicator Layout has items with keys title,
/// icon, value, and indicator.
/// ```json
/// {
///     "title": "Example Title",
///     "icon": "./path-to-icon/icon.svg",
///     "value": "55",
///     "indicator": { "value": 50, "enabled": true }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeedback {
    /// A value to Identify the instance's action you want to modify.
    pub context: Value,
    /// A JSON object
    /// - Keys: The key is a name of the element in layout to be changed with given `value`.
    /// - Values: The value to be set in key named layout element.
    pub payload: Map<String, Value>,
}
impl SendableEvent for SetFeedback {
    const EVENT: &'static str = "setFeedback";
}
