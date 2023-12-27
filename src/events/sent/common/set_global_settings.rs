use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The plugin and Property Inspector can save persistent data globally. The data will be saved
/// securely to the Keychain on macOS and the Credential Store on Windows.
/// This API can be used to save tokens that should be available to every action in the plugin.
///
/// This is done using the setGlobalSettings event
///
/// Note that when the plugin uses this API, the Property Inspector will automatically receive a
/// `didReceiveGlobalSettings` callback with the new settings. Similarly, when the Property
/// Inspector uses this API, the plugin will automatically receive a `didReceiveGlobalSettings`
/// callback with the new settings.
///
/// This API has been introduced in Stream Deck 4.1.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetGlobalSettings {
    /// A value to Identify the plugin (inPluginUUID) or the Property Inspector (inPropertyInspectorUUID). This value is
    /// received during the Registration procedure.
    pub context: Value,
    /// A JSON object which is persistently saved for the action's instance.
    pub payload: Map<String, Value>,
}
impl SendableEvent for SetGlobalSettings {
    const EVENT: &'static str = "setGlobalSettings";
}
