use std::num::ParseIntError;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Failed to parse int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Failed to connect to WebSocket: {0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Event is missing the `event` key")]
    EventMissingEventKey,
    #[error("Unknown event: {0}")]
    UnknownEvent(String),
    #[error("Expected event to be an object")]
    ExpectedEventToBeObject,
    #[error("Plugin is missing initialization args")]
    PluginMissingArgs,
    #[error("Plugin received invalid args: expected {expected}, got {actual}")]
    PluginInvalidArg { expected: String, actual: String },
}
