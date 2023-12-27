pub mod common;
pub mod plugin;
pub mod property_inspector;

use crate::error::{Error, Result};
use serde::Serialize;
use serde_json::Value;

pub trait SendableEvent: Serialize {
    const EVENT: &'static str;
}

pub fn serialize_event<T: SendableEvent>(event: &T) -> Result<Value> {
    let mut out = serde_json::to_value(event)?;
    let old_event = out
        .as_object_mut()
        .ok_or(Error::ExpectedEventToBeObject)?
        .insert("event".to_string(), Value::String(T::EVENT.to_string()));
    assert_eq!(old_event, None);
    Ok(out)
}
