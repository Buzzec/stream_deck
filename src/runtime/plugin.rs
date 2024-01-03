use crate::error::{Error, Result};
use crate::events::sent::plugin::log_message::LogMessage;
use crate::events::sent::{serialize_event, SendableEvent};
use crate::runtime::info_parameter::InfoParameter;
use futures_util::stream::SplitStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::env::args;
use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::task::{spawn_blocking, JoinHandle};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, instrument, Level};
use tracing_subscriber::filter::Directive;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
pub struct Registration {
    pub port: u16,
    pub plugin_uuid: String,
    pub register_event: String,
    pub info: InfoParameter,
    pub sends: UnboundedSender<Message>,
    pub ws: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub send_task: JoinHandle<()>,
}
impl Registration {
    #[instrument]
    pub async fn register_from_args() -> Result<Self> {
        Self::register(args()).await
    }

    #[instrument(skip_all)]
    pub async fn register(args: impl IntoIterator<Item = String>) -> Result<Self> {
        let mut args = args.into_iter().skip(1);
        let port_text: String = args.next().ok_or(Error::PluginMissingArgs)?;
        if port_text != "-port" {
            return Err(Error::PluginInvalidArg {
                expected: "-port".to_string(),
                actual: port_text,
            });
        }
        let port = args
            .next()
            .ok_or(Error::PluginMissingArgs)?
            .parse::<u16>()?;
        let plugin_uuid_text: String = args.next().ok_or(Error::PluginMissingArgs)?;
        if plugin_uuid_text != "-pluginUUID" {
            return Err(Error::PluginInvalidArg {
                expected: "-pluginUUID".to_string(),
                actual: plugin_uuid_text,
            });
        }
        let plugin_uuid = args.next().ok_or(Error::PluginMissingArgs)?;
        let register_event_text: String = args.next().ok_or(Error::PluginMissingArgs)?;
        if register_event_text != "-registerEvent" {
            return Err(Error::PluginInvalidArg {
                expected: "-registerEvent".to_string(),
                actual: register_event_text,
            });
        }
        let register_event = args.next().ok_or(Error::PluginMissingArgs)?;
        let info_text: String = args.next().ok_or(Error::PluginMissingArgs)?;
        if info_text != "-info" {
            return Err(Error::PluginInvalidArg {
                expected: "-info".to_string(),
                actual: info_text,
            });
        }
        let info_text = args.next().ok_or(Error::PluginMissingArgs)?;
        let info: InfoParameter = serde_json::from_str(&info_text)?;

        let (mut ws, _) =
            tokio_tungstenite::connect_async(format!("ws://localhost:{}", port)).await?;
        ws.send(Message::Text(
            json!({
                "event": register_event,
                "uuid": plugin_uuid,
            })
            .to_string(),
        ))
        .await?;

        let (mut tx, rx) = ws.split();
        let (sends, mut sends_rx) = unbounded_channel();
        let send_task = tokio::spawn(async move {
            while let Some(message) = sends_rx.recv().await {
                tx.send(message).await.unwrap();
            }
        });

        let sends_clone = sends.clone();
        spawn_blocking(move || -> Result<_> {
            let subscriber = tracing_subscriber::fmt()
                .json()
                .with_writer(move || {
                    struct LogsWriter {
                        buffer: Vec<u8>,
                        sender: UnboundedSender<Message>,
                    }
                    impl Write for LogsWriter {
                        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                            self.buffer.write(buf)
                        }

                        fn flush(&mut self) -> io::Result<()> {
                            let message_buffer = String::from_utf8_lossy(&self.buffer);
                            let message = serialize_event(&LogMessage {
                                payload: message_buffer
                                    .trim()
                                    .trim_end_matches(&['\n', '\r'])
                                    .into(),
                            })
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                            .to_string();

                            self.sender
                                .send(Message::Text(message))
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                            Ok(())
                        }
                    }
                    impl Drop for LogsWriter {
                        fn drop(&mut self) {
                            self.flush().unwrap();
                        }
                    }

                    LogsWriter {
                        buffer: Vec::new(),
                        sender: sends_clone.clone(),
                    }
                })
                .with_env_filter(
                    EnvFilter::builder()
                        .with_default_directive(Directive::from(Level::DEBUG))
                        .from_env_lossy(),
                )
                .finish();
            tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");
            Ok(())
        })
        .await
        .unwrap()?;

        Ok(Self {
            port,
            plugin_uuid,
            register_event,
            info,
            sends,
            ws: rx,
            send_task,
        })
    }

    pub fn send_event(&mut self, event: &impl SendableEvent) -> Result<()> {
        let message = serialize_event(event)?.to_string();
        debug!("Sending Message: {message}");
        self.sends.send(Message::Text(message)).unwrap();
        Ok(())
    }
}
