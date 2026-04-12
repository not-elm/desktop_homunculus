//! `/signals` provides a mechanism for bridging between external processes.
//! For example, it can be used to send values from external applications created by users to a Webview created within a MOD.

use axum::Json;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use bevy::platform::collections::HashMap;
use futures::SinkExt as _;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{ApiError, SignalInfo, SignalsApi};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// List all active signal channels.
#[utoipa::path(
    get,
    path = "/",
    tag = "signals",
    responses(
        (status = 200, description = "List of active signal channels", body = Vec<SignalInfo>),
    ),
)]
pub async fn list_signals(State(api): State<SignalsApi>) -> HttpResult<Vec<SignalInfo>> {
    api.list().await.into_http_result()
}

/// Send a signal to all subscribers.
///
/// The signal is sent to all processes that are streaming the signal at `GET /signals/{signal}`.
#[utoipa::path(
    post,
    path = "/{signal}",
    tag = "signals",
    params(
        ("signal" = String, Path, description = "Signal channel name"),
    ),
    request_body = Object,
    responses(
        (status = 200, description = "Signal sent"),
    ),
)]
pub async fn send(
    State(api): State<SignalsApi>,
    Path(signal): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<(), ApiError> {
    api.send(signal, body).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum WsClientMessage {
    Subscribe { channel: String },
    Unsubscribe { channel: String },
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum WsServerMessage {
    Subscribed {
        #[serde(rename = "type")]
        msg_type: &'static str,
        channel: String,
    },
    Error {
        #[serde(rename = "type")]
        msg_type: &'static str,
        channel: String,
        message: String,
    },
    Event {
        channel: String,
        data: serde_json::Value,
    },
}

impl WsServerMessage {
    fn subscribed(channel: String) -> Self {
        Self::Subscribed {
            msg_type: "subscribed",
            channel,
        }
    }

    fn error(channel: String, message: String) -> Self {
        Self::Error {
            msg_type: "error",
            channel,
            message,
        }
    }

    fn event(channel: String, data: serde_json::Value) -> Self {
        Self::Event { channel, data }
    }
}

/// Upgrade to WebSocket for multiplexed signal streaming.
pub async fn ws_handler(
    State(api): State<SignalsApi>,
    ws: WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_ws_connection(socket, api))
}

const MAX_SUBSCRIPTIONS: usize = 64;
const MERGE_CHANNEL_CAPACITY: usize = 256;

async fn handle_ws_connection(socket: WebSocket, api: SignalsApi) {
    use futures::StreamExt;
    let (mut ws_sink, mut ws_stream) = socket.split();
    let (merge_tx, mut merge_rx) = mpsc::channel::<WsServerMessage>(MERGE_CHANNEL_CAPACITY);
    let mut forwarding_tasks: HashMap<String, JoinHandle<()>> = HashMap::new();

    loop {
        tokio::select! {
            Some(msg) = merge_rx.recv() => {
                let text = serde_json::to_string(&msg).unwrap();
                if ws_sink.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
            frame = ws_stream.next() => {
                match frame {
                    Some(Ok(Message::Text(text))) => {
                        handle_client_message(
                            &text, &api, &merge_tx, &mut forwarding_tasks,
                        ).await;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }

    for (_, handle) in forwarding_tasks {
        handle.abort();
    }
}

async fn handle_client_message(
    text: &str,
    api: &SignalsApi,
    merge_tx: &mpsc::Sender<WsServerMessage>,
    forwarding_tasks: &mut HashMap<String, JoinHandle<()>>,
) {
    let msg: WsClientMessage = match serde_json::from_str(text) {
        Ok(msg) => msg,
        Err(_) => return,
    };

    match msg {
        WsClientMessage::Subscribe { channel } => {
            handle_subscribe(channel, api, merge_tx, forwarding_tasks).await;
        }
        WsClientMessage::Unsubscribe { channel } => {
            handle_unsubscribe(channel, forwarding_tasks);
        }
    }
}

async fn handle_subscribe(
    channel: String,
    api: &SignalsApi,
    merge_tx: &mpsc::Sender<WsServerMessage>,
    forwarding_tasks: &mut HashMap<String, JoinHandle<()>>,
) {
    if forwarding_tasks.contains_key(&channel) {
        let _ = merge_tx.send(WsServerMessage::subscribed(channel)).await;
        return;
    }

    if forwarding_tasks.len() >= MAX_SUBSCRIPTIONS {
        let _ = merge_tx
            .send(WsServerMessage::error(
                channel,
                "subscription limit exceeded".to_string(),
            ))
            .await;
        return;
    }

    let rx = match api.clone().stream(channel.clone()).await {
        Ok(stream) => stream,
        Err(e) => {
            let _ = merge_tx
                .send(WsServerMessage::error(channel, e.to_string()))
                .await;
            return;
        }
    };

    let _ = merge_tx
        .send(WsServerMessage::subscribed(channel.clone()))
        .await;

    let tx = merge_tx.clone();
    let ch = channel.clone();
    let handle = tokio::spawn(async move {
        use bevy::tasks::futures_lite::StreamExt;
        let mut stream = std::pin::pin!(rx);
        while let Some(value) = stream.next().await {
            if tx
                .send(WsServerMessage::event(ch.clone(), value))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    forwarding_tasks.insert(channel, handle);
}

fn handle_unsubscribe(channel: String, forwarding_tasks: &mut HashMap<String, JoinHandle<()>>) {
    if let Some(handle) = forwarding_tasks.remove(&channel) {
        handle.abort();
    }
}
