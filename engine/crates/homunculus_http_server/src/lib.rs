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
//! ### Persona Management
//! - `POST /personas` - Create a new persona
//! - `GET /personas` - List all personas
//! - `GET /personas/{id}` - Get persona details
//! - `PATCH /personas/{id}` - Partial update persona
//! - `DELETE /personas/{id}` - Delete persona
//! - `GET /personas/{id}/events` - SSE event stream
//! - `GET /personas/stream` - Combined SSE stream for all personas
//!
//! ### VRM Operations (via persona)
//! - `POST /personas/{id}/vrm` - Attach VRM model
//! - `DELETE /personas/{id}/vrm` - Detach VRM model
//! - `POST /personas/{id}/vrm/vrma/play` - Play VRMA animation
//! - `POST /personas/{id}/vrm/vrma/stop` - Stop VRMA animation
//!
//! ### Effects
//! - `POST /effects/stamps` - Display visual stamp effect
//!
//! ### Preferences
//! - `GET/PUT /preferences/{key}` - Get/set preference values
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

pub mod prelude {
    pub use crate::HomunculusHttpServerPlugin;
}

use crate::route::{
    assets, audio, coordinates, displays, info, persona, preferences, settings, shadow_panel, stt,
    webviews,
};
use crate::state::HttpState;
use axum::Router;
use bevy::prelude::*;
use bevy_flurx::action::side_effect;
use bevy_flurx::prelude::Reactor;
use homunculus_api::prelude::ApiReactor;
use homunculus_core::rpc_registry::{RpcRegistry, SharedRpcRegistry};
use homunculus_utils::config::HomunculusConfig;
use route::entities;
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Homunculus API",
        version = "1.0.0",
        description = "Desktop Homunculus HTTP API for controlling VRM characters, webviews, audio, and more.",
        license(name = "LGPL-3.0-only"),
    ),
    tags(
        (name = "app", description = "Application lifecycle"),
        (name = "audio", description = "Sound effects and background music"),
        (name = "personas", description = "Persona management"),
        (name = "entities", description = "Entity transform and tween control"),
        (name = "webviews", description = "WebView management"),
        (name = "coordinates", description = "Coordinate system conversion"),
        (name = "effects", description = "Visual effects"),
        (name = "preferences", description = "User preferences"),
        (name = "signals", description = "Pub/sub signal system"),
        (name = "settings", description = "Application settings"),
        (name = "shadow-panel", description = "Shadow panel transparency"),
        (name = "displays", description = "Display information"),
        (name = "mods", description = "Mod management"),
        (name = "commands", description = "Command execution"),
        (name = "assets", description = "Asset management"),
        (name = "rpc", description = "MOD service RPC registration and proxy"),
        (name = "stt", description = "Speech-to-text"),
        (name = "dialog", description = "Native OS dialogs"),
    ),
    servers(
        (url = "http://localhost:3100", description = "Local development"),
    ),
)]
pub struct ApiDoc;

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
/// - Effects system (visual stamps, sound effects)
/// - Preferences management
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
        app.add_systems(PreStartup, setup);
    }
}

/// Build the OpenAPI specification without starting the HTTP server.
///
/// This constructs the full router tree to collect all path annotations,
/// then extracts the OpenAPI spec without actually starting any server.
pub fn create_openapi() -> utoipa::openapi::OpenApi {
    let (_, api) = build_openapi_router().split_for_parts();
    api
}

fn setup(
    mut commands: Commands,
    reactor: Res<ApiReactor>,
    config: Res<HomunculusConfig>,
    rpc_registry: Res<SharedRpcRegistry>,
) {
    let reactor = reactor.clone();
    let config = config.clone();
    let addr = config.host();
    let rpc_registry = rpc_registry.0.clone();
    commands.spawn(Reactor::schedule(|task| async move {
        task.will(
            Update,
            side_effect::tokio::spawn(async move {
                if let Err(e) = start_http_server(reactor, config, rpc_registry, addr).await {
                    error!("Failed to start http server: {e}");
                }
            }),
        )
        .await;
    }));
}

