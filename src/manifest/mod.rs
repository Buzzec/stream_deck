pub mod action;
mod applications_to_monitor;
pub mod os;
pub mod profile;
pub mod software;

use action::Action;
use applications_to_monitor::ApplicationsToMonitor;
use os::OS;
use profile::Profile;
use serde::{Deserialize, Serialize};
use software::Software;

/// Describes the plugin.
///
/// Every plugin has a manifest to provide information to the Stream Deck application as well as the Stream Deck store.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    /// Specifies an array of actions. A plugin can indeed have one or multiple actions. For example, the Game Capture
    /// plugin has six actions: Scene, Record, Screenshot, Flashback Recording, Stream, Live Commentary.
    pub actions: Vec<Action>,
    /// The author of the plugin. This string is displayed to the user in the Stream Deck store.
    pub author: String,
    /// The relative path to the HTML/binary file containing the plugin code.
    pub code_path: String,
    /// Provides a general description of what the plugin does. This string is displayed to the user in the
    /// Stream Deck store.
    pub description: String,
    /// The relative path to a png image without the .png extension. This image is displayed in the Stream Deck
    /// marketplace.
    /// The PNG image should be a 288 x 288 px. You should provide @1x and @2x(288 x 288 px & 576 x 576 px respectively).
    /// The Stream Deck application takes care of loading the appropriate version of the image.
    pub icon: String,
    /// The name of the plugin. This string is displayed to the user in the Stream Deck store.
    pub name: String,
    /// Plugin's semantic version (1.0.0)
    pub version: String,
    /// The current SDK version is 2
    #[serde(rename = "SDKVersion")]
    pub sdk_version: u8,
    /// The list of operating systems & versions supported by the plugin.
    #[serde(rename = "OS")]
    pub os: Vec<OS>,
    /// Indicates which version of the Stream Deck application is required to install the plugin.
    pub software: Software,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// Override CodePath for macOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_path_mac: Option<String>,
    /// Override CodePath for Windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_path_windows: Option<String>,
    /// Specifies an array of profiles. A plugin can have one or more profiles proposed to the user on installation.
    /// This lets you create full screen plugins.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<Profile>,
    /// The relative path to the Property Inspector HTML file if your plugin wants to display some custom settings in
    /// the Property Inspector. If missing, the plugin will have an empty Property Inspector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_inspector_path: Option<String>,
    /// Specify the default window size when a Javascript plugin or Property Inspector opens a window using
    /// window.open(). The default value is `[500, 650]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_window_size: Option<[u32; 2]>,
    /// A site to provide more information about the plugin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// List of application identifiers to monitor (applications launched or terminated). See the
    /// `applicationDidLaunch` and `applicationDidTerminate` events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_to_monitor: Option<ApplicationsToMonitor>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Category {
    /// The name of the custom category in which the actions should be listed. This string is visible to the user in
    /// the actions list. If you don't provide a category, the actions will appear inside a "Custom" category.
    pub category: String,
    /// The relative path to a PNG image without the .png extension. This image is used in the actions list. The PNG
    /// image should be a 28pt x 28pt image. You should provide @1x and @2x versions of the image. The Stream Deck
    /// application takes care of loading the appropriate version of the image.
    pub category_icon: String,
}
