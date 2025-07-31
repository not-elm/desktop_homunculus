//! # Homunculus Drag
//!
//! This crate provides drag and drop functionality for VRM mascot models in the
//! Desktop Homunculus application, enabling users to interactively move their
//! mascots around the desktop.
//!
//! ## Overview
//!
//! `homunculus_drag` implements a comprehensive drag system that allows users to
//! click and drag VRM models across the desktop. The system includes automatic
//! animations during dragging, sitting detection when dropped on windows, and
//! proper state management throughout the interaction.
//!
//! ## Key Features
//!
//! - **Interactive Dragging**: Click and drag VRM models with the primary mouse button
//! - **Drag Animations**: Automatic VRMA animation playback during drag operations
//! - **Sitting Detection**: Automatically detect when mascots are dropped on windows
//! - **State Management**: Proper VRM state transitions during drag operations
//! - **Multi-Monitor Support**: Seamless dragging across multiple displays
//! - **Bone Offset Handling**: Accurate positioning accounting for VRM bone structure
//!
//! ## Drag Interaction Flow
//!
//! 1. **Drag Start**: User clicks on a VRM model with the primary mouse button
//!    - Drag animation begins playing
//!    - VRM state changes to "drag"
//!    - Hip bone offset is calculated and stored
//!
//! 2. **Drag Move**: User moves mouse while holding the button
//!    - VRM model position updates to follow cursor
//!    - Position is adjusted for bone offsets to maintain proper alignment
//!
//! 3. **Drag End**: User releases the mouse button
//!    - System checks if mascot was dropped on a window
//!    - If on window: enters sitting mode on that window
//!    - If not on window: returns to default idle state
//!
//! ## Sitting Integration
//!
//! When a mascot is dropped on a window, the system:
//! - Detects the target window using global cursor position
//! - Calculates appropriate sitting position on the window
//! - Transitions the mascot to sitting state
//! - Applies proper transform for window positioning
//!
//! ## Animation System
//!
//! The drag system automatically loads and plays a "drag.vrma" animation file
//! when dragging begins. This animation loops continuously until the drag
//! operation ends, providing visual feedback to the user.

use bevy::animation::RepeatAnimation;
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy_vrm1::prelude::{Initialized, ParentSearcher, PlayVrma};
use bevy_vrm1::vrm::Vrm;
use bevy_vrm1::vrma::VrmaHandle;
use homunculus_core::prelude::{
    AppWindows, BoneOffsets, Coordinate, MascotTracker, VrmState, global_cursor_pos,
};
use homunculus_screen::prelude::GlobalWindows;
use homunculus_sitting::SittingWindow;
use std::time::Duration;

/// Plugin that provides drag and drop functionality for VRM mascot models.
///
/// This plugin enables interactive dragging of VRM models across the desktop,
/// including automatic animations, sitting detection, and proper state management
/// throughout the drag interaction lifecycle.
///
/// # Functionality
///
/// The plugin automatically sets up observers on newly initialized VRM models
/// to handle drag interactions. It provides:
///
/// - **Drag Start Observer**: Handles mouse button press events
/// - **Drag Move Observer**: Updates model position during dragging
/// - **Drag End Observer**: Manages drop behavior and state transitions
/// - **Animation Integration**: Loads and plays drag animations
///
/// # Requirements
///
/// This plugin requires the following to be available:
/// - VRM models with the `Vrm` and `Initialized` components
/// - Asset server for loading drag animation files
/// - Core homunculus systems for coordinate transformation and bone offset calculation
/// - Sitting system for window detection and sitting behavior
///
/// # Automatic Setup
///
/// The plugin automatically observes newly spawned VRM models and sets up
/// the necessary drag interaction handlers. No additional setup is required
/// beyond adding the plugin to your app.
pub struct HomunculusDragPlugin;

impl Plugin for HomunculusDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, observe_vrm);
    }
}

#[derive(Component)]
struct DragHipsOffset(Vec3);

#[derive(Component)]
struct DragVrma;

fn observe_vrm(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    vrms: Query<Entity, (With<Vrm>, Added<Initialized>)>,
) {
    for vrm in vrms.iter() {
        let drag_vrma = commands
            .spawn((DragVrma, VrmaHandle(asset_server.load("vrma/drag.vrma"))))
            .id();
        commands
            .entity(vrm)
            .observe(on_drag_start)
            .observe(on_drag_move)
            .observe(on_drag_end)
            .add_child(drag_vrma)
            .observe(
                move |trigger: Trigger<Pointer<DragStart>>, mut commands: Commands| {
                    if !matches!(trigger.event.button, PointerButton::Primary) {
                        return;
                    }
                    commands.entity(drag_vrma).trigger(PlayVrma {
                        repeat: RepeatAnimation::Forever,
                        transition_duration: Duration::from_millis(600),
                    });
                },
            );
    }
}

fn on_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    parent_searcher: ParentSearcher,
    bone_offsets: BoneOffsets,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let Some(vrm_entity) = parent_searcher.find_vrm(trigger.target()) else {
        return;
    };
    let Some(hips) = bone_offsets.hips_offset(vrm_entity) else {
        return;
    };
    commands
        .entity(vrm_entity)
        .insert(DragHipsOffset(hips.xy().extend(0.0)))
        .insert(VrmState::from("drag"));
}

fn on_drag_move(
    trigger: Trigger<Pointer<Drag>>,
    mut commands: Commands,
    parent_searcher: ParentSearcher,
    coordinate: Coordinate,
    transforms: Query<(&Transform, &DragHipsOffset)>,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let location = &trigger.pointer_location;
    let Some(vrm_entity) = parent_searcher.find_vrm(trigger.target()) else {
        return;
    };
    let Ok((transform, hips_offset)) = transforms.get(vrm_entity) else {
        return;
    };
    let vrm_pos = transform.translation;
    let NormalizedRenderTarget::Window(window_ref) = location.target else {
        return;
    };
    let Some(current) =
        coordinate.to_world_pos_from_window(window_ref.entity(), location.position, vrm_pos)
    else {
        return;
    };
    commands.entity(vrm_entity).insert(Transform {
        translation: current - hips_offset.0,
        ..*transform
    });
}

fn on_drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    windows: AppWindows,
    tracker: MascotTracker,
    parent_searcher: ParentSearcher,
) {
    if !matches!(trigger.event.button, PointerButton::Primary) {
        return;
    }
    let Some(vrm) = parent_searcher.find_vrm(trigger.target) else {
        return;
    };

    let Some(global_cursor_pos) = global_cursor_pos(&trigger, &windows) else {
        return;
    };
    match GlobalWindows::find_all().and_then(|gw| gw.find_sitting_window(global_cursor_pos)) {
        Some(global_window) => {
            let sitting_pos = global_window.sitting_pos(global_cursor_pos);
            let sitting_window = SittingWindow::new(global_window, sitting_pos);
            let Some(transform) = tracker.tracking_on_sitting(vrm, sitting_window.sitting_pos())
            else {
                return;
            };
            info!("Sitting on {:?}", sitting_window.window.title);
            commands.entity(vrm).insert((
                sitting_window,
                transform,
                VrmState::from(VrmState::SITTING),
            ));
        }
        None => {
            commands.entity(vrm).insert(VrmState::default());
        }
    }
}
