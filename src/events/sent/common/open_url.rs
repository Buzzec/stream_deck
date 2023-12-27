use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};

/// The plugin and Property Inspector can tell the Stream Deck application to open an URL in the
/// default browser using the `openUrl` event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrl {
    /// A JSON object
    pub payload: OpenUrlPayload,
}
impl SendableEvent for OpenUrl {
    const EVENT: &'static str = "openUrl";
}

/// Payload for [`OpenUrl`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OpenUrlPayload {
    /// A URL to open in the default browser.
    pub url: String,
}