async fn start_http_server(
    reactor: ApiReactor,
    config: HomunculusConfig,
    rpc_registry: Arc<RwLock<RpcRegistry>>,
    addr: String,
) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("HTTP server listening on {addr}");
    axum::serve(listener, create_router(reactor, config, rpc_registry)).await?;
    Ok(())
}

/// Build the OpenApiRouter with all routes registered, used for both
/// the live server and OpenAPI spec generation.
fn build_openapi_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/app", app_router())
        .nest("/settings", settings_router())
        .nest("/shadow-panel", shadow_panel_router())
        .nest("/entities", entities_router())
        .nest("/personas", persona_router())
        .nest("/coordinates", coordinates_router())
        .nest("/preferences", preferences_router())
        .nest("/webviews", webviews_router())
        .nest("/displays", display_router())
        .nest("/signals", signals_router())
        .nest("/audio", audio_router())
        .nest("/effects", effects_router())
        .nest("/mods", mods_router())
        .nest("/commands", commands_router())
        .nest("/stt", stt_router())
        .nest("/dialog", dialog_router())
        .routes(routes!(assets::list))
        .routes(routes!(assets::import))
        .routes(routes!(assets::get_asset_file))
        .nest("/rpc", rpc_openapi_router())
}

fn create_router(
    reactor: ApiReactor,
    config: HomunculusConfig,
    rpc_registry: Arc<RwLock<RpcRegistry>>,
) -> Router {
    let (router, _openapi) = build_openapi_router().split_for_parts();
    router
        .with_state(HttpState::new(
            reactor.clone(),
            config.clone(),
            rpc_registry.clone(),
        ))
        .nest_service(
            "/mcp",
            homunculus_mcp::create_mcp_service(reactor, config, rpc_registry),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

fn rpc_openapi_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(route::rpc::register))
        .routes(routes!(route::rpc::deregister))
        .routes(routes!(route::rpc::list_registrations))
        .routes(routes!(route::rpc::call))
}

fn app_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(route::health))
        .routes(routes!(info::get))
        .routes(routes!(route::app::exit))
}

fn audio_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(audio::se_play))
        .routes(routes!(
            audio::bgm_status,
            audio::bgm_play,
            audio::bgm_update
        ))
        .routes(routes!(audio::bgm_stop))
        .routes(routes!(audio::bgm_pause))
        .routes(routes!(audio::bgm_resume))
}

fn effects_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new().routes(routes!(route::effects::stamp))
}

fn mods_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(route::mods::list))
        .routes(routes!(route::mods::list_menus))
        .routes(routes!(route::mods::get_one))
}

fn stt_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(stt::recognize))
        .routes(routes!(stt::download_model))
        .routes(routes!(stt::cancel_download))
        .routes(routes!(stt::download_model_stream))
        .routes(routes!(stt::list_models))
        .routes(routes!(stt::list_languages))
        .routes(routes!(stt::ptt_start))
        .routes(routes!(stt::ptt_stop))
}

fn dialog_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(route::dialog::pick_folder))
        .routes(routes!(route::dialog::pick_file))
        .routes(routes!(route::dialog::pick_files))
}

fn commands_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new().routes(routes!(route::mods::execute_command))
}

fn entities_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(entities::get))
        .nest("/{entity}", entities_id_router())
}

fn entities_id_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(entities::transform::get, entities::transform::put))
        .routes(routes!(entities::name::get))
        .routes(routes!(entities::move_to::move_to))
        .routes(routes!(entities::tween::tween_position))
        .routes(routes!(entities::tween::tween_rotation))
        .routes(routes!(entities::tween::tween_scale))
}

fn persona_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(persona::get::list, persona::create::create))
        .routes(routes!(persona::snapshot::snapshot))
        .routes(routes!(persona::stream::stream))
        .nest("/{id}", persona_id_router())
}

