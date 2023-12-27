use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};

/// The plugin and Property Inspector can request the persistent global data using the
/// `getGlobalSettings` event
///
/// The plugin or Property Inspector will receive asynchronously an event `didReceiveGlobalSettings`
/// containing the global settings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetGlobalSettings {
    /// A value to Identify the plugin (inPluginUUID) or the Property Inspector (inPropertyInspectorUUID). This value is 
    /// received during the Registration procedure.
    pub context: String,
}
impl SendableEvent for GetGlobalSettings {
    const EVENT: &'static str = "getGlobalSettings";
}
