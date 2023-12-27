use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The plugin can send a setImage event to the Stream Deck application to dynamically change the image displayed by an instance of an action.
/// Deck window.
///
/// Examples of base64 encoded images:
/// ```json
/// "data:image/png;base64,iVBORw0KGgoA..."
/// "data:image/jpg;base64,/9j/4AAQSkZJ..."
/// "data:image/bmp;base64,/9j/Qk32PAAA..."
/// ```
/// Starting with Stream Deck 4.5.1, this API accepts svg images. Here is an example with an svg image:
/// ```json
/// {
///     "event": "setImage",
///     "context": context,
///     "payload": {
///         "image": "data:image/svg+xml;charset=utf8,<svg height=\"100\" width=\"100\"><circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" stroke-width=\"3\" fill=\"red\" /></svg>",
///         "target": 2
///     }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetImage {
    /// A value to Identify the instance's action you want to modify.
    pub context: Value,
    /// A JSON object
    pub payload: SetImagePayload,
}
impl SendableEvent for SetImage {
    const EVENT: &'static str = "setImage";
}

/// Payload for [`SetImage`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetImagePayload {
    /// The image to display encoded in base64 with the image format declared in the mime type (PNG, JPEG, BMP, ...).
    /// svg is also supported. If not provided, the image is reset to the default image from the manifest.
    pub image: String,
    /// Specify if you want to display the title on the hardware and software (0), only on the
    /// hardware (1), or only on the software (2). Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<u8>,
    /// A 0-based integer value representing the state of an action with multiple states.
    /// If not specified, the title is set to all states.
    pub state: Option<u32>,
}
