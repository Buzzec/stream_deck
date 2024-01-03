use crate::common::{DeviceSize, DeviceTypeInteger};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When a device is plugged into the computer, the plugin will receive a deviceDidConnect event.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDidConnect {
    /// deviceDidConnect
    pub event: String,
    /// A value to identify the device.
    pub device: Value,
    /// A JSON object containing information about the device.
    pub device_info: DeviceInfo,
}

/// The deviceInfo object contains the following members
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    /// Type of device.
    #[serde(rename = "type")]
    pub device_type: DeviceTypeInteger,
    /// The number of columns and rows of keys that the device owns.
    pub size: DeviceSize,
    /// The name of the device set by the user.
    pub name: String,
}
