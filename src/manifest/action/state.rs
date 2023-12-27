use crate::common::{Color, FontFamily, FontStyle, TileAlignment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    /// The default image for the state. When a user sets a custom image on the primary state, Stream Deck will
    /// automatically set the secondary state to a darker version of the same icon.
    pub image: String,
    /// This can be used if you want to provide a different image for the state when the action is displayed in a
    /// Multi-Action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_action_image: Option<String>,
    /// Displayed in the dropdown menu in the Multi-action. For example, the Game Capture Record action has `Start` and
    /// `Stop`. If the name is not provided, the state will not appear in the Multi-Action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Default title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Boolean to hide/show the title. True by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_title: Option<bool>,
    /// Default title color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<Color>,
    /// Default title vertical alignment. Possible values are "top", "bottom" and "middle".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_alignment: Option<TileAlignment>,
    /// Default font family for the title. Possible values are "Arial", "Arial Black", "Comic Sans MS", "Courier",
    /// "Courier New", "Georgia", "Impact", "Microsoft Sans Serif", "Symbol", "Tahoma", "Times New Roman",
    /// "Trebuchet MS", "Verdana", "Webdings", "Wingdings".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<FontFamily>,
    /// Default font style for the title. Possible values are "Regular", "Bold", "Italic", and "Bold Italic". Note that
    /// some fonts might not support all values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// Default font size for the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,
    /// Boolean to have an underline under the title. False by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_underline: Option<bool>,
}
