use crate::common::DeviceType;
use crate::common::{Color, DeviceSize};
use crate::manifest::os::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoParameter {
    /// A JSON object containing information about the application.
    pub application: Application,
    /// A JSON object containing information about the plugin.
    pub plugin: Plugin,
    /// A JSON array containing information about the devices.
    pub devices: Vec<Device>,
    /// Pixel ratio value to indicate if the Stream Deck application is running on a HiDPI screen.
    pub device_pixel_ratio: f64,
    /// A JSON object containing information about the preferred user colors.
    pub colors: Colors,
}

/// A JSON object containing information about the application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    /// In which language the Stream Deck application is running. Possible values are `en`, `fr`, `de`, `es`, `ja`,
    /// `zh_CN`.
    pub language: Language,
    /// On which platform the Stream Deck application is running. Possible values are
    /// `kESDSDKApplicationInfoPlatformMac` ("mac") and `kESDSDKApplicationInfoPlatformWindows` ("windows").
    pub platform: Platform,
    /// The Stream Deck application version.
    pub version: String,
    /// The operating system version.
    pub platform_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    /// English
    En,
    /// French
    Fr,
    /// German
    De,
    /// Spanish
    Es,
    /// Japanese
    Ja,
    /// Chinese
    #[serde(rename = "zh_CN")]
    ZhCn,
}

/// A JSON object containing information about the plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    /// The plugin version as written in the manifest.json.
    pub version: String,
    /// The unique identifier of the plugin.
    pub uuid: String,
}

/// A JSON object containing information about the device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// A value to identify the device.
    pub id: Value,
    /// Type of device.
    #[serde(rename = "type", with = "device_type_from_int")]
    pub device_type: DeviceType,
    /// The number of columns and rows of keys that the device owns.
    pub size: DeviceSize,
    /// The name of the device set by the user.
    pub name: String,
}

/// A JSON object containing information about the preferred user colors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    /// The background color of a pressed key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_pressed_background_color: Option<Color>,
    /// The border color of a pressed key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_pressed_border_color: Option<Color>,
    /// The text color of a pressed key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_pressed_text_color: Option<Color>,
    /// The color of a disabled key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_color: Option<Color>,
    /// The color of a highlighted key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_color: Option<Color>,
    /// The color of a pressed key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_down_color: Option<Color>,
}

mod device_type_from_int {
    use crate::common::DeviceType;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(val: &DeviceType, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(match val {
            DeviceType::StreamDeck => 0,
            DeviceType::StreamDeckMini => 1,
            DeviceType::StreamDeckXL => 2,
            DeviceType::StreamDeckMobile => 3,
            DeviceType::CorsairGKeys => 4,
            DeviceType::StreamDeckPedal => 5,
            DeviceType::CorsairVoyager => 6,
            DeviceType::StreamDeckPlus => 7,
        })
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserialize: D) -> Result<DeviceType, D::Error> {
        let val = u8::deserialize(deserialize)?;
        Ok(match val {
            0 => DeviceType::StreamDeck,
            1 => DeviceType::StreamDeckMini,
            2 => DeviceType::StreamDeckXL,
            3 => DeviceType::StreamDeckMobile,
            4 => DeviceType::CorsairGKeys,
            5 => DeviceType::StreamDeckPedal,
            6 => DeviceType::CorsairVoyager,
            7 => DeviceType::StreamDeckPlus,
            x => return Err(D::Error::custom(format!("invalid device type: {x}"))),
        })
    }
}
