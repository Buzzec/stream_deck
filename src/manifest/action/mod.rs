pub mod encoder;
pub mod state;

use crate::common::Controller;
use crate::manifest::action::encoder::Encoder;
use serde::{Deserialize, Serialize};
use state::State;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    /// The unique identifier of the action. It must be a uniform type identifier (UTI) that contains only lowercase
    /// alphanumeric characters (a-z, 0-9), hyphen (-), and period (.). The string must be in reverse-DNS format. For
    /// example, if your domain is elgato.com and you create a plugin named Hello with the action My Action, you could
    /// assign the string com.elgato.hello.myaction as your action's Unique Identifier.
    #[serde(rename = "UUID")]
    pub uuid: String,
    /// The relative path to a PNG image without the .png extension. This image is displayed in the actions list.
    /// The PNG image should be a 20pt x 20pt image. You should provide @1x and @2x versions of the image. The Stream
    /// Deck application takes care of loading the appropriate version of the image. This icon is not required for
    /// actions not visible in the actions list (VisibleInActionsList set to false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The name of the action. This string is visible to the user in the actions list.
    pub name: String,
    /// Specifies an array of states. Each action can have one or more states. For example, the Hotkey action has a
    /// single state. However, the Game Capture Record action has two states, active and inactive. The state of an
    /// action, supporting multiple states, is always automatically toggled whenever the action's key is released
    /// (after being pressed). In addition, it is possible to force the action to switch its state by sending a
    /// setState event.
    pub states: Vec<State>,
    /// This can override PropertyInspectorPath member from the plugin if you wish to have a different
    /// PropertyInspectorPath based on the action. The relative path to the Property Inspector HTML file if your plugin
    /// wants to display some custom settings in the Property Inspector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_inspector_path: Option<String>,
    /// Boolean to prevent the action from being used in a Multi Action. True by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_in_multi_actions: Option<bool>,
    /// The string is displayed as a tooltip when the user leaves the mouse over your action in the actions list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    /// Boolean to disable image caching. False by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_caching: Option<bool>,
    /// Determines whether the state of the action should automatically toggle when the user presses the action; only
    /// applies to actions that have two states defined. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_automatic_states: Option<bool>,
    /// Boolean to hide the action in the actions list. This can be used for a plugin that only works with a specific
    /// profile. True by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_in_actions_list: Option<bool>,
    /// Boolean to disable the title field for users in the property inspector. True by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_title_enabled: Option<bool>,
    /// Specifies an array of controllers. Valid values include "Keypad" and "Encoder". `["Keypad"]` by default.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub controllers: Vec<Controller>,
    /// An object containing encoder information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder: Option<Encoder>,
}
