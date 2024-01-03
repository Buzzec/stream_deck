use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::common::Context;

pub mod common;
pub mod plugin;
pub mod property_inspector;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ActionEvent<P> {
    /// The action's unique identifier. If your plugin supports multiple actions, you should use
    /// this value to see which action was triggered.
    pub action: String,
    /// Event identifier
    pub event: String,
    /// A value identifying the instance's action. You will need to pass this opaque value to
    /// several APIs like the setTitle API.
    pub context: Context,
    /// A value to identify the device.
    pub device: Value,
    /// A JSON object
    pub payload: P,
}
