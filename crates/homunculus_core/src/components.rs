use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Loading;

/// The marker component that attaches to the entity of the window used in the app.
/// Windows are generated for each monitor.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AppWindow;

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ShadowPanel;

/// Represents the global screen coordinates.
/// If there are multiple screens, the coordinates of the leftmost screen are used as the origin.
#[derive(Debug, Copy, Clone, PartialEq, Deref, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
#[repr(transparent)]
pub struct GlobalViewport(pub Vec2);

/// Cameras are spawned for each window.
/// This component is attached to the camera corresponding to the window that is the [`PrimaryWindow`](bevy::prelude::PrimaryWindow).
#[derive(Debug, Component, Eq, PartialEq, Copy, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PrimaryCamera;

/// Represents the state of the VRM model.
#[repr(transparent)]
#[derive(Debug, Component, Eq, PartialEq, Clone, Reflect, Serialize, Deserialize, Deref)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VrmState(pub String);

impl VrmState {
    pub const SITTING: &'static str = "sitting";
}

impl From<&str> for VrmState {
    fn from(state: &str) -> Self {
        Self(state.to_string())
    }
}

impl Default for VrmState {
    fn default() -> Self {
        Self("idle".to_string())
    }
}

pub struct CoreComponentsPlugin;

impl Plugin for CoreComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Loading>()
            .register_type::<ShadowPanel>()
            .register_type::<GlobalViewport>()
            .register_type::<PrimaryCamera>()
            .register_type::<VrmState>()
            .register_type::<AppWindow>();
    }
}
