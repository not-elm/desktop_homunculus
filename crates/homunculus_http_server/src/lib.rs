//! # Homunculus HTTP Server
//!
//! This crate provides a REST API server for the Desktop Homunculus application,
//! enabling external applications and scripts to control the mascot through
//! HTTP requests.
//!
//! ## Overview
//!
//! `homunculus_http_server` implements a comprehensive REST API using Axum
//! that exposes all major Homunculus functionality through HTTP endpoints.
//! This allows external applications, web interfaces, browser extensions,
//! and scripts to interact with the desktop mascot.
//!
//! ## Key Features
//!
//! - **Complete REST API**: Full HTTP interface to all mascot functionality
//! - **CORS Support**: Cross-origin requests enabled for web applications
//! - **Async Integration**: Non-blocking integration with Bevy's game loop
//! - **JSON Responses**: Structured JSON responses for all endpoints
//! - **Route Organization**: Logically organized endpoint groups
//! - **Error Handling**: Comprehensive error responses and logging
//!
//! ## API Endpoints
//!
//! The server provides REST endpoints organized by functionality:
//!
//! ### Application Control
//! - `POST /app/exit` - Exit the application
//!
//! ### VRM Management
//! - `GET /vrm/` - Get VRM model information
//! - `POST /vrm/` - Spawn new VRM model
//! - `GET /vrm/all` - List all VRM models
//! - `DELETE /vrm/{entity}/despawn` - Remove VRM model
//!
//! ### Animation Control
//! - `PUT /vrma/{entity}/play` - Play VRMA animation
//! - `PUT /vrma/{entity}/stop` - Stop VRMA animation
//!
//! ### GPT Integration
//! - `POST /gpt/chat` - Send chat message to GPT
//! - `GET/PUT /gpt/model` - Get/set GPT model
//! - `GET/PUT /gpt/system-prompt` - Get/set system prompt
//!
//! ### Effects and Media
//! - `POST /effects/stamps` - Display visual stamp effect
//! - `POST /effects/sounds` - Play sound effect
//!
//! ### Preferences and Settings
//! - `GET/PUT /preferences/{key}` - Get/set preference values
//! - `GET/PUT /settings/fps` - Get/set frame rate limit
//!
//! ## Server Configuration
//!
//! The HTTP server runs on `127.0.0.1:3100` by default and includes:
//! - Full CORS headers for web application access
//! - JSON request/response handling
//! - Comprehensive error logging and tracing
//! - Integration with Bevy's async task system
//!
//! ## External Integration
//!
//! This API enables integration with:
//! - Web-based control panels
//! - Browser extensions
//! - Automation scripts
//! - Third-party applications
//! - Development tools and debuggers

mod extract;
mod route;
mod state;

use crate::route::{cameras, displays, preferences, scripts, shadow_panel, vrm, vrma, webviews};
use crate::state::HttpState;
use axum::Router;
use axum::routing::{delete, get, post, put};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use homunculus_api::prelude::ApiReactor;
use homunculus_core::prelude::OutputLog;
use route::entities;
use tokio::runtime::Runtime;
use tower_http::cors::{Any, CorsLayer};

pub mod prelude {
    pub use crate::HomunculusHttpServerPlugin;
}

/// Plugin that provides a REST API HTTP server for external control of the Homunculus application.
///
/// This plugin starts an HTTP server that exposes comprehensive REST endpoints
/// for controlling all aspects of the desktop mascot, enabling integration with
/// external applications, web interfaces, and automation scripts.
///
/// # Server Details
///
/// - **Address**: `127.0.0.1:3100`
/// - **Protocol**: HTTP/1.1 with full CORS support
/// - **Format**: JSON request/response bodies
/// - **Runtime**: Tokio async runtime integrated with Bevy
///
/// # API Categories
///
/// The server provides endpoints organized into logical groups:
/// - Application control (exit, status)
/// - VRM model management (spawn, despawn, list)
/// - Animation control (play, stop VRMA animations)
/// - GPT integration (chat, model selection, prompts)
/// - Effects system (visual stamps, sound effects)
/// - Preferences and settings management
/// - Camera and display control
/// - WebView management
///
/// # Integration
///
/// The server integrates seamlessly with the Homunculus API system,
/// using the ApiReactor to communicate with the main application thread.
/// All requests are processed asynchronously without blocking the game loop.
///
/// # CORS Configuration
///
/// The server is configured with permissive CORS headers to enable
/// access from web applications, browser extensions, and other
/// cross-origin clients.
pub struct HomunculusHttpServerPlugin;

