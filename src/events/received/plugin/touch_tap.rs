use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When the user touches the display, the plugin will receive the touchTap event (SD+).
pub type TouchTap = ActionEvent<TouchTapPayload>;

/// Payload for [`TouchTap`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TouchTapPayload {
    /// This JSON object contains data that you can set and are stored persistently.
    pub settings: Value,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// The array which holds (x, y) coordinates as a position of tap inside of LCD slot associated
    /// with action.
    pub tap_pos: [u32; 2],
    /// Boolean which is `true` when long tap happened
    pub hold: bool,
}
