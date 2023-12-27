use anyhow::Result;
use futures_util::StreamExt;
use stream_deck::runtime::plugin::Registration;
use tokio::select;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, instrument, warn};

#[instrument]
#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let registration_result = Registration::register().await;
    let registration = match registration_result {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to register: {:?}", e);
            return Err(e.into());
        }
    };
    info!("Starting Plugin");
    let res = run(registration).await;

    warn!("Exiting: {res:?}");
    res
}

#[instrument(skip(registration))]
async fn run(mut registration: Registration) -> Result<()> {
    let mut counter = 0;
    loop {
        let message = select! {
            m = registration.ws.next() => { m }
            _ = sleep(std::time::Duration::from_secs(1)) => {
                info!("Counter at {counter}");
                counter += 1;
                continue;
            }
        };

        match message {
            Some(Ok(Message::Ping(data))) => {
                debug!("Received Ping: {data:?}");
                registration.sends.send(Message::Pong(data)).unwrap();
            }
            Some(Ok(message)) => {
                debug!("Received: {message:?}");
            }
            x => {
                error!("Received Unknown: {:?}\n", x);
                return Ok(());
            }
        }
    }
}
