use anyhow::Result;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::mem::replace;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use stream_deck::common::Controller;
use stream_deck::events::received::plugin::PluginReceivedEvent;
use stream_deck::events::sent::plugin::set_title::{SetTitle, SetTitlePayload};
use stream_deck::events::sent::serialize_event;
use stream_deck::runtime::plugin::Registration;
use tokio::select;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, instrument, trace, warn};

#[instrument]
#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let registration_result = Registration::register_from_args().await;
    let mut registration = match registration_result {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to register: {:?}", e);
            return Err(e.into());
        }
    };
    info!("Starting Plugin");
    let res = run(&mut registration).await;

    warn!("Exiting: {res:?}");
    res
}

struct VisibleAction {
    counter: Arc<(AtomicIsize, AtomicBool)>,
    reset_task: Option<JoinHandle<Result<()>>>,
}

#[instrument(skip(registration))]
async fn run(registration: &mut Registration) -> Result<()> {
    let mut actions = HashMap::<String, VisibleAction>::new();
    let out = loop {
        let message = select! {
            m = registration.ws.next() => { m }
        };

        match message {
            Some(Ok(Message::Ping(data))) => {
                trace!("Received Ping: {data:?}");
                registration.sends.send(Message::Pong(data)).unwrap();
            }
            Some(Ok(message)) => {
                let event = PluginReceivedEvent::try_from(message)?;
                debug!("Received: {:?}\n", event);
                match event {
                    PluginReceivedEvent::DidReceiveSettings(_) => {}
                    PluginReceivedEvent::DidReceiveGlobalSettings(_) => {}
                    PluginReceivedEvent::KeyDown(down) => {
                        let context = down.context.clone();
                        let sender = registration.sends.clone();
                        if let Some(action) = actions.get_mut(&down.context.to_string()) {
                            let counter = action.counter.clone();
                            action.counter.1.store(true, Ordering::SeqCst);
                            let old_task = replace(
                                &mut action.reset_task,
                                Some(tokio::spawn(async move {
                                    tokio::time::sleep(Duration::from_millis(500)).await;
                                    if !counter.1.load(Ordering::SeqCst) {
                                        return Ok(());
                                    }
                                    let mut val = counter.0.load(Ordering::SeqCst);
                                    loop {
                                        if val < 0 {
                                            return Ok(());
                                        }
                                        match counter.0.compare_exchange_weak(
                                            val,
                                            0,
                                            Ordering::SeqCst,
                                            Ordering::SeqCst,
                                        ) {
                                            Ok(_) => break,
                                            Err(new_old_val) => val = new_old_val,
                                        }
                                    }
                                    sender.send(Message::Text(
                                        serialize_event(&SetTitle {
                                            context: down.context,
                                            payload: SetTitlePayload {
                                                title: 0usize.to_string(),
                                                target: None,
                                                state: None,
                                            },
                                        })?
                                        .to_string(),
                                    ))?;
                                    Ok(())
                                })),
                            );
                            if let Some(task) = old_task {
                                warn!("Task still alive!");
                                task.abort();
                            }
                            let val = action.counter.0.fetch_add(1, Ordering::SeqCst);
                            registration.send_event(&SetTitle {
                                context,
                                payload: SetTitlePayload {
                                    title: (val + 1).to_string(),
                                    target: None,
                                    state: None,
                                },
                            })?;
                        }
                    }
                    PluginReceivedEvent::KeyUp(up) => {
                        if let Some(action) = actions.get_mut(&up.context.to_string()) {
                            action.counter.1.store(false, Ordering::SeqCst);
                            if let Some(task) = action.reset_task.take() {
                                task.abort();
                            }
                        }
                    }
                    PluginReceivedEvent::TouchTap(_) => {}
                    PluginReceivedEvent::DialDown(_) => {}
                    PluginReceivedEvent::DialUp(_) => {}
                    PluginReceivedEvent::DialRotate(_) => {}
                    PluginReceivedEvent::WillAppear(will_appear) => {
                        if will_appear.payload.controller == Controller::Keypad {
                            actions.insert(
                                will_appear.context.to_string(),
                                VisibleAction {
                                    counter: Arc::new((
                                        AtomicIsize::new(0),
                                        AtomicBool::new(false),
                                    )),
                                    reset_task: None,
                                },
                            );
                            registration.send_event(&SetTitle {
                                context: will_appear.context,
                                payload: SetTitlePayload {
                                    title: "0".to_string(),
                                    target: None,
                                    state: None,
                                },
                            })?;
                        }
                    }
                    PluginReceivedEvent::WillDisappear(will_disappear) => {
                        let action = actions.remove(&will_disappear.context.to_string());
                        if let Some(action) = action {
                            action.counter.0.store(-1, Ordering::SeqCst);
                            action.counter.1.store(false, Ordering::SeqCst);
                            if let Some(task) = action.reset_task {
                                task.abort();
                            }
                        }
                    }
                    PluginReceivedEvent::TitleParametersDidChange(_) => {}
                    PluginReceivedEvent::DeviceDidConnect(_) => {}
                    PluginReceivedEvent::DeviceDidDisconnect(_) => {}
                    PluginReceivedEvent::ApplicationDidLaunch(_) => {}
                    PluginReceivedEvent::ApplicationDidTerminate(_) => {}
                    PluginReceivedEvent::SystemDidWakeUp(_) => {}
                    PluginReceivedEvent::DidReceiveDeepLink(_) => {}
                    PluginReceivedEvent::PropertyInspectorDidAppear(_) => {}
                    PluginReceivedEvent::PropertyInspectorDidDisappear(_) => {}
                    PluginReceivedEvent::SendToPlugin(_) => {}
                }
            }
            x => {
                error!("Received Unknown: {:?}\n", x);
                break Ok(());
            }
        }
    };
    out
}
