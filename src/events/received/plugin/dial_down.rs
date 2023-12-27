use crate::common::Controller;
use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When the user presses the encoder down, the plugin will receive the dialDown event (SD+).
pub type DialDown = ActionEvent<DialDownPayload>;

/// Payload for [`DialDown`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DialDownPayload {
    /// This JSON object contains data that you can set and are stored persistently.
    pub settings: Value,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// Encoder
    pub controller: Controller,
}
