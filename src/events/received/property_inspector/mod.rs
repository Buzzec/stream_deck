pub mod send_to_property_inspector;

pub use super::common::*;

use crate::error::Error;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyInspectorReceivedEvent {
    DidReceiveSettings(did_receive_settings::DidReceiveSettings),
    DidReceiveGlobalSettings(did_receive_global_settings::DidReceiveGlobalSettings),
    SendToPropertyInspector(send_to_property_inspector::SendToPropertyInspector),
}
impl TryFrom<Value> for PropertyInspectorReceivedEvent {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let event = value
            .as_object()
            .and_then(|x| x.get("event"))
            .and_then(|x| x.as_str())
            .ok_or(Error::EventMissingEventKey)?;

        Ok(match event {
            "didReceiveSettings" => {
                PropertyInspectorReceivedEvent::DidReceiveSettings(serde_json::from_value(value)?)
            }
            "didReceiveGlobalSettings" => PropertyInspectorReceivedEvent::DidReceiveGlobalSettings(
                serde_json::from_value(value)?,
            ),
            "sendToPropertyInspector" => PropertyInspectorReceivedEvent::SendToPropertyInspector(
                serde_json::from_value(value)?,
            ),
            _ => return Err(Error::UnknownEvent(event.to_string())),
        })
    }
}
