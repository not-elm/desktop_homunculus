//! Transform tool implementations for the MCP handler.

use bevy::math::{Quat, Vec2, Vec3};
use homunculus_api::entities::MoveTarget;
use homunculus_api::entities::tween::{
    EasingFunction, TweenPositionArgs, TweenRotationArgs, TweenScaleArgs,
};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};

use super::super::HomunculusMcpHandler;

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

/// Parameters for the `move_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MoveCharacterParams {
    /// Viewport x position in pixels (0 = left edge of primary monitor).
    pub x: f32,
    /// Viewport y position in pixels (0 = top edge of primary monitor).
    pub y: f32,
}

/// Parameters for the `tween_position` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweenPositionParams {
    /// Target x position.
    pub target_x: f32,
    /// Target y position.
    pub target_y: f32,
    /// Target z position.
    pub target_z: f32,
    /// Duration of the tween in milliseconds.
    pub duration_ms: u64,
    /// Easing function name (default: "linear"). See EasingFunction for available options.
    pub easing: Option<String>,
    /// Whether to wait for the tween to finish before returning (default: false).
    pub wait: Option<bool>,
}

/// Parameters for the `tween_rotation` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweenRotationParams {
    /// Target rotation quaternion x component.
    pub target_x: f32,
    /// Target rotation quaternion y component.
    pub target_y: f32,
    /// Target rotation quaternion z component.
    pub target_z: f32,
    /// Target rotation quaternion w component.
    pub target_w: f32,
    /// Duration of the tween in milliseconds.
    pub duration_ms: u64,
    /// Easing function name (default: "linear"). See EasingFunction for available options.
    pub easing: Option<String>,
    /// Whether to wait for the tween to finish before returning (default: false).
    pub wait: Option<bool>,
}

/// Parameters for the `tween_scale` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweenScaleParams {
    /// Target x scale.
    pub target_x: f32,
    /// Target y scale.
    pub target_y: f32,
    /// Target z scale.
    pub target_z: f32,
    /// Duration of the tween in milliseconds.
    pub duration_ms: u64,
    /// Easing function name (default: "linear"). See EasingFunction for available options.
    pub easing: Option<String>,
    /// Whether to wait for the tween to finish before returning (default: false).
    pub wait: Option<bool>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parses an easing function name string into an [`EasingFunction`].
///
/// Falls back to [`EasingFunction::Linear`] if the string is not recognised.
fn parse_easing(easing: &Option<String>) -> EasingFunction {
    match easing {
        Some(s) => serde_json::from_value::<EasingFunction>(serde_json::Value::String(s.clone()))
            .unwrap_or_default(),
        None => EasingFunction::default(),
    }
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = transform_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Move the active character to a screen position.
    #[tool(
        name = "move_character",
        description = "Move the active character to a screen position. Coordinates are in viewport pixels (0,0 = top-left of primary monitor)."
    )]
    async fn move_character(&self, params: Parameters<MoveCharacterParams>) -> String {
        let args = params.0;

        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let target = MoveTarget::Viewport {
            position: Vec2::new(args.x, args.y),
        };

        match self.entities_api.move_to(entity, target).await {
            Ok(()) => format!("Moved character to ({}, {})", args.x, args.y),
            Err(e) => format!("Error moving character: {e}"),
        }
    }

    /// Smoothly animate an entity's position to a target value over time.
    #[tool(
        name = "tween_position",
        description = "Smoothly animate an entity's position to a target value over time. Use this for smooth character movement and animations."
    )]
    async fn tween_position(&self, params: Parameters<TweenPositionParams>) -> String {
        let args = params.0;

        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let easing = parse_easing(&args.easing);
        let tween_args = TweenPositionArgs {
            target: Vec3::new(args.target_x, args.target_y, args.target_z),
            duration_ms: args.duration_ms,
            easing,
            wait: args.wait.unwrap_or(false),
        };

        match self.entities_api.tween_position(entity, tween_args).await {
            Ok(()) => "Tweening position".to_string(),
            Err(e) => format!("Error tweening position: {e}"),
        }
    }

    /// Smoothly animate an entity's rotation to a target value over time.
    #[tool(
        name = "tween_rotation",
        description = "Smoothly animate an entity's rotation to a target value over time. Rotation is specified as a quaternion (x, y, z, w)."
    )]
    async fn tween_rotation(&self, params: Parameters<TweenRotationParams>) -> String {
        let args = params.0;

        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let easing = parse_easing(&args.easing);
        let tween_args = TweenRotationArgs {
            target: Quat::from_xyzw(args.target_x, args.target_y, args.target_z, args.target_w),
            duration_ms: args.duration_ms,
            easing,
            wait: args.wait.unwrap_or(false),
        };

        match self.entities_api.tween_rotation(entity, tween_args).await {
            Ok(()) => "Tweening rotation".to_string(),
            Err(e) => format!("Error tweening rotation: {e}"),
        }
    }

    /// Smoothly animate an entity's scale to a target value over time.
    #[tool(
        name = "tween_scale",
        description = "Smoothly animate an entity's scale to a target value over time. Use this for grow/shrink effects and size animations."
    )]
    async fn tween_scale(&self, params: Parameters<TweenScaleParams>) -> String {
        let args = params.0;

        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let easing = parse_easing(&args.easing);
        let tween_args = TweenScaleArgs {
            target: Vec3::new(args.target_x, args.target_y, args.target_z),
            duration_ms: args.duration_ms,
            easing,
            wait: args.wait.unwrap_or(false),
        };

        match self.entities_api.tween_scale(entity, tween_args).await {
            Ok(()) => "Tweening scale".to_string(),
            Err(e) => format!("Error tweening scale: {e}"),
        }
    }
}
