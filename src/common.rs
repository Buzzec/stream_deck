use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TileAlignment {
    Top,
    Bottom,
    Middle,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq)]
pub enum FontFamily {
    Arial,
    #[serde(rename = "Arial Black")]
    ArialBlack,
    #[serde(rename = "Comic Sans MS")]
    ComicSansMS,
    Courier,
    #[serde(rename = "Courier New")]
    CourierNew,
    Georgia,
    Impact,
    #[serde(rename = "Microsoft Sans Serif")]
    MicrosoftSansSerif,
    Symbol,
    Tahoma,
    #[serde(rename = "Times New Roman")]
    TimesNewRoman,
    #[serde(rename = "Trebuchet MS")]
    TrebuchetMS,
    Verdana,
    Webdings,
    Wingdings,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    #[serde(rename = "Bold Italic")]
    BoldItalic,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq)]
pub enum Controller {
    Keypad,
    Encoder,
}

pub type Color = String;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceType {
    #[serde(rename = "kESDSDKDeviceType_StreamDeck")]
    StreamDeck,
    #[serde(rename = "kESDSDKDeviceType_StreamDeckMini")]
    StreamDeckMini,
    #[serde(rename = "kESDSDKDeviceType_StreamDeckXL")]
    StreamDeckXL,
    #[serde(rename = "kESDSDKDeviceType_StreamDeckMobile")]
    StreamDeckMobile,
    #[serde(rename = "kESDSDKDeviceType_CorsairGKeys")]
    CorsairGKeys,
    #[serde(rename = "kESDSDKDeviceType_StreamDeckPedal")]
    StreamDeckPedal,
    #[serde(rename = "kESDSDKDeviceType_CorsairVoyager")]
    CorsairVoyager,
    #[serde(rename = "kESDSDKDeviceType_StreamDeckPlus")]
    StreamDeckPlus,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DeviceTypeInteger {
    StreamDeck = 0,
    StreamDeckMini = 1,
    StreamDeckXL = 2,
    StreamDeckMobile = 3,
    CorsairGKeys = 4,
    StreamDeckPedal = 5,
    CorsairVoyager = 6,
    StreamDeckPlus = 7,
}

/// The size of a device.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSize {
    pub columns: u32,
    pub rows: u32,
}

pub type Context = String;
