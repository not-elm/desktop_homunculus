//! # Homunculus Sitting
//!
//! This crate provides "sitting" functionality for VRM mascot models, allowing
//! them to perch on the edges of windows and other applications on the desktop.
//!
//! ## Overview
//!
//! `homunculus_sitting` implements a sitting system where VRM models can be
//! positioned on the borders or frames of other application windows. The system
//! automatically tracks window movement and resizing, keeping the mascot properly
//! positioned relative to the target window.
//!
//! ## Key Features
//!
//! - **Window Attachment**: Mascots can sit on any detectable desktop window
//! - **Automatic Tracking**: Positions update automatically when windows move or resize
//! - **Sitting Animations**: Automatic VRMA animation playback while sitting
//! - **State Management**: Proper integration with VRM state system
//! - **Multi-Monitor Support**: Works across multiple displays and window arrangements
//!
//! ## How Sitting Works
//!
//! 1. **Window Detection**: The system detects when a mascot is dropped on a window
//! 2. **Position Calculation**: Calculates the relative position on the window frame
//! 3. **State Transition**: Changes VRM state to "sitting" and begins tracking
//! 4. **Animation Start**: Loads and plays the sitting animation
//! 5. **Continuous Tracking**: Updates position as the target window moves
//!
//! ## Integration
//!
//! The sitting system integrates with:
//! - [`homunculus_drag`]: Triggers sitting when mascots are dropped on windows
//! - [`homunculus_screen`]: Uses global window detection for target windows
//! - [`homunculus_core`]: Uses VRM state management and coordinate systems
//! - VRM animation system: Plays sitting animations automatically
//!
//! ## Animation System
//!
//! Sitting animations are driven by mods (e.g., `mods/elmer`) via the SSE
//! event system. The sitting crate handles position tracking independently
//! using a smoothed hips bone offset anchor (`HipsAnchor`) that prevents
//! position jumps during asynchronous animation transitions.

use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::*;
use homunculus_core::prelude::*;
use homunculus_screen::prelude::*;

/// Vertical adjustment factor for sitting position offset.
pub const SITTING_ADJUST: f32 = 0.9;

/// Smoothly interpolated hips bone Y offset for stable sitting position tracking.
///
/// Captures the hips Y offset at sitting start and exponentially converges
/// toward the live bone value. This prevents position jumps when the sitting
/// animation transitions asynchronously from mods.
#[derive(Debug, Default, Clone, Copy)]
struct HipsAnchor {
    offset_y: f32,
}

impl HipsAnchor {
    /// Exponential blend rate. Converges ~98% in 1 second.
    const BLEND_SPEED: f32 = 4.0;
    /// Snap immediately when difference exceeds this threshold (e.g., model swap).
    const SNAP_THRESHOLD: f32 = 0.3;

    fn new(initial: f32) -> Self {
        Self { offset_y: initial }
    }

    fn update(&mut self, live: f32, dt: f32) {
        if (self.offset_y - live).abs() > Self::SNAP_THRESHOLD {
            self.offset_y = live;
        } else {
            let alpha = 1.0 - (-Self::BLEND_SPEED * dt).exp();
            self.offset_y = self.offset_y.lerp(live, alpha);
        }
    }

    fn offset_y(&self) -> f32 {
        self.offset_y
    }
}

/// Plugin that provides sitting functionality for VRM mascot models.
///
/// This plugin enables mascots to "sit" on the edges of desktop windows,
/// automatically tracking window movement. The sitting behavior is triggered
/// when mascots are dropped on windows during drag operations.
///
/// # Systems
///
/// - `track_to_sitting_window`: Updates mascot position when target window moves or resizes.
///   Uses `HipsAnchor` to smoothly interpolate the hips bone offset, preventing position
///   jumps during asynchronous animation transitions from mods.
/// - `remove_sitting_window`: Cleans up sitting components when mascot leaves sitting state.
///
/// # Performance
///
/// Systems run conditionally using `run_if(any_mascots_sitting)` to avoid
/// unnecessary processing when no mascots are currently sitting.
pub struct HomunculusSittingPlugin;

impl Plugin for HomunculusSittingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (remove_sitting_window.run_if(any_mascots_sitting),))
            .add_systems(Update, track_to_sitting_window.run_if(any_mascots_sitting));
    }
}

fn any_mascots_sitting(mascots: Query<&VrmState>) -> bool {
    mascots
        .iter()
        .any(|state| state.as_str() == VrmState::SITTING)
}

#[derive(Debug, Default, Clone, Component)]
pub struct SittingWindow {
    pub window: GlobalWindow,
    pub mascot_viewport_offset: Vec2,
    hips_anchor: HipsAnchor,
}

impl SittingWindow {
    pub fn new(
        global_window: GlobalWindow,
        sitting_pos: GlobalViewport,
        hips_offset_y: f32,
    ) -> Self {
        Self {
            mascot_viewport_offset: *sitting_pos - global_window.frame.min,
            window: global_window,
            hips_anchor: HipsAnchor::new(hips_offset_y),
        }
    }

    #[inline]
    pub fn update(&self) -> Option<Self> {
        let new_window = self.window.update()?;
        Some(Self {
            window: new_window,
            ..*self
        })
    }

    #[inline]
    pub fn sitting_pos(&self) -> GlobalViewport {
        GlobalViewport(self.window.frame.min + self.mascot_viewport_offset)
    }

    /// Returns the smoothed hips offset Y value used for position tracking.
    #[inline]
    pub fn hips_offset_y(&self) -> f32 {
        self.hips_anchor.offset_y()
    }

    /// Updates the hips anchor toward the live bone offset.
    #[inline]
    pub fn update_hips_anchor(&mut self, live_hips_offset_y: f32, dt: f32) {
        self.hips_anchor.update(live_hips_offset_y, dt);
    }
}

fn track_to_sitting_window(
    mut commands: Commands,
    mut sitting_windows: Query<(Entity, &mut SittingWindow)>,
    tracker: MascotTracker,
    offsets: BoneOffsets,
    time: Res<Time>,
) {
    sitting_windows
        .iter_mut()
        .for_each(|(vrm_entity, mut sitting_window)| {
            // Update hips anchor FIRST so the copy inherits the updated value
            if let Some(live_offset) = offsets.hips_offset(vrm_entity) {
                sitting_window.update_hips_anchor(live_offset.y, time.delta_secs());
            }
            let Some(new_sitting_window) = sitting_window.update() else {
                return;
            };
            let hips_offset_y = new_sitting_window.hips_offset_y();
            let sitting_pos = new_sitting_window.sitting_pos();
            let Some(transform) = tracker.tracking_with_fixed_offset(
                vrm_entity,
                sitting_pos,
                hips_offset_y,
                SITTING_ADJUST,
            ) else {
                return;
            };
            commands
                .entity(vrm_entity)
                .try_insert((new_sitting_window, transform));
        });
}

fn remove_sitting_window(
    par_commands: ParallelCommands,
    mascots: Query<(Entity, &VrmState), (Changed<VrmState>, With<SittingWindow>)>,
) {
    mascots.par_iter().for_each(|(entity, state)| {
        if state.as_str() != VrmState::SITTING {
            par_commands.command_scope(|mut commands| {
                commands.entity(entity).remove::<SittingWindow>();
            });
        }
    });
}
