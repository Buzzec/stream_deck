use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};

/// The plugin and Property Inspector can request the persistent data stored for the action's
/// instance using the getSettings event.
///
/// The plugin or Property Inspector will receive asynchronously an event didReceiveSettings
/// containing the settings for this action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetSettings {
    /// A value to Identify the instance's action or Property Inspector. In the case of the Property Inspector, this
    /// value is received by the Property Inspector as parameter of the `connectElgatoStreamDeckSocket` function.
    pub context: String,
}
impl SendableEvent for GetSettings {
    const EVENT: &'static str = "getSettings";
}
