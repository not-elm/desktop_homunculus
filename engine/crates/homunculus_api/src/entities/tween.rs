use bevy::math::curve::easing::EaseFunction;
use bevy::prelude::{Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// Easing functions for tweening animations.
///
/// Defines the rate of change of a parameter over time, allowing smooth transitions
/// between values. Each variant represents a different mathematical curve that controls
/// how the interpolation progresses from start to end.
///
/// The naming convention follows: `{CurveName}{Direction}` where:
/// - `In`: Slow start, fast end
/// - `Out`: Fast start, slow end
/// - `InOut`: Slow start and end, fast middle
///
/// # Example
/// ```
/// use homunculus_api::entities::tween::EasingFunction;
///
/// let easing = EasingFunction::QuadraticInOut;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum EasingFunction {
    /// Linear interpolation (constant rate of change)
    #[default]
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SmoothStepIn,
    SmoothStepOut,
    SmoothStep,
    SmootherStepIn,
    SmootherStepOut,
    SmootherStep,
    SineIn,
    SineOut,
    SineInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

/// Request arguments for tweening an entity's position.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TweenPositionArgs {
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 3]))]
    pub target: Vec3,
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

/// Request arguments for tweening an entity's rotation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TweenRotationArgs {
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 4]))]
    pub target: Quat,
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

/// Request arguments for tweening an entity's scale.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TweenScaleArgs {
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 3]))]
    pub target: Vec3,
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

/// Request arguments for tweening an entity's position using viewport coordinates.
///
/// Viewport coordinates are in pixels relative to the primary monitor (0,0 = top-left).
/// They are converted to world coordinates internally.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TweenPositionViewportArgs {
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 2]))]
    pub position: Vec2,
    pub duration_ms: u64,
    #[serde(default)]
    pub easing: EasingFunction,
    #[serde(default)]
    pub wait: bool,
}

impl From<EasingFunction> for EaseFunction {
    fn from(ef: EasingFunction) -> Self {
        match ef {
            EasingFunction::Linear => EaseFunction::Linear,
            EasingFunction::QuadraticIn => EaseFunction::QuadraticIn,
            EasingFunction::QuadraticOut => EaseFunction::QuadraticOut,
            EasingFunction::QuadraticInOut => EaseFunction::QuadraticInOut,
            EasingFunction::CubicIn => EaseFunction::CubicIn,
            EasingFunction::CubicOut => EaseFunction::CubicOut,
            EasingFunction::CubicInOut => EaseFunction::CubicInOut,
            EasingFunction::QuarticIn => EaseFunction::QuarticIn,
            EasingFunction::QuarticOut => EaseFunction::QuarticOut,
            EasingFunction::QuarticInOut => EaseFunction::QuarticInOut,
            EasingFunction::QuinticIn => EaseFunction::QuinticIn,
            EasingFunction::QuinticOut => EaseFunction::QuinticOut,
            EasingFunction::QuinticInOut => EaseFunction::QuinticInOut,
            EasingFunction::SmoothStepIn => EaseFunction::SmoothStepIn,
            EasingFunction::SmoothStepOut => EaseFunction::SmoothStepOut,
            EasingFunction::SmoothStep => EaseFunction::SmoothStep,
            EasingFunction::SmootherStepIn => EaseFunction::SmootherStepIn,
            EasingFunction::SmootherStepOut => EaseFunction::SmootherStepOut,
            EasingFunction::SmootherStep => EaseFunction::SmootherStep,
            EasingFunction::SineIn => EaseFunction::SineIn,
            EasingFunction::SineOut => EaseFunction::SineOut,
            EasingFunction::SineInOut => EaseFunction::SineInOut,
            EasingFunction::CircularIn => EaseFunction::CircularIn,
            EasingFunction::CircularOut => EaseFunction::CircularOut,
            EasingFunction::CircularInOut => EaseFunction::CircularInOut,
            EasingFunction::ExponentialIn => EaseFunction::ExponentialIn,
            EasingFunction::ExponentialOut => EaseFunction::ExponentialOut,
            EasingFunction::ExponentialInOut => EaseFunction::ExponentialInOut,
            EasingFunction::ElasticIn => EaseFunction::ElasticIn,
            EasingFunction::ElasticOut => EaseFunction::ElasticOut,
            EasingFunction::ElasticInOut => EaseFunction::ElasticInOut,
            EasingFunction::BackIn => EaseFunction::BackIn,
            EasingFunction::BackOut => EaseFunction::BackOut,
            EasingFunction::BackInOut => EaseFunction::BackInOut,
            EasingFunction::BounceIn => EaseFunction::BounceIn,
            EasingFunction::BounceOut => EaseFunction::BounceOut,
            EasingFunction::BounceInOut => EaseFunction::BounceInOut,
        }
    }
}

