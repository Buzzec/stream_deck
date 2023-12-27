use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The plugin can send a showAlert event to the Stream Deck application to temporarily show an alert icon on the image
/// displayed by an instance of an action.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ShowAlert {
    /// A value to identify the instance's action.
    pub context: Value,
}
impl SendableEvent for ShowAlert {
    const EVENT: &'static str = "showAlert";
}
