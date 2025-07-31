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
//! When a mascot enters sitting mode, the system:
//! - Loads a "sitting.vrma" animation file
//! - Plays the animation on loop while sitting
//! - Automatically stops the animation when leaving sitting mode

use bevy::animation::RepeatAnimation;
use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_vrm1::prelude::{Initialized, PlayVrma, Vrm};
use bevy_vrm1::vrma::VrmaHandle;
use homunculus_core::prelude::*;
use homunculus_screen::prelude::*;
use std::time::Duration;

/// Plugin that provides sitting functionality for VRM mascot models.
///
/// This plugin enables mascots to "sit" on the edges of desktop windows,
/// automatically tracking window movement and playing appropriate animations.
/// The sitting behavior is triggered when mascots are dropped on windows
/// during drag operations.
///
/// # Systems
///
/// The plugin adds several systems that manage sitting behavior:
///
/// - `spawn_sitting_vrma`: Loads sitting animation assets for newly initialized VRM models
/// - `play_sitting_vrma`: Starts sitting animation when VRM state changes to sitting
/// - `track_to_sitting_window`: Updates mascot position when target window moves or resizes
/// - `remove_sitting_window`: Cleans up sitting components when mascot leaves sitting state
///
/// # Performance
///
/// Most systems run conditionally using `run_if(any_mascots_sitting)` to avoid
/// unnecessary processing when no mascots are currently sitting.
///
/// # Animation Integration
///
/// The plugin automatically loads "vrma/sitting.vrma" animation files and
/// plays them when mascots enter sitting mode. Animations loop continuously
/// while sitting and stop when the mascot leaves sitting state.
pub struct HomunculusSittingPlugin;

impl Plugin for HomunculusSittingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_sitting_vrma,
                (remove_sitting_window, play_sitting_vrma).run_if(any_mascots_sitting),
            ),
        )
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
}

impl SittingWindow {
    pub fn new(global_window: GlobalWindow, sitting_pos: GlobalViewport) -> Self {
        Self {
            mascot_viewport_offset: *sitting_pos - global_window.frame.min,
            window: global_window,
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
}

#[derive(Component)]
struct SittingVrma(Entity);

fn spawn_sitting_vrma(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    vrms: Query<Entity, (With<Vrm>, Added<Initialized>)>,
) {
    for vrm in vrms.iter() {
        let vrma = commands
            .spawn(VrmaHandle(asset_server.load("vrma/sitting.vrma")))
            .id();
        commands
            .entity(vrm)
            .insert(SittingVrma(vrma))
            .add_child(vrma);
    }
}

fn play_sitting_vrma(
    mut commands: Commands,
    vrmas: Query<(&VrmState, &SittingVrma), Changed<VrmState>>,
) {
    for (state, vrma) in vrmas.iter() {
        if state.as_str() == VrmState::SITTING {
            commands.entity(vrma.0).trigger(PlayVrma {
                repeat: RepeatAnimation::Forever,
                transition_duration: Duration::from_millis(600),
            });
        }
    }
}

fn track_to_sitting_window(
    mut commands: Commands,
    sitting_windows: Query<(Entity, &SittingWindow)>,
    tracker: MascotTracker,
) {
    sitting_windows
        .iter()
        .for_each(|(vrm_entity, sitting_window)| {
            let Some(new_sitting_window) = sitting_window.update() else {
                return;
            };
            let sitting_pos = new_sitting_window.sitting_pos();
            let Some(transform) = tracker.tracking_on_sitting(vrm_entity, sitting_pos) else {
                return;
            };
            commands
                .entity(vrm_entity)
                .insert((new_sitting_window, transform));
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

// fn move_above_window(mut commands: Commands, mascots: Query<(Entity, &SittingWindow), With<Vrm>>) {
//     for (mascot_entity, sitting_window) in mascots.iter() {
//         if let Some(above_window) = unsafe { find_above_global_window(sitting_window.window.frame) }
//         {
//             commands.entity(mascot_entity).insert(SittingWindow {
//                 window: above_window,
//                 mascot_viewport_offset: sitting_window.mascot_viewport_offset,
//             });
//         }
//     }
// }
