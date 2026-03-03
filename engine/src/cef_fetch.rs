use base64::Engine;
use base64::engine::general_purpose::STANDARD as Base64Engine;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_cef::prelude::{HostEmitEvent, JsEmitEventPlugin, Receive};
use homunculus_core::prelude::OutputLog;
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

const FETCH_RESPONSE_EVENT: &str = "cef_fetch_response";

pub struct CefFetchPlugin;

impl Plugin for CefFetchPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = async_channel::unbounded();
        app.insert_resource(CefFetchClient(reqwest::Client::new()))
            .insert_resource(CefFetchResponseSender(tx))
            .insert_resource(CefFetchResponseReceiver(rx))
            .insert_resource(AbortRegistry::default())
            .add_plugins((
                JsEmitEventPlugin::<CefFetchRequest>::default(),
                JsEmitEventPlugin::<CefFetchAbort>::default(),
            ))
            .add_observer(handle_fetch_request)
            .add_observer(handle_fetch_abort)
            .add_systems(Update, emit_fetch_responses);
    }
}

#[derive(Resource, Clone)]
struct CefFetchClient(reqwest::Client);

#[derive(Resource)]
struct CefFetchResponseSender(async_channel::Sender<CefFetchResponseEvent>);

#[derive(Resource)]
struct CefFetchResponseReceiver(async_channel::Receiver<CefFetchResponseEvent>);

#[derive(Resource, Default)]
struct AbortRegistry(Arc<Mutex<HashSet<String>>>);

#[derive(Debug, Deserialize)]
pub struct CefFetchRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
    #[serde(rename = "bodyBase64")]
    pub body_base64: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CefFetchAbort {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct CefFetchResponse {
    pub id: String,
    pub ok: bool,
    pub status: u16,
    #[serde(rename = "statusText")]
    pub status_text: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    #[serde(rename = "bodyBase64")]
    pub body_base64: Option<String>,
    pub error: Option<String>,
}

struct CefFetchResponseEvent {
    webview: Entity,
    response: CefFetchResponse,
}

fn handle_fetch_abort(trigger: On<Receive<CefFetchAbort>>, registry: Res<AbortRegistry>) {
    if trigger.kind != "abort" {
        return;
    }
    if let Ok(mut aborted) = registry.0.lock() {
        aborted.insert(trigger.id.clone());
    }
}

fn handle_fetch_request(
    trigger: On<Receive<CefFetchRequest>>,
    client: Res<CefFetchClient>,
    sender: Res<CefFetchResponseSender>,
    registry: Res<AbortRegistry>,
) {
    let request = &trigger.payload;
    if request.kind != "request" {
        return;
    }
    let webview = trigger.webview;
    let client = client.0.clone();
    let sender = sender.0.clone();
    let registry = registry.0.clone();

    let id = request.id.clone();
    let url = request.url.clone();
    let method = request.method.clone().unwrap_or_else(|| "GET".to_string());
    let headers = request.headers.clone().unwrap_or_default();
    let body_base64 = request.body_base64.clone();
    IoTaskPool::get()
        .spawn(async move {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                if take_abort(&registry, &id) {
                    return;
                }

                let response = match build_request(&client, &url, &method, &headers, body_base64) {
                    Ok(builder) => execute_request(builder, id.clone(), url.clone()).await,
                    Err(err) => CefFetchResponse::error(id.clone(), url.clone(), err),
                };

                if take_abort(&registry, &id) {
                    return;
                }

                sender
                    .send(CefFetchResponseEvent { webview, response })
                    .await
                    .output_log_if_error("CefFetch");
            });
        })
        .detach();
}

fn emit_fetch_responses(mut commands: Commands, receiver: Res<CefFetchResponseReceiver>) {
    while let Ok(event) = receiver.0.try_recv() {
        commands.trigger(HostEmitEvent::new(
            event.webview,
            FETCH_RESPONSE_EVENT,
            &event.response,
        ));
    }
}

fn build_request(
    client: &reqwest::Client,
    url: &str,
    method: &str,
    headers: &[(String, String)],
    body_base64: Option<String>,
) -> Result<reqwest::RequestBuilder, String> {
    let method = method
        .parse::<reqwest::Method>()
        .map_err(|e| format!("Invalid method: {e}"))?;
    let mut builder = client.request(method, url);
    for (name, value) in headers {
        let Ok(header_name) = HeaderName::from_bytes(name.as_bytes()) else {
            continue;
        };
        let Ok(header_value) = HeaderValue::from_str(value) else {
            continue;
        };
        builder = builder.header(header_name, header_value);
    }
    if let Some(body_base64) = body_base64 {
        let bytes = Base64Engine
            .decode(body_base64.as_bytes())
            .map_err(|e| format!("Invalid body base64: {e}"))?;
        builder = builder.body(bytes);
    }
    Ok(builder)
}

async fn execute_request(
    builder: reqwest::RequestBuilder,
    id: String,
    url: String,
) -> CefFetchResponse {
    let response = match builder.send().await {
        Ok(response) => response,
        Err(err) => {
            return CefFetchResponse::error(id, url, err.to_string());
        }
    };

    let status = response.status();
    let status_text = status.canonical_reason().unwrap_or("").to_string();
    let url = response.url().to_string();
    let ok = status.is_success();
    let headers = response_headers(response.headers());
    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(err) => {
            return CefFetchResponse::error(id, url, err.to_string());
        }
    };
    let body_base64 = Some(Base64Engine.encode(body_bytes));

    CefFetchResponse {
        id,
        ok,
        status: status.as_u16(),
        status_text,
        url,
        headers,
        body_base64,
        error: None,
    }
}

impl CefFetchResponse {
    fn error(id: String, url: String, message: String) -> Self {
        Self {
            id,
            ok: false,
            status: 0,
            status_text: String::new(),
            url,
            headers: Vec::new(),
            body_base64: None,
            error: Some(message),
        }
    }
}

fn response_headers(headers: &reqwest::header::HeaderMap) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect()
}

fn take_abort(registry: &Arc<Mutex<HashSet<String>>>, id: &str) -> bool {
    registry
        .lock()
        .ok()
        .map(|mut set| set.remove(id))
        .unwrap_or(false)
}
