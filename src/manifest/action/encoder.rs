use crate::common::Color;
use serde::{Deserialize, Serialize};

/// The `Encoder` property is used to describe and configure the dial and display segment on
/// Stream Deck +.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Encoder {
    /// The default background image for the encoders touch display slot.
    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// The default icon found in the property inspector, dial stack image, and the layout. If no
    /// icon is set Stream Deck will use the action list icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// A string containing the name of a built-in layout or the partial path to a JSON file with a
    /// custom layout definition. You can dynamically change the layout with with setFeedbackLayout
    /// event. The default layout is the Icon Layout ($X1)
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    /// The color that will be used in the dial stack as background color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_color: Option<Color>,
    /// An object containing strings to describe actions in the property inspector. "Rotate", "Push", "Touch" and "Long Touch".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_description: Option<TriggerDescription>,
}

/// Used to describe encoder actions in the property inspector.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TriggerDescription {
    pub rotate: String,
    pub push: String,
    pub touch: String,
    pub long_touch: String,
}
