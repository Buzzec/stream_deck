use crate::common::Controller;
use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When an instance of an action ceases to be displayed on Stream Deck, for example, when switching profiles or folders, the plugin will receive a willDisappear event. You will see such an event when:
/// - the user switches between profiles
/// - the user deletes an action
pub type WillDisappear = ActionEvent<WillDisappearPayload>;

/// Payload for [`WillDisappear`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WillDisappearPayload {
    /// This JSON object contains data that you can set and are stored persistently.
    pub settings: Value,
    /// The coordinates of the action triggered.
    pub coordinates: Coordinates,
    /// Defines the controller type the action is applicable to. Keypad refers to a standard action
    /// on a Stream Deck device, e.g. 1 of the 15 buttons on the Stream Deck MK.2, or a pedal on the
    /// Stream Deck Pedal, etc., whereas an Encoder refers to a dial / touchscreen on the
    /// Stream Deck+.
    pub controller: Controller,
    /// Only set when the action has multiple states defined in its manifest.json. The 0-based value
    /// contains the current state of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<u32>,
    /// Boolean indicating if the action is inside a Multi-Action.
    pub is_in_multi_action: bool,
}
