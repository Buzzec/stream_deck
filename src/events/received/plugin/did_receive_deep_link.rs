use serde::{Deserialize, Serialize};

/// *Available from Stream Deck 6.5 onwards.*
///
/// Occurs when Stream Deck receives a deep-link message intended for the plugin. The message is
/// re-routed to the plugin, and provided as part of the payload. One-way deep-link message can be
/// routed to the plugin using the URL format:
/// ```plain
/// streamdeck://plugins/message/<PLUGIN_UUID>/{MESSAGE}
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveDeepLink {
    /// didReceiveDeepLink
    pub event: String,
    /// A JSON object
    pub payload: DidReceiveDeepLinkPayload,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveDeepLinkPayload {
    /// The deep-link URL, with the prefix omitted. For example the URL
    /// `streamdeck://plugins/message/com.elgato.test/hello-world` would result in a url of
    /// `hello-world`
    pub url: String,
}