fn persona_id_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(
            persona::get::get,
            persona::update::patch,
            persona::delete::delete
        ))
        .routes(routes!(
            persona::fields::get_name,
            persona::fields::put_name
        ))
        .routes(routes!(persona::fields::get_age, persona::fields::put_age))
        .routes(routes!(
            persona::fields::get_gender,
            persona::fields::put_gender
        ))
        .routes(routes!(
            persona::fields::get_first_person_pronoun,
            persona::fields::put_first_person_pronoun
        ))
        .routes(routes!(
            persona::fields::get_profile,
            persona::fields::put_profile
        ))
        .routes(routes!(
            persona::fields::get_personality,
            persona::fields::put_personality
        ))
        .routes(routes!(persona::state::get, persona::state::put))
        .routes(routes!(
            persona::fields::get_metadata,
            persona::fields::put_metadata
        ))
        .routes(routes!(
            persona::fields::get_transform,
            persona::fields::put_transform
        ))
        .routes(routes!(persona::spawn::spawn))
        .routes(routes!(persona::spawn::despawn))
        .routes(routes!(persona::events::events))
        .routes(routes!(persona::vrm::attach, persona::vrm::detach))
        .routes(routes!(
            persona::vrm::expressions::list_expressions,
            persona::vrm::expressions::modify_expressions,
            persona::vrm::expressions::clear_expressions
        ))
        .routes(routes!(persona::vrm::vrma::play_vrma))
        .routes(routes!(persona::vrm::vrma::stop_vrma))
        .routes(routes!(persona::vrm::vrma::get_vrma))
        .routes(routes!(persona::vrm::position::get_position))
        .routes(routes!(persona::vrm::bone::get_bone))
        .routes(routes!(persona::vrm::look::look_cursor))
        .routes(routes!(persona::vrm::look::look_target))
        .routes(routes!(persona::vrm::look::unlook))
        .routes(routes!(
            persona::vrm::spring_bones::list_spring_bones,
            persona::vrm::spring_bones::patch_spring_bones
        ))
        .routes(routes!(persona::vrm::speech::speech_timeline))
        .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024))
}

fn coordinates_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(coordinates::world_2d::world_2d))
        .routes(routes!(coordinates::global_viewport::global_viewport))
}

fn settings_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new().routes(routes!(settings::get, settings::put))
}

fn shadow_panel_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new().routes(routes!(shadow_panel::alpha::get, shadow_panel::alpha::put))
}

fn webviews_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(webviews::list, webviews::open))
        .routes(routes!(webviews::get, webviews::patch, webviews::delete))
        .routes(routes!(webviews::is_closed))
        .routes(routes!(webviews::navigate))
        .routes(routes!(webviews::navigate_back))
        .routes(routes!(webviews::navigate_forward))
        .routes(routes!(webviews::reload))
        .routes(routes!(
            webviews::get_linked_persona,
            webviews::set_linked_persona,
            webviews::unlink_persona
        ))
}

fn preferences_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(preferences::list))
        .routes(routes!(preferences::load, preferences::save))
}

fn display_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new().routes(routes!(displays::all))
}

fn signals_router() -> OpenApiRouter<HttpState> {
    OpenApiRouter::new()
        .routes(routes!(route::signals::list))
        .routes(routes!(route::signals::send))
        .route("/ws", axum::routing::get(route::signals::ws_handler))
}

