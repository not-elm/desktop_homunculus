//! PostUpdate constraint system that corrects webview `GlobalTransform`s
//! after Bevy's standard transform propagation.
//!
//! Webviews are children of persona entities but should remain upright
//! (billboard-style) or only partially follow the parent's rotation.
//! This module provides the math (swing-twist decomposition) and the
//! Bevy systems that enforce those constraints every frame.

use bevy::prelude::*;
use bevy::transform::TransformSystems;
use homunculus_core::prelude::{LinkedPersona, TransformConstraint};

/// Bevy plugin that registers the PostUpdate constraint systems.
pub struct ConstraintPlugin;

impl Plugin for ConstraintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (auto_insert_constraints, apply_transform_constraints)
                .chain()
                .after(TransformSystems::Propagate),
        );
    }
}

/// Inserts a default `TransformConstraint` on every entity that just received `LinkedPersona`.
fn auto_insert_constraints(mut commands: Commands, query: Query<Entity, Added<LinkedPersona>>) {
    for entity in &query {
        commands
            .entity(entity)
            .try_insert(TransformConstraint::default());
    }
}

/// Overrides the propagated `GlobalTransform` of constrained child entities.
fn apply_transform_constraints(
    mut children: Query<(&ChildOf, &TransformConstraint, &mut GlobalTransform)>,
    parents: Query<&GlobalTransform, Without<TransformConstraint>>,
) {
    for (child_of, constraint, mut global) in &mut children {
        let Ok(parent_global) = parents.get(child_of.parent()) else {
            continue;
        };
        *global = compute_constrained_global(parent_global, constraint);
    }
}

// ---------------------------------------------------------------------------
// Math helpers
// ---------------------------------------------------------------------------

/// Builds a `GlobalTransform` that honours the given constraint relative to a parent.
///
/// The pipeline:
/// 1. Fast-path: `rotation_follow == 0` and `max_tilt_degrees == 0` → identity rotation.
/// 2. Slerp from identity toward the parent rotation by `rotation_follow`.
/// 3. Decompose into swing (tilt from Y-up) and twist (yaw around Y).
/// 4. Clamp swing to `max_tilt_degrees`, discard twist.
/// 5. Compute translation as `parent_T + constrained_R * intended_offset`.
/// 6. Scale is `Vec3::ONE` when `lock_scale` is true.
pub fn compute_constrained_global(
    parent_global: &GlobalTransform,
    constraint: &TransformConstraint,
) -> GlobalTransform {
    let parent_rot = parent_global.rotation();
    let parent_translation = parent_global.translation();
    let parent_scale = parent_global.scale();

    let rotation = compute_constrained_rotation(
        parent_rot,
        constraint.rotation_follow,
        constraint.max_tilt_degrees,
    );

    let scale = if constraint.lock_scale {
        Vec3::ONE
    } else {
        parent_scale
    };

    let translation = parent_translation + rotation * constraint.intended_offset;

    GlobalTransform::from(Transform {
        translation,
        rotation,
        scale,
    })
}

/// Computes the constrained rotation from a parent rotation.
fn compute_constrained_rotation(parent_rot: Quat, follow: f32, max_tilt_degrees: f32) -> Quat {
    if follow == 0.0 && max_tilt_degrees == 0.0 {
        return Quat::IDENTITY;
    }

    let blended = Quat::IDENTITY.slerp(parent_rot, follow.clamp(0.0, 1.0));
    let (swing, _twist) = swing_twist_y(blended);
    clamp_swing(swing, max_tilt_degrees)
}

