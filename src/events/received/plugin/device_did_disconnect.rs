use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When a device is unplugged from the computer, the plugin will receive a deviceDidDisconnect event.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDidDisconnect {
    /// deviceDidDisconnect
    pub event: String,
    /// A value to identify the device.
    pub device: Value,
}