#[cfg(test)]
mod tests {
    use crate::create_router;
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode};
    use bevy::log::LogPlugin;
    use bevy::prelude::*;
    use bevy::tasks::{block_on, poll_once};
    use homunculus_api::HomunculusApiPlugin;
    use homunculus_api::prelude::{ApiReactor, ShadowPanelApiPlugin, WebviewApiPlugin};
    use homunculus_core::prelude::{
        AssetRegistry, ModInfo, ModMenuMetadata, ModMenuMetadataList, ModRegistry,
    };
    use homunculus_core::rpc_registry::RpcRegistry;
    use homunculus_prefs::PrefsDatabase;
    use homunculus_utils::config::HomunculusConfig;
    use homunculus_utils::prelude::{AssetDeclaration, AssetType};
    use http_body_util::BodyExt;
    use serde::de::DeserializeOwned;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::path::PathBuf;
    use std::sync::{Arc, RwLock};
    use tokio::pin;
    use tower::ServiceExt;

    pub fn test_app() -> (App, Router) {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            bevy_flurx::prelude::FlurxPlugin,
            TransformPlugin,
            WindowPlugin::default(),
            AssetPlugin::default(),
            ImagePlugin::default_linear(),
            HomunculusApiPlugin
                .build()
                .disable::<ShadowPanelApiPlugin>()
                .disable::<WebviewApiPlugin>(),
        ));

        app.insert_non_send_resource(PrefsDatabase::open_in_memory());
        app.init_resource::<AssetRegistry>();
        app.init_resource::<ModRegistry>();
        app.init_resource::<ModMenuMetadataList>();
        app.init_resource::<homunculus_core::prelude::PersonaIndex>();
        let config = HomunculusConfig::default();
        let rpc_registry = Arc::new(RwLock::new(RpcRegistry::default()));
        let router = create_router(
            app.world().resource::<ApiReactor>().clone(),
            config,
            rpc_registry,
        );
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

    #[test]
    fn test_health() {
        let (mut app, router) = test_app();
        let request = Request::get("/app/health").body(Body::empty()).unwrap();
        block_on(assert_response(
            &mut app,
            router,
            request,
            crate::route::HealthResponse {
                status: "ok".to_string(),
            },
        ));
    }

    #[test]
    fn test_list_mods_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/mods").body(Body::empty()).unwrap();
        block_on(assert_response::<Vec<ModInfo>>(
            &mut app,
            router,
            request,
            vec![],
        ));
    }

    #[test]
    fn test_list_mods_with_entries() {
        let (mut app, router) = test_app();
        app.world_mut()
            .resource_mut::<ModRegistry>()
            .register(ModInfo {
                name: "test-mod".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test mod".to_string()),
                author: None,
                license: None,
                service_script_path: Some(PathBuf::from("/main.js")),
                commands: vec!["build".to_string()],
                assets: HashMap::from([(
                    "test-asset".to_string(),
                    AssetDeclaration {
                        path: "test.vrm".to_string(),
                        asset_type: AssetType::Vrm,
                        description: None,
                    },
                )]),
                menus: vec![],
                tray: None,
                mod_dir: PathBuf::default(),
            });
        let request = Request::get("/mods").body(Body::empty()).unwrap();
        block_on(assert_response(
            &mut app,
            router,
            request,
            vec![ModInfo {
                name: "test-mod".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test mod".to_string()),
                author: None,
                license: None,
                service_script_path: Some(PathBuf::from("/main.js")),
                commands: vec!["build".to_string()],
                assets: HashMap::from([(
                    "test-asset".to_string(),
                    AssetDeclaration {
                        path: "test.vrm".to_string(),
                        asset_type: AssetType::Vrm,
                        description: None,
                    },
                )]),
                menus: vec![],
                tray: None,
                mod_dir: PathBuf::default(),
            }],
        ));
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

    pub async fn call_any_status(
        app: &mut App,
        router: Router,
        request: Request<Body>,
    ) -> Response<Body> {
        let h = router.oneshot(request);
        pin!(h);

        loop {
            app.update();
            if let Some(result) = block_on(poll_once(&mut h)) {
                return result.unwrap();
            }
        }
    }

    #[test]
    fn test_get_mod_by_name() {
        let (mut app, router) = test_app();
        app.world_mut()
            .resource_mut::<ModRegistry>()
            .register(ModInfo {
                name: "test-mod".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test mod".to_string()),
                author: None,
                license: None,
                service_script_path: Some(PathBuf::from("/main.js")),
                commands: vec!["build".to_string()],
                assets: HashMap::from([(
                    "test-asset".to_string(),
                    AssetDeclaration {
                        path: "test.vrm".to_string(),
                        asset_type: AssetType::Vrm,
                        description: None,
                    },
                )]),
                menus: vec![],
                tray: None,
                mod_dir: PathBuf::default(),
            });
        let request = Request::get("/mods/test-mod").body(Body::empty()).unwrap();
        block_on(assert_response(
            &mut app,
            router,
            request,
            ModInfo {
                name: "test-mod".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test mod".to_string()),
                author: None,
                license: None,
                service_script_path: Some(PathBuf::from("/main.js")),
                commands: vec!["build".to_string()],
                assets: HashMap::from([(
                    "test-asset".to_string(),
                    AssetDeclaration {
                        path: "test.vrm".to_string(),
                        asset_type: AssetType::Vrm,
                        description: None,
                    },
                )]),
                menus: vec![],
                tray: None,
                mod_dir: PathBuf::default(),
            },
        ));
    }

    #[test]
    fn test_get_mod_not_found() {
        let (mut app, router) = test_app();
        let request = Request::get("/mods/nonexistent")
            .body(Body::empty())
            .unwrap();
        block_on(async {
            let response = call_any_status(&mut app, router, request).await;
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(json["error"], "Mod 'nonexistent' not found");
        });
    }

    #[test]
    fn test_list_menus_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/mods/menus").body(Body::empty()).unwrap();
        block_on(assert_response::<Vec<ModMenuMetadata>>(
            &mut app,
            router,
            request,
            vec![],
        ));
    }

    #[test]
    fn test_info_empty_mods() {
        let (mut app, router) = test_app();
        let request = Request::get("/app/info").body(Body::empty()).unwrap();
        block_on(assert_response(
            &mut app,
            router,
            request,
            crate::route::info::AppInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
                platform: crate::route::info::PlatformInfo {
                    os: std::env::consts::OS.to_string(),
                    arch: std::env::consts::ARCH.to_string(),
                },
                features: crate::route::info::FEATURES
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect(),
                mods: vec![],
            },
        ));
    }

    #[test]
    fn test_info_with_mods() {
        let (mut app, router) = test_app();
        app.world_mut()
            .resource_mut::<ModRegistry>()
            .register(ModInfo {
                name: "test-mod".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test mod".to_string()),
                author: None,
                license: None,
                service_script_path: Some(PathBuf::from("/main.js")),
                commands: vec!["build".to_string()],
                assets: HashMap::from([(
                    "test-asset".to_string(),
                    AssetDeclaration {
                        path: "test.vrm".to_string(),
                        asset_type: AssetType::Vrm,
                        description: None,
                    },
                )]),
                menus: vec![],
                tray: None,
                mod_dir: PathBuf::default(),
            });
        let request = Request::get("/app/info").body(Body::empty()).unwrap();
        block_on(assert_response(
            &mut app,
            router,
            request,
            crate::route::info::AppInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
                platform: crate::route::info::PlatformInfo {
                    os: std::env::consts::OS.to_string(),
                    arch: std::env::consts::ARCH.to_string(),
                },
                features: crate::route::info::FEATURES
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect(),
                mods: vec![ModInfo {
                    name: "test-mod".to_string(),
                    version: "1.0.0".to_string(),
                    description: Some("A test mod".to_string()),
                    author: None,
                    license: None,
                    service_script_path: Some(PathBuf::from("/main.js")),
                    commands: vec!["build".to_string()],
                    assets: HashMap::from([(
                        "test-asset".to_string(),
                        AssetDeclaration {
                            path: "test.vrm".to_string(),
                            asset_type: AssetType::Vrm,
                            description: None,
                        },
                    )]),
                    menus: vec![],
                    tray: None,
                    mod_dir: PathBuf::default(),
                }],
            },
        ));
    }

    #[test]
    fn test_list_menus_with_entries() {
        let (mut app, router) = test_app();
        app.world_mut()
            .resource_mut::<ModMenuMetadataList>()
            .push(ModMenuMetadata {
                id: "greet".to_string(),
                mod_name: "elmer".to_string(),
                text: "Greet".to_string(),
                command: "speak".to_string(),
            });
        let request = Request::get("/mods/menus").body(Body::empty()).unwrap();
        block_on(assert_response(
            &mut app,
            router,
            request,
            vec![ModMenuMetadata {
                id: "greet".to_string(),
                mod_name: "elmer".to_string(),
                text: "Greet".to_string(),
                command: "speak".to_string(),
            }],
        ));
    }

    #[test]
    fn test_list_signals_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/signals").body(Body::empty()).unwrap();
        block_on(assert_response::<Vec<homunculus_api::prelude::SignalInfo>>(
            &mut app,
            router,
            request,
            vec![],
        ));
    }

    #[test]
    fn test_get_asset_file_returns_file_content() {
        let (mut app, router) = test_app();

        // Create a temp file with known content
        let tmp = std::env::temp_dir().join("test_asset_file.png");
        std::fs::write(&tmp, b"fake-png-content").unwrap();

        // Register an asset pointing to the temp file
        app.world_mut().resource_mut::<AssetRegistry>().register(
            homunculus_core::prelude::AssetEntry {
                id: homunculus_utils::prelude::AssetId::new("test-mod:my-image"),
                path: PathBuf::from("my-image.png"),
                absolute_path: tmp.clone(),
                asset_type: AssetType::Image,
                description: None,
                mod_name: "test-mod".to_string(),
            },
        );

        let request = Request::get("/assets/file?id=test-mod:my-image")
            .body(Body::empty())
            .unwrap();
        let response = block_on(call(&mut app, router, request));
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(content_type, "image/png");

        let nosniff = response
            .headers()
            .get("x-content-type-options")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(nosniff, "nosniff");

        let body = block_on(response.into_body().collect()).unwrap().to_bytes();
        assert_eq!(&body[..], b"fake-png-content");

        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    fn test_get_asset_file_not_found() {
        let (mut app, router) = test_app();
        let request = Request::get("/assets/file?id=nonexistent:asset")
            .body(Body::empty())
            .unwrap();
        let response = block_on(call_any_status(&mut app, router, request));
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_get_asset_file_missing_id() {
        let (mut app, router) = test_app();
        let request = Request::get("/assets/file").body(Body::empty()).unwrap();
        let response = block_on(call_any_status(&mut app, router, request));
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_get_asset_file_imported_asset() {
        let (mut app, router) = test_app();

        let tmp = std::env::temp_dir().join("test_imported_asset.jpg");
        std::fs::write(&tmp, b"fake-jpg-data").unwrap();

        // Use register_imported (same path as POST /assets/import)
        app.world_mut()
            .resource_mut::<AssetRegistry>()
            .register_imported(homunculus_core::prelude::AssetEntry {
                id: homunculus_utils::prelude::AssetId::new("vrm:local:my-persona"),
                path: PathBuf::from("vrm_local_my-persona.jpg"),
                absolute_path: tmp.clone(),
                asset_type: AssetType::Image,
                description: None,
                mod_name: "local".to_string(),
            });

        let request = Request::get("/assets/file?id=vrm:local:my-persona")
            .body(Body::empty())
            .unwrap();
        let response = block_on(call(&mut app, router, request));
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(content_type, "image/jpeg");

        let body = block_on(response.into_body().collect()).unwrap().to_bytes();
        assert_eq!(&body[..], b"fake-jpg-data");

        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    fn test_get_asset_file_missing_file_on_disk() {
        let (mut app, router) = test_app();

        // Register asset with a path that does not exist on disk
        app.world_mut().resource_mut::<AssetRegistry>().register(
            homunculus_core::prelude::AssetEntry {
                id: homunculus_utils::prelude::AssetId::new("test-mod:ghost"),
                path: PathBuf::from("ghost.png"),
                absolute_path: PathBuf::from("/tmp/does_not_exist_12345.png"),
                asset_type: AssetType::Image,
                description: None,
                mod_name: "test-mod".to_string(),
            },
        );

        let request = Request::get("/assets/file?id=test-mod:ghost")
            .body(Body::empty())
            .unwrap();
        let response = block_on(call_any_status(&mut app, router, request));
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_list_signals_with_channels() {
        let (mut app, router) = test_app();

        // Send a signal to create a channel
        let request = Request::post("/signals/test-channel")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"hello":"world"}"#))
            .unwrap();
        block_on(async {
            let response = call(&mut app, router.clone(), request).await;
            assert_eq!(response.status(), StatusCode::OK);
        });

        // List signals — should contain the channel we just created
        let request = Request::get("/signals").body(Body::empty()).unwrap();
        block_on(async {
            let response = call(&mut app, router, request).await;
            assert_eq!(response.status(), StatusCode::OK);
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let signals: Vec<homunculus_api::prelude::SignalInfo> =
                serde_json::from_slice(&body).unwrap();
            assert_eq!(signals.len(), 1);
            assert_eq!(signals[0].signal, "test-channel");
            // subscribers should be 0 (no active SSE listeners)
            assert_eq!(signals[0].subscribers, 0);
        });
    }
}
