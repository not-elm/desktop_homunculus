# Bevy & ECS Patterns

## Plugin Architecture

- Each crate is a self-contained Bevy plugin registering its own systems, events, and resources.
- Plugin composition happens in `src/main.rs`. `HomunculusModPlugin` must be added first.
- The `develop` feature flag enables `bevy_egui` inspector and CEF debug mode.

## Three-Layer Architecture

New features follow the Core → API → HTTP layering:

1. **Core** (`homunculus_core`): Shared components, events, resources, system parameters.
2. **API** (`homunculus_api`): Domain-specific async APIs bridging HTTP with Bevy ECS.
3. **HTTP** (`homunculus_http_server`): Axum routes exposing the API layer.

## ApiReactor Pattern

The central pattern for bridging async HTTP handlers with Bevy's single-threaded ECS.

### Creating a new API type

Use the `api!` macro:

```rust
api!(pub MyApi);

impl MyApi {
    pub async fn do_something(&self, args: MyArgs) -> ApiResult<MyResult> {
        self.0.schedule(move |task| async move {
            task.will(Update, once::run(my_system).with(args)).await
        }).await
    }
}
```

### Creating a route handler

```rust
pub async fn my_handler(
    State(api): State<MyApi>,
    Json(body): Json<MyRequest>,
) -> HttpResult<MyResponse> {
    api.do_something(body).await.into_http_result()
}
```

### Registering

- Add the route in `homunculus_http_server/src/lib.rs` → `create_router()`.
- Add the API resource to `HttpState` (implements `FromRef` for all domain APIs).

## Event Channels

- Use `VrmEventSender<E>` / `VrmEventReceiver<E>` for async broadcast events.
- Events use `async-broadcast` with capacity 256 and overflow enabled.
- Log send failures with `.output_log_if_error()`.

## System Parameters

Use custom `SystemParam` types from `homunculus_core::system_param::prelude`:
- `Coordinate`, `MascotTracker`, `BoneOffsets`, `Monitors`, `VrmAabb`, `AssetResolver`.

## Testing

- Use the HTTP test framework: `test_app()`, `call()`, `assert_response()`.
- Tests sync async HTTP with Bevy's synchronous `app.update()` loop via polling.
- Run tests: `cargo test --workspace` or `cargo test -p <crate_name>`.
