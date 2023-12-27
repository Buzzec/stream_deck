use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When the user rotates the encoder, the plugin will receive the dialRotate event (SD+).
pub type DialRotate = ActionEvent<DialRotatePayload>;

/// Payload for [`DialRotate`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DialRotatePayload {
    /// This JSON object contains data that you can set and are stored persistently.
    pub settings: Value,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// The integer which holds the number of "ticks" on encoder rotation. Positive values are for
    /// clockwise rotation, negative values are for counterclockwise rotation, zero value is never
    /// happen
    pub ticks: i32,
    /// Boolean which is true on rotation when encoder pressed
    pub pressed: bool,
}
