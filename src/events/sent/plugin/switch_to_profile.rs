use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The plugin can tell the Stream Deck application to switch to one of his preconfigured read-only profile using the
/// `switchToProfile` event.
///
/// Note that a plugin can only switch to read-only profiles declared in its manifest.json file. If the profile field is
/// missing or empty, the Stream Deck application will switch to the previously selected profile.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SwitchToProfile {
    /// A value to Identify the plugin. This value should be set to the PluginUUID received during the registration
    /// procedure.
    pub context: Value,
    /// A value to identify the device.
    pub device: Value,
    /// A JSON object
    pub payload: SwitchToProfilePayload,
}
impl SendableEvent for SwitchToProfile {
    const EVENT: &'static str = "switchToProfile";
}

/// Payload for [`SwitchToProfile`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SwitchToProfilePayload {
    /// The name of the profile to switch to. The name should be identical to the name provided in the manifest.json
    /// file.
    pub profile: String,
    /// Page to show when switching to the profile; indexed from 0.
    pub page: u32,
}