impl Plugin for HomunculusHttpServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(reactor: Res<ApiReactor>) {
    let reactor = reactor.clone();
    IoTaskPool::get()
        .spawn(async move {
            let rt = Runtime::new().unwrap();
            rt.spawn(async move { start_http_server(reactor).await })
                .await
                .output_log_if_error("HTTP");
        })
        .detach();
}

async fn start_http_server(reactor: ApiReactor) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100").await?;
    axum::serve(listener, create_router(reactor)).await?;
    Ok(())
}

fn create_router(reactor: ApiReactor) -> Router {
    Router::new()
        .nest("/app", app_router())
        .nest("/shadow-panel", shadow_panel_router())
        .nest("/gpt", gpt_router())
        .nest("/entities", entities_router())
        .nest("/vrm", vrm_router())
        .nest("/vrma/{entity}", vrma_router())
        .nest("/cameras", cameras_router())
        .nest("/preferences", preferences_router())
        .nest("/settings", settings_router())
        .nest("/webviews", webviews_router())
        .nest("/displays", display_router())
        .nest("/scripts", scripts_router())
        .nest("/mods", mods_router())
        .nest("/commands", commands_router())
        .nest("/effects", effects_router())
        .with_state(HttpState::from(reactor))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

fn app_router() -> Router<HttpState> {
    Router::new().route("/exit", post(route::app::exit))
}

fn effects_router() -> Router<HttpState> {
    Router::new()
        .route("/stamps", post(route::effects::stamp))
        .route("/sounds", post(route::effects::sound))
}

fn entities_router() -> Router<HttpState> {
    Router::new()
        .route("/", get(entities::get))
        .nest("/{entity}", entities_id_router())
}

fn entities_id_router() -> Router<HttpState> {
    Router::new()
        .route("/transform", get(entities::transform::get))
        .route("/transform", put(entities::transform::put))
        .route("/name", get(entities::name::get))
}

fn vrm_router() -> Router<HttpState> {
    Router::new()
        .route("/", get(vrm::get))
        .route("/", post(vrm::spawn))
        .route("/all", get(vrm::all))
        .route("/wait-load", get(vrm::wait_load))
        .nest("/{entity}", vrm_entity_router())
}

fn vrm_entity_router() -> Router<HttpState> {
    Router::new()
        .route("/state", get(vrm::state::get).put(vrm::state::put))
        .route("/events", get(route::vrm::events))
        .route("/vrma", get(vrm::vrma))
        .route("/speech/voicevox", post(vrm::speech::voicevox))
        .route("/look", delete(vrm::look::unlook))
        .route("/look/target/{target}", put(vrm::look::target))
        .route("/look/cursor", put(vrm::look::cursor))
        .route("/despawn", delete(vrm::despawn))
}

fn vrma_router() -> Router<HttpState> {
    Router::new()
        .route("/play", put(vrma::play))
        .route("/stop", put(vrma::stop))
}

fn cameras_router() -> Router<HttpState> {
    Router::new()
        .route("/world-2d", get(cameras::world_2d))
        .route("/global-viewport", get(cameras::global_viewport))
}

fn shadow_panel_router() -> Router<HttpState> {
    Router::new().route(
        "/alpha",
        get(shadow_panel::alpha::get).put(shadow_panel::alpha::put),
    )
}

fn webviews_router() -> Router<HttpState> {
    Router::new()
        .route("/", post(webviews::open))
        .route("/{entity}/close", post(webviews::close))
        .route("/{entity}/is-closed", get(webviews::is_closed))
}

fn preferences_router() -> Router<HttpState> {
    Router::new()
        .route("/{key}", get(preferences::load))
        .route("/{key}", put(preferences::save))
}

fn settings_router() -> Router<HttpState> {
    Router::new()
        .route("/fps", get(route::settings::fps))
        .route("/fps", put(route::settings::set_fps))
}

fn display_router() -> Router<HttpState> {
    Router::new().route("/", get(displays::all))
}

fn scripts_router() -> Router<HttpState> {
    Router::new().route("/js", post(scripts::js))
}

fn mods_router() -> Router<HttpState> {
    Router::new().route("/menus", get(route::mods::menus))
}

fn gpt_router() -> Router<HttpState> {
    Router::new()
        .route("/available-models", get(route::gpt::available_models))
        .route("/model", get(route::gpt::model::get))
        .route("/model", put(route::gpt::model::put))
        .route("/use-web-search", get(route::gpt::use_web_search::get))
        .route("/use-web-search", put(route::gpt::use_web_search::put))
        .route("/system-prompt", get(route::gpt::system_prompt::get))
        .route("/system-prompt", put(route::gpt::system_prompt::put))
        .route("/speaker/voicevox", get(route::gpt::speaker::get))
        .route("/speaker/voicevox", put(route::gpt::speaker::put))
        .route("/chat", post(route::gpt::chat))
}

fn commands_router() -> Router<HttpState> {
    Router::new()
        .route("/{command}", get(route::commands::stream))
        .route("/{command}", post(route::commands::send))
}

#[cfg(test)]
mod tests {
    use crate::create_router;
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode};
    use bevy::prelude::*;
    use bevy::render::camera::CameraPlugin;
    use bevy::tasks::{block_on, poll_once};
    use homunculus_api::HomunculusApiPlugin;
    use homunculus_api::prelude::{ApiReactor, ShadowPanelApiPlugin};
    use homunculus_prefs::PrefsDatabase;
    use http_body_util::BodyExt;
    use serde::de::DeserializeOwned;
    use std::fmt::Debug;
    use tokio::pin;
    use tower::ServiceExt;

    pub fn test_app() -> (App, Router) {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy_flurx::prelude::FlurxPlugin,
            TransformPlugin,
            WindowPlugin::default(),
            AssetPlugin::default(),
            ImagePlugin::default_linear(),
            CameraPlugin,
            HomunculusApiPlugin
                .build()
                .disable::<ShadowPanelApiPlugin>(),
        ));

        app.insert_non_send_resource(PrefsDatabase::open_in_memory());
        let router = create_router(app.world().resource::<ApiReactor>().clone());
        (app, router)
    }

    pub async fn assert_response<B>(
        app: &mut App,
        router: Router,
        request: Request<Body>,
        expected: B,
    ) where
        B: DeserializeOwned + PartialEq + Debug + Send + Sync + 'static,
    {
        let response = call(app, router, request).await;
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice::<B>(&body).unwrap();
        assert_eq!(body, expected);
    }

    pub async fn call(app: &mut App, router: Router, request: Request<Body>) -> Response<Body> {
        let h = router.oneshot(request);
        pin!(h);

        loop {
            app.update();
            if let Some(result) = block_on(poll_once(&mut h)) {
                let response = result.unwrap();
                let status = response.status();
                if status != StatusCode::OK {
                    let body = response.into_body().collect().await.unwrap();
                    let text = String::from_utf8(body.to_bytes().to_vec()).unwrap();
                    panic!("Failed api status={status}\n{text}",);
                }
                return response;
            }
        }
    }
}
