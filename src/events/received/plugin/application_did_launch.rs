use serde::{Deserialize, Serialize};

/// A plugin can request in its manifest.json to be notified when some applications are launched or
/// terminated. The manifest.json should contain an ApplicationsToMonitor object specifying the list
/// of application identifiers to monitor. On macOS, the application bundle identifier is used while
/// the exe filename is used on Windows. For example, the Apple Mail sample plugin uses the
/// following:
/// ```json
/// "ApplicationsToMonitor": {
///     "mac": [
///         "com.apple.mail"
///     ]
/// ]
/// ```
/// When a monitored application is launched, the plugin will receive the `applicationDidLaunch` event.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationDidLaunch {
    /// applicationDidLaunch
    pub event: String,
    /// A JSON object
    pub payload: ApplicationDidLaunchPayload,
}

/// Payload for [`ApplicationDidLaunch`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationDidLaunchPayload {
    /// The identifier of the application that has been launched.
    pub application: String,
}