/// Decomposes a quaternion into swing (tilt away from Y-up) and twist (yaw around Y).
///
/// Given `q`, we factor it as `q = swing * twist` where `twist` is a pure Y-axis rotation
/// and `swing` contains the remaining tilt.
///
/// # Near-perpendicular guard
///
/// When the rotation is close to 90° from Y (i.e. the Y component is near zero),
/// the twist extraction becomes numerically unstable.
/// In that case we return `(q, Quat::IDENTITY)` — all rotation is treated as swing.
pub fn swing_twist_y(q: Quat) -> (Quat, Quat) {
    // The twist around Y is extracted from the (w, y) components.
    if q.y.abs() < 1e-5 && q.w.abs() < 1e-5 {
        // Near-perpendicular: cannot reliably separate twist.
        return (q, Quat::IDENTITY);
    }

    let twist = Quat::from_xyzw(0.0, q.y, 0.0, q.w).normalize();
    let swing = q * twist.inverse();

    (swing, twist)
}

/// Clamps the swing angle to at most `max_degrees`.
///
/// Returns `Quat::IDENTITY` if `max_degrees <= 0.0` (no tilt allowed).
/// If the swing angle is already within limits, returns it unchanged.
pub fn clamp_swing(swing: Quat, max_degrees: f32) -> Quat {
    if max_degrees <= 0.0 {
        return Quat::IDENTITY;
    }

    let (axis, angle) = swing.to_axis_angle();
    let max_radians = max_degrees.to_radians();

    if angle <= max_radians {
        swing
    } else {
        Quat::from_axis_angle(axis, max_radians)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    const EPSILON: f32 = 1e-4;
    /// Looser tolerance for numerically sensitive decompositions.
    const LOOSE_EPSILON: f32 = 1e-2;

    /// Helper: angle in degrees between two quaternions.
    fn angle_between_deg(a: Quat, b: Quat) -> f32 {
        a.angle_between(b).to_degrees()
    }

    /// Helper: checks that a quaternion is approximately identity.
    fn assert_near_identity(q: Quat, label: &str) {
        assert_near_identity_eps(q, EPSILON, label);
    }

    /// Helper with configurable tolerance.
    fn assert_near_identity_eps(q: Quat, eps: f32, label: &str) {
        let angle = q.angle_between(Quat::IDENTITY);
        assert!(
            angle < eps,
            "{label}: expected near-identity, got angle {angle} rad ({} deg)",
            angle.to_degrees()
        );
    }

    // -----------------------------------------------------------------------
    // swing_twist_y
    // -----------------------------------------------------------------------

    #[test]
    fn swing_twist_identity() {
        let (swing, twist) = swing_twist_y(Quat::IDENTITY);
        assert_near_identity(swing, "swing");
        assert_near_identity(twist, "twist");
    }

    #[test]
    fn swing_twist_pure_yaw() {
        // 45° rotation around Y → all twist, no swing
        let yaw = Quat::from_rotation_y(PI / 4.0);
        let (swing, twist) = swing_twist_y(yaw);
        assert_near_identity(swing, "swing");
        assert!(
            angle_between_deg(twist, yaw) < 0.1,
            "twist should equal input yaw"
        );
    }

    #[test]
    fn swing_twist_pure_tilt() {
        // 30° rotation around X → pure swing, near-zero twist
        let tilt = Quat::from_rotation_x(PI / 6.0);
        let (swing, twist) = swing_twist_y(tilt);
        assert_near_identity(twist, "twist");
        assert!(
            angle_between_deg(swing, tilt) < 0.1,
            "swing should equal input tilt"
        );
    }

    #[test]
    fn swing_twist_near_perpendicular_guard() {
        // 90° rotation around X → twist extraction is numerically unstable.
        // The guard fires when both q.y and q.w are near zero; for a pure
        // X rotation q.y==0 so twist normalises to identity with small error.
        let tilt_90 = Quat::from_rotation_x(FRAC_PI_2);
        let (swing, twist) = swing_twist_y(tilt_90);
        assert_near_identity_eps(twist, LOOSE_EPSILON, "twist (near-perpendicular)");
        assert!(
            angle_between_deg(swing, tilt_90) < 0.1,
            "swing should capture all rotation"
        );
    }

    // -----------------------------------------------------------------------
    // clamp_swing
    // -----------------------------------------------------------------------

    #[test]
    fn clamp_swing_within_limit() {
        let swing = Quat::from_rotation_x(5f32.to_radians());
        let clamped = clamp_swing(swing, 10.0);
        assert!(
            angle_between_deg(clamped, swing) < 0.1,
            "should be unchanged"
        );
    }

    #[test]
    fn clamp_swing_exceeds_limit() {
        let swing = Quat::from_rotation_x(30f32.to_radians());
        let clamped = clamp_swing(swing, 10.0);
        let angle = clamped.angle_between(Quat::IDENTITY).to_degrees();
        assert!((angle - 10.0).abs() < 0.5, "expected ~10°, got {angle}°");
    }

    #[test]
    fn clamp_swing_zero_degrees_returns_identity() {
        let swing = Quat::from_rotation_x(15f32.to_radians());
        let clamped = clamp_swing(swing, 0.0);
        assert_near_identity(clamped, "clamped");
    }

    // -----------------------------------------------------------------------
    // compute_constrained_global
    // -----------------------------------------------------------------------

    #[test]
    fn billboard_mode_no_rotation() {
        // follow=0, lock_scale=true, parent rotated + scaled
        let parent = GlobalTransform::from(Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_rotation_y(PI / 3.0),
            scale: Vec3::splat(2.0),
        });
        let constraint = TransformConstraint {
            rotation_follow: 0.0,
            max_tilt_degrees: 0.0,
            lock_scale: true,
            intended_offset: Vec3::new(0.0, 1.0, 0.0),
        };

        let result = compute_constrained_global(&parent, &constraint);
        let t = result.compute_transform();

        assert_near_identity(t.rotation, "rotation");
        assert!(
            (t.scale - Vec3::ONE).length() < EPSILON,
            "scale should be ONE, got {:?}",
            t.scale
        );
        // Translation = parent_T + IDENTITY * offset = (1,2,3) + (0,1,0) = (1,3,3)
        let expected_t = Vec3::new(1.0, 3.0, 3.0);
        assert!(
            (t.translation - expected_t).length() < EPSILON,
            "expected translation {expected_t:?}, got {:?}",
            t.translation
        );
    }

    #[test]
    fn scale_locked_ignores_parent_scale() {
        let parent = GlobalTransform::from(Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(3.0),
        });
        let constraint = TransformConstraint {
            rotation_follow: 0.0,
            max_tilt_degrees: 0.0,
            lock_scale: true,
            intended_offset: Vec3::new(1.0, 0.0, 0.0),
        };

        let result = compute_constrained_global(&parent, &constraint);
        let t = result.compute_transform();

        assert!(
            (t.scale - Vec3::ONE).length() < EPSILON,
            "scale should be ONE, got {:?}",
            t.scale
        );
        // Offset should NOT be multiplied by parent scale
        let expected_t = Vec3::new(1.0, 0.0, 0.0);
        assert!(
            (t.translation - expected_t).length() < EPSILON,
            "expected translation {expected_t:?}, got {:?}",
            t.translation
        );
    }

    #[test]
    fn partial_follow_with_clamp() {
        // follow=1.0, maxTilt=10°, parent tilted 60° around X
        let parent = GlobalTransform::from(Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_rotation_x(60f32.to_radians()),
            scale: Vec3::ONE,
        });
        let constraint = TransformConstraint {
            rotation_follow: 1.0,
            max_tilt_degrees: 10.0,
            lock_scale: true,
            intended_offset: Vec3::ZERO,
        };

        let result = compute_constrained_global(&parent, &constraint);
        let t = result.compute_transform();

        let tilt_deg = t.rotation.angle_between(Quat::IDENTITY).to_degrees();
        assert!(tilt_deg <= 10.5, "expected tilt <= ~10°, got {tilt_deg}°");
    }
}
