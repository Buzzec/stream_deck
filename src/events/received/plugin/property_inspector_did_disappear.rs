use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The plugin will receive a `propertyInspectorDidDisappear` event when the Property Inspector disappears
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PropertyInspectorDidDisappear {
    /// The action's unique identifier.
    pub action: String,
    /// propertyInspectorDidDisappear
    pub event: String,
    /// A value identifying the instance's action. You will need to pass this opaque value to
    /// several APIs like the setTitle API.
    pub context: Value,
    /// A value to identify the device.
    pub device: Value,
}
