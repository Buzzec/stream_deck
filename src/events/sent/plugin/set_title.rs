use crate::common::Context;
use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};

/// The plugin can send a setTitle event to the Stream Deck application to dynamically change the
/// title displayed by an instance of an action.
///
/// Note: Show the title on your hardware or software using the Show Title checkbox in the Stream
/// Deck window.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetTitle {
    /// A value to Identify the instance's action you want to modify.
    pub context: Context,
    /// A JSON object
    pub payload: SetTitlePayload,
}
impl SendableEvent for SetTitle {
    const EVENT: &'static str = "setTitle";
}

/// Payload for [`SetTitle`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetTitlePayload {
    /// The title to display. If there is no title parameter, the title is reset to the title set by
    /// the user.
    pub title: String,
    /// Specify if you want to display the title on the hardware and software (0), only on the
    /// hardware (1), or only on the software (2). Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<u8>,
    /// A 0-based integer value representing the state of an action with multiple states.
    /// If not specified, the title is set to all states.
    pub state: Option<u32>,
}