use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_tweening::{
    EaseMethod, Tween, TweenAnim,
    lens::{TransformPositionLens, TransformRotationLens, TransformScaleLens},
};
use homunculus_core::prelude::{Coordinate, GlobalViewport};
use std::time::Duration;

fn apply_position_tween(
    In((entity, args)): In<(Entity, TweenPositionArgs)>,
    transforms: Query<&Transform>,
    mut commands: Commands,
) -> ApiResult {
    if args.duration_ms == 0 {
        return Err(ApiError::InvalidInput(
            "duration must be greater than 0".into(),
        ));
    }

    let current = transforms
        .get(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    let ease_function: EaseFunction = args.easing.into();
    let ease_method: EaseMethod = ease_function.into();

    let tween = Tween::new(
        ease_method,
        Duration::from_millis(args.duration_ms),
        TransformPositionLens {
            start: current.translation,
            end: args.target,
        },
    );

    commands.entity(entity).try_insert(TweenAnim::new(tween));
    Ok(())
}

fn apply_position_tween_viewport(
    In((entity, args)): In<(Entity, TweenPositionViewportArgs)>,
    coordinate: Coordinate,
    transforms: Query<&Transform>,
    mut commands: Commands,
) -> ApiResult {
    if args.duration_ms == 0 {
        return Err(ApiError::InvalidInput(
            "duration must be greater than 0".into(),
        ));
    }

    let current = transforms
        .get(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    let world_pos = coordinate
        .to_world_2d_by_global(GlobalViewport(args.position))
        .ok_or(ApiError::FailedToWorldPosition)?;

    let target = Vec3::new(world_pos.x, world_pos.y, current.translation.z);

    let ease_function: EaseFunction = args.easing.into();
    let ease_method: EaseMethod = ease_function.into();

    let tween = Tween::new(
        ease_method,
        Duration::from_millis(args.duration_ms),
        TransformPositionLens {
            start: current.translation,
            end: target,
        },
    );

    commands.entity(entity).try_insert(TweenAnim::new(tween));
    Ok(())
}

fn apply_rotation_tween(
    In((entity, args)): In<(Entity, TweenRotationArgs)>,
    transforms: Query<&Transform>,
    mut commands: Commands,
) -> ApiResult {
    if args.duration_ms == 0 {
        return Err(ApiError::InvalidInput(
            "duration must be greater than 0".into(),
        ));
    }

    let current = transforms
        .get(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    let ease_function: EaseFunction = args.easing.into();
    let ease_method: EaseMethod = ease_function.into();

    let tween = Tween::new(
        ease_method,
        Duration::from_millis(args.duration_ms),
        TransformRotationLens {
            start: current.rotation,
            end: args.target,
        },
    );

    commands.entity(entity).try_insert(TweenAnim::new(tween));
    Ok(())
}

fn apply_scale_tween(
    In((entity, args)): In<(Entity, TweenScaleArgs)>,
    transforms: Query<&Transform>,
    mut commands: Commands,
) -> ApiResult {
    if args.duration_ms == 0 {
        return Err(ApiError::InvalidInput(
            "duration must be greater than 0".into(),
        ));
    }

    let current = transforms
        .get(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    let ease_function: EaseFunction = args.easing.into();
    let ease_method: EaseMethod = ease_function.into();

    let tween = Tween::new(
        ease_method,
        Duration::from_millis(args.duration_ms),
        TransformScaleLens {
            start: current.scale,
            end: args.target,
        },
    );

    commands.entity(entity).try_insert(TweenAnim::new(tween));
    Ok(())
}

use crate::entities::EntitiesApi;
use bevy_flurx::prelude::*;

impl EntitiesApi {
    /// Tweens the entity's position to a target value over a specified duration.
    ///
    /// # Arguments
    /// * `entity` - The entity to animate
    /// * `args` - Tween parameters including target position, duration, easing function, and wait flag
    ///
    /// # Example
    /// ```no_run
    /// use bevy::prelude::*;
    /// use homunculus_api::entities::{EntitiesApi, TweenPositionArgs, EasingFunction};
    ///
    /// # async fn example(api: EntitiesApi, entity: Entity) -> Result<(), Box<dyn std::error::Error>> {
    /// api.tween_position(
    ///     entity,
    ///     TweenPositionArgs {
    ///         target: Vec3::new(100.0, 50.0, 0.0),
    ///         duration_ms: 1000,
    ///         easing: EasingFunction::QuadraticInOut,
    ///         wait: true,
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tween_position(&self, entity: Entity, args: TweenPositionArgs) -> ApiResult {
        let wait_duration = args.duration_ms;
        let should_wait = args.wait;

        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(apply_position_tween).with((entity, args)))
                    .await?;

                if should_wait {
                    task.will(
                        Update,
                        delay::time().with(Duration::from_millis(wait_duration)),
                    )
                    .await;
                }

                Ok(())
            })
            .await?
    }

    /// Tweens the entity's position using viewport coordinates.
    ///
    /// Viewport coordinates (pixels, 0,0 = top-left of primary monitor) are converted
    /// to world coordinates before applying the tween. The entity's z-position is preserved.
    pub async fn tween_position_viewport(
        &self,
        entity: Entity,
        args: TweenPositionViewportArgs,
    ) -> ApiResult {
        let wait_duration = args.duration_ms;
        let should_wait = args.wait;

        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(apply_position_tween_viewport).with((entity, args)),
                )
                .await?;

                if should_wait {
                    task.will(
                        Update,
                        delay::time().with(Duration::from_millis(wait_duration)),
                    )
                    .await;
                }

                Ok(())
            })
            .await?
    }

    /// Tweens the entity's rotation to a target value over a specified duration.
    ///
    /// # Arguments
    /// * `entity` - The entity to animate
    /// * `args` - Tween parameters including target rotation, duration, easing function, and wait flag
    ///
    /// # Example
    /// ```no_run
    /// use bevy::prelude::*;
    /// use homunculus_api::entities::{EntitiesApi, TweenRotationArgs, EasingFunction};
    ///
    /// # async fn example(api: EntitiesApi, entity: Entity) -> Result<(), Box<dyn std::error::Error>> {
    /// api.tween_rotation(
    ///     entity,
    ///     TweenRotationArgs {
    ///         target: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
    ///         duration_ms: 500,
    ///         easing: EasingFunction::CubicInOut,
    ///         wait: false,
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tween_rotation(&self, entity: Entity, args: TweenRotationArgs) -> ApiResult {
        let wait_duration = args.duration_ms;
        let should_wait = args.wait;

        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(apply_rotation_tween).with((entity, args)))
                    .await?;

                if should_wait {
                    task.will(
                        Update,
                        delay::time().with(Duration::from_millis(wait_duration)),
                    )
                    .await;
                }

                Ok(())
            })
            .await?
    }

    /// Tweens the entity's scale to a target value over a specified duration.
    ///
    /// # Arguments
    /// * `entity` - The entity to animate
    /// * `args` - Tween parameters including target scale, duration, easing function, and wait flag
    ///
    /// # Example
    /// ```no_run
    /// use bevy::prelude::*;
    /// use homunculus_api::entities::{EntitiesApi, TweenScaleArgs, EasingFunction};
    ///
    /// # async fn example(api: EntitiesApi, entity: Entity) -> Result<(), Box<dyn std::error::Error>> {
    /// api.tween_scale(
    ///     entity,
    ///     TweenScaleArgs {
    ///         target: Vec3::splat(2.0),
    ///         duration_ms: 800,
    ///         easing: EasingFunction::ElasticOut,
    ///         wait: true,
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tween_scale(&self, entity: Entity, args: TweenScaleArgs) -> ApiResult {
        let wait_duration = args.duration_ms;
        let should_wait = args.wait;

        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(apply_scale_tween).with((entity, args)))
                    .await?;

                if should_wait {
                    task.will(
                        Update,
                        delay::time().with(Duration::from_millis(wait_duration)),
                    )
                    .await;
                }

                Ok(())
            })
            .await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_function_serde() {
        let json = r#""quadraticInOut""#;
        let easing: EasingFunction = serde_json::from_str(json).unwrap();
        assert!(matches!(easing, EasingFunction::QuadraticInOut));

        let serialized = serde_json::to_string(&easing).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn test_easing_function_default() {
        let easing = EasingFunction::default();
        assert!(matches!(easing, EasingFunction::Linear));
    }

    #[test]
    fn test_smooth_step_variants() {
        let json = r#""smoothStep""#;
        let easing: EasingFunction = serde_json::from_str(json).unwrap();
        assert!(matches!(easing, EasingFunction::SmoothStep));

        let json = r#""smootherStepIn""#;
        let easing: EasingFunction = serde_json::from_str(json).unwrap();
        assert!(matches!(easing, EasingFunction::SmootherStepIn));
    }

    #[test]
    fn test_conversion_to_bevy() {
        // Test that our enum converts correctly to Bevy's EaseFunction
        let our_easing = EasingFunction::SmoothStep;
        let bevy_easing: EaseFunction = our_easing.into();
        assert!(matches!(bevy_easing, EaseFunction::SmoothStep));

        let our_easing = EasingFunction::SmootherStepOut;
        let bevy_easing: EaseFunction = our_easing.into();
        assert!(matches!(bevy_easing, EaseFunction::SmootherStepOut));
    }

    #[test]
    fn test_tween_position_args_serde() {
        let json = r#"{
        "target": [100.0, 50.0, 0.0],
        "durationMs": 1000,
        "easing": "quadraticInOut",
        "wait": true
    }"#;
        let args: TweenPositionArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.target, bevy::prelude::Vec3::new(100.0, 50.0, 0.0));
        assert_eq!(args.duration_ms, 1000);
        assert!(matches!(args.easing, EasingFunction::QuadraticInOut));
        assert_eq!(args.wait, true);
    }

    #[test]
    fn test_tween_rotation_args_serde() {
        let json = r#"{
        "target": [0.0, 0.707, 0.0, 0.707],
        "durationMs": 500
    }"#;
        let args: TweenRotationArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.duration_ms, 500);
        assert!(!args.wait); // default is false
    }

    #[test]
    fn test_tween_scale_args_serde() {
        let json = r#"{
        "target": [2.0, 2.0, 2.0],
        "durationMs": 300
    }"#;
        let args: TweenScaleArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.target, bevy::prelude::Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_apply_position_tween_system() {
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::*;

        let mut app = App::new();
        let entity = app.world_mut().spawn(Transform::default()).id();

        let args = TweenPositionArgs {
            target: Vec3::new(100.0, 50.0, 0.0),
            duration_ms: 1000,
            easing: EasingFunction::Linear,
            wait: false,
        };

        let result = app
            .world_mut()
            .run_system_once_with(apply_position_tween, (entity, args));

        // The system returns ApiResult, which is wrapped by Bevy
        let api_result = result.unwrap();
        assert!(api_result.is_ok());
    }

    #[test]
    fn test_apply_position_tween_entity_not_found() {
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::*;

        let mut app = App::new();
        let fake_entity = Entity::from_bits(99999);

        let args = TweenPositionArgs {
            target: Vec3::ZERO,
            duration_ms: 1000,
            easing: EasingFunction::Linear,
            wait: false,
        };

        let result = app
            .world_mut()
            .run_system_once_with(apply_position_tween, (fake_entity, args));

        // The system returns ApiResult, which is wrapped by Bevy
        let api_result = result.unwrap();
        assert!(api_result.is_err());
    }

    #[test]
    fn test_apply_position_tween_zero_duration() {
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::*;

        let mut app = App::new();
        let entity = app.world_mut().spawn(Transform::default()).id();

        let args = TweenPositionArgs {
            target: Vec3::ZERO,
            duration_ms: 0,
            easing: EasingFunction::Linear,
            wait: false,
        };

        let result = app
            .world_mut()
            .run_system_once_with(apply_position_tween, (entity, args));

        // The system returns ApiResult, which is wrapped by Bevy
        let api_result = result.unwrap();
        assert!(api_result.is_err());
    }

    #[test]
    fn test_apply_rotation_tween_system() {
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::*;

        let mut app = App::new();
        let entity = app.world_mut().spawn(Transform::default()).id();

        let args = TweenRotationArgs {
            target: Quat::from_rotation_y(1.57), // 90 degrees
            duration_ms: 500,
            easing: EasingFunction::SineInOut,
            wait: false,
        };

        let result = app
            .world_mut()
            .run_system_once_with(apply_rotation_tween, (entity, args));

        let api_result = result.unwrap();
        assert!(api_result.is_ok());
    }

    #[test]
    fn test_apply_scale_tween_system() {
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::*;

        let mut app = App::new();
        let entity = app.world_mut().spawn(Transform::default()).id();

        let args = TweenScaleArgs {
            target: Vec3::splat(2.0),
            duration_ms: 300,
            easing: EasingFunction::BounceOut,
            wait: false,
        };

        let result = app
            .world_mut()
            .run_system_once_with(apply_scale_tween, (entity, args));

        let api_result = result.unwrap();
        assert!(api_result.is_ok());
    }
}
