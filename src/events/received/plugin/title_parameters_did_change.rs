use crate::common::{Color, Controller, FontFamily, FontStyle, TileAlignment};
use crate::events::received::ActionEvent;
use crate::events::Coordinates;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type TitleParametersDidChange = ActionEvent<TitleParametersDidChangePayload>;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TitleParametersDidChangePayload {
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
    /// The new title.
    pub title: String,
    /// A JSON object describing the new title parameters.
    pub title_parameters: TitleParameters,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TitleParameters {
    /// The font family for the title.
    #[serde(with = "optional_string")]
    pub font_family: Option<FontFamily>,
    /// The font size for the title.
    pub font_size: u32,
    /// The font style for the title.
    #[serde(with = "optional_string")]
    pub font_style: Option<FontStyle>,
    /// Boolean indicating an underline under the title.
    pub font_underline: bool,
    pub show_title: bool,
    pub title_alignment: TileAlignment,
    pub title_color: Color,
}

mod optional_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::Value;

    pub fn serialize<S: Serializer, T: Serialize>(
        val: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(val) = val {
            val.serialize(serializer)
        } else {
            serializer.serialize_str("")
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>, T: for<'de2> Deserialize<'de2>>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error> {
        let val = String::deserialize(deserializer)?;
        if val.is_empty() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_value(Value::String(val)).map_err(
                |err| serde::de::Error::custom(err.to_string()),
            )?))
        }
    }
}
