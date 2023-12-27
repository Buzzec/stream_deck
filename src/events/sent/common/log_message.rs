use crate::events::sent::SendableEvent;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// The plugin and Property Inspector can use the logMessage event to write a debug message to the
/// logs file.
///
/// Logs are saved to disk per plugin in the folder `~/Library/Logs/StreamDeck/` on macOS and
/// `%appdata%\Elgato\StreamDeck\logs\` on Windows. Note that the log files are rotated each time
/// the Stream Deck application is relaunched.
///
/// This API has been introduced in Stream Deck 4.1.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogMessage<'a> {
    /// A JSON object
    pub payload: LogMessagePayload<'a>,
}
impl<'a> SendableEvent for LogMessage<'a> {
    const EVENT: &'static str = "logMessage";
}
impl<'a> From<String> for LogMessage<'a> {
    fn from(value: String) -> Self {
        Self {
            payload: LogMessagePayload {
                message: Cow::Owned(value),
            },
        }
    }
}
impl<'a> From<&'a str> for LogMessage<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            payload: LogMessagePayload {
                message: Cow::Borrowed(value),
            },
        }
    }
}
impl<'a> From<Cow<'a, str>> for LogMessage<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self {
            payload: LogMessagePayload { message: value },
        }
    }
}

/// Payload for [`LogMessage`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LogMessagePayload<'a> {
    /// A string to write to the logs file.
    pub message: Cow<'a, str>,
}
impl<'a> From<String> for LogMessagePayload<'a> {
    fn from(value: String) -> Self {
        Self {
            message: Cow::Owned(value),
        }
    }
}
impl<'a> From<&'a str> for LogMessagePayload<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            message: Cow::Borrowed(value),
        }
    }
}
impl<'a> From<Cow<'a, str>> for LogMessagePayload<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self { message: value }
    }
}
