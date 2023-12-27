pub mod application_did_launch;
pub mod application_did_terminate;
pub mod device_did_connect;
pub mod device_did_disconnect;
pub mod dial_down;
pub mod dial_rotate;
pub mod dial_up;
pub mod did_receive_deep_link;
pub mod key_down;
pub mod key_up;
pub mod property_inspector_did_appear;
pub mod property_inspector_did_disappear;
pub mod send_to_plugin;
pub mod system_did_wake_up;
pub mod title_parameters_did_change;
pub mod touch_tap;
pub mod will_appear;
pub mod will_disappear;

pub use super::common::*;
use crate::error::Error;
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PluginReceivedEvent {
    DidReceiveSettings(did_receive_settings::DidReceiveSettings),
    DidReceiveGlobalSettings(did_receive_global_settings::DidReceiveGlobalSettings),
    KeyDown(key_down::KeyDown),
    KeyUp(key_up::KeyUp),
    TouchTap(touch_tap::TouchTap),
    DialDown(dial_down::DialDown),
    DialUp(dial_up::DialUp),
    DialRotate(dial_rotate::DialRotate),
    WillAppear(will_appear::WillAppear),
    WillDisappear(will_disappear::WillDisappear),
    TitleParametersDidChange(title_parameters_did_change::TitleParametersDidChange),
    DeviceDidConnect(device_did_connect::DeviceDidConnect),
    DeviceDidDisconnect(device_did_disconnect::DeviceDidDisconnect),
    ApplicationDidLaunch(application_did_launch::ApplicationDidLaunch),
    ApplicationDidTerminate(application_did_terminate::ApplicationDidTerminate),
    SystemDidWakeUp(system_did_wake_up::SystemDidWakeUp),
    DidReceiveDeepLink(did_receive_deep_link::DidReceiveDeepLink),
    PropertyInspectorDidAppear(property_inspector_did_appear::PropertyInspectorDidAppear),
    PropertyInspectorDidDisappear(property_inspector_did_disappear::PropertyInspectorDidDisappear),
    SendToPlugin(send_to_plugin::SendToPlugin),
}
impl TryFrom<Value> for PluginReceivedEvent {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let event = value
            .as_object()
            .and_then(|x| x.get("event"))
            .and_then(|x| x.as_str())
            .ok_or(Error::EventMissingEventKey)?;

        Ok(match event {
            "didReceiveSettings" => {
                PluginReceivedEvent::DidReceiveSettings(serde_json::from_value(value)?)
            }
            "didReceiveGlobalSettings" => {
                PluginReceivedEvent::DidReceiveGlobalSettings(serde_json::from_value(value)?)
            }
            "keyDown" => PluginReceivedEvent::KeyDown(serde_json::from_value(value)?),
            "keyUp" => PluginReceivedEvent::KeyUp(serde_json::from_value(value)?),
            "touchTap" => PluginReceivedEvent::TouchTap(serde_json::from_value(value)?),
            "dialDown" => PluginReceivedEvent::DialDown(serde_json::from_value(value)?),
            "dialUp" => PluginReceivedEvent::DialUp(serde_json::from_value(value)?),
            "dialRotate" => PluginReceivedEvent::DialRotate(serde_json::from_value(value)?),
            "willAppear" => PluginReceivedEvent::WillAppear(serde_json::from_value(value)?),
            "willDisappear" => PluginReceivedEvent::WillDisappear(serde_json::from_value(value)?),
            "titleParametersDidChange" => {
                PluginReceivedEvent::TitleParametersDidChange(serde_json::from_value(value)?)
            }
            "deviceDidConnect" => {
                PluginReceivedEvent::DeviceDidConnect(serde_json::from_value(value)?)
            }
            "deviceDidDisconnect" => {
                PluginReceivedEvent::DeviceDidDisconnect(serde_json::from_value(value)?)
            }
            "applicationDidLaunch" => {
                PluginReceivedEvent::ApplicationDidLaunch(serde_json::from_value(value)?)
            }
            "applicationDidTerminate" => {
                PluginReceivedEvent::ApplicationDidTerminate(serde_json::from_value(value)?)
            }
            "systemDidWakeUp" => {
                PluginReceivedEvent::SystemDidWakeUp(serde_json::from_value(value)?)
            }
            "didReceiveDeepLink" => {
                PluginReceivedEvent::DidReceiveDeepLink(serde_json::from_value(value)?)
            }
            "propertyInspectorDidAppear" => {
                PluginReceivedEvent::PropertyInspectorDidAppear(serde_json::from_value(value)?)
            }
            "propertyInspectorDidDisappear" => {
                PluginReceivedEvent::PropertyInspectorDidDisappear(serde_json::from_value(value)?)
            }
            "sendToPlugin" => PluginReceivedEvent::SendToPlugin(serde_json::from_value(value)?),
            x => return Err(Error::UnknownEvent(x.to_string())),
        })
    }
}
