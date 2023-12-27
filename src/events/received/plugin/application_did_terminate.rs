use serde::{Deserialize, Serialize};

/// A plugin can request in its manifest.json to be notified when some applications are launched or terminated.
/// The manifest.json should contain an ApplicationsToMonitor object specifying the list of application identifiers to
/// monitor. On macOS, the application bundle identifier is used while the exe filename is used on Windows. For example,
/// the Apple Mail sample plugin uses the following:
/// ```json
/// "ApplicationsToMonitor": {
///     "mac": [
///         "com.apple.mail"
///     ]
/// ]
/// ```
/// When a monitored application is terminated, the plugin will receive the `applicationDidTerminate` event.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationDidTerminate {
    /// applicationDidTerminate
    pub event: String,
    /// A JSON object
    pub payload: ApplicationDidLaunchPayload,
}

/// Payload for [`ApplicationDidTerminate`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationDidLaunchPayload {
    /// The identifier of the application that has been launched.
    pub application: String,
}
