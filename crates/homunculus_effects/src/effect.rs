// use bevy::asset::RenderAssetUsages;
// use bevy::prelude::*;
// use bevy::render::camera::RenderTarget;
// use bevy::render::render_resource::{
//     AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat,
// };
// use bevy::window::{PrimaryWindow, WindowRef};
// use bevy::winit::WinitWindows;
// use homunculus_core::prelude::CameraOrders;
// use scap::Target;
// use scap::frame::{BGRAFrame, Frame, FrameType};
// use std::sync::Mutex;
// use std::time::Duration;

// pub(crate) struct EffectPlugin;

// impl Plugin for EffectPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(UiMaterialPlugin::<EffectMaterial>::default())
//             .add_systems(Startup, (spawn_effect_camera, spawn_effect_images).chain());
//     }
// }

// #[derive(Deref, DerefMut, Component)]
// struct CaptureStream(Mutex<scap::capturer::Capturer>);

// #[derive(Component)]
// struct EffectCamera;

// #[derive(Component)]
// struct EffectImage;

// fn spawn_effect_camera(mut commands: Commands, windows: Query<Entity, With<PrimaryWindow>>) {
//     for window in windows.iter() {
//         commands.spawn((
//             EffectCamera,
//             Camera2d,
//             Camera {
//                 order: CameraOrders::EFFECT,
//                 target: RenderTarget::Window(WindowRef::Entity(window)),
//                 ..default()
//             },
//         ));
//     }
// }

// fn spawn_effect_images(mut commands: Commands, cameras: Query<Entity, With<EffectCamera>>) {
//     for camera in cameras.iter() {
//         commands.spawn((
//             EffectImage,
//             Pickable::IGNORE,
//             Node {
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//                 ..default()
//             },
//             UiTargetCamera(camera),
//         ));
//     }
// }

// fn start_capture(
//     mut commands: Commands,
//     mut windows: Query<&mut Window>,
//     mut materials: ResMut<Assets<EffectMaterial>>,
//     asset_server: Res<AssetServer>,
//     effects: Query<Entity, With<EffectImage>>,
//     _: NonSend<WinitWindows>,
// ) {
//     for effect in effects.iter() {
//         let targets = scap::get_all_targets();
//         let target = targets
//             .iter()
//             .find(|t| matches!(t, Target::Display(_)))
//             .cloned();
//         let window_ids: Vec<u64> = unsafe { homunculus_screen::prelude::app_window_numbers() };
//         let excludes = window_ids
//             .iter()
//             .flat_map(|id| {
//                 targets.iter().find(|t| match t {
//                     Target::Window(target) => target.id as u64 == *id,
//                     _ => false,
//                 })
//             })
//             .cloned()
//             .collect::<Vec<_>>();
//         let mut capture = scap::capturer::Capturer::new(scap::capturer::Options {
//             fps: 30,
//             target,
//             output_type: FrameType::BGRAFrame,
//             excluded_targets: Some(excludes),
//             ..default()
//         });
//         capture.start_capture();
//         let capture = CaptureStream(Mutex::new(capture));
//         commands.entity(effect).insert((
//             capture,
//             MaterialNode(materials.add(EffectMaterial {
//                 mask: asset_server.load("plugins/streaming/burning.png"),
//                 ..default()
//             })),
//             EffectStopWatch(Timer::new(Duration::from_secs(5), TimerMode::Once)),
//         ));
//     }

//     for mut window in windows.iter_mut() {
//         window.transparent = false;
//     }
// }

// #[derive(Component, Deref, DerefMut)]
// struct EffectStopWatch(Timer);

// fn b(
//     mut images: ResMut<Assets<Image>>,
//     mut materials: ResMut<Assets<EffectMaterial>>,
//     mut effects: Query<(
//         &CaptureStream,
//         &MaterialNode<EffectMaterial>,
//         &mut EffectStopWatch,
//     )>,
//     time: Res<Time>,
// ) {
//     for (stream, effect, mut stop_watch) in effects.iter_mut() {
//         let Ok(stream) = stream.0.try_lock() else {
//             continue;
//         };
//         let Ok(Frame::BGRA(frame)) = stream.get_next_frame() else {
//             return;
//         };
//         if frame.width == 0 || frame.height == 0 {
//             return;
//         }
//         let Some(effect_material) = materials.get_mut(effect.0.id()) else {
//             continue;
//         };
//         let time = stop_watch.tick(time.delta());
//         images.remove(effect_material.screen.id());
//         effect_material.screen = images.add(create_screenshot_image(frame));
//         effect_material.threshold = time.fraction();
//         // if let Some(image) = images.get_mut(&effect_material.screen) {
//         //     *image = i;
//         // }
//     }
// }

// fn stop_capture(
//     mut commands: Commands,
//     mut windows: Query<&mut Window>,
//     effects: Query<(Entity, &EffectStopWatch)>,
// ) {
//     for (entity, timer) in effects.iter() {
//         if timer.finished() {
//             commands
//                 .entity(entity)
//                 .remove::<EffectStopWatch>()
//                 .remove::<MaterialNode<EffectMaterial>>();
//             for mut window in windows.iter_mut() {
//                 window.transparent = true;
//             }
//         }
//     }
// }

// fn create_screenshot_image(frame: BGRAFrame) -> Image {
//     Image::new(
//         Extent3d {
//             width: frame.width as u32,
//             height: frame.height as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         frame.data,
//         TextureFormat::Bgra8UnormSrgb,
//         RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
//     )
// }

// #[derive(Clone, Reflect, AsBindGroup, Asset, Default)]
// struct EffectMaterial {
//     #[texture(0)]
//     #[sampler(1)]
//     screen: Handle<Image>,
//     #[texture(2)]
//     #[sampler(3)]
//     mask: Handle<Image>,
//     #[uniform(4)]
//     threshold: f32,
// }

// impl UiMaterial for EffectMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "plugins/streaming/burn.wgsl".into()
//     }
// }
