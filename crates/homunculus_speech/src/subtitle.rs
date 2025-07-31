use bevy::prelude::*;
use bevy_vrm1::prelude::{Cameras, HeadBoneEntity};
use homunculus_core::prelude::PrimaryCamera;
use std::path::PathBuf;

#[derive(Event)]
pub struct RequestShowSubtitle {
    pub vrm: Entity,
    pub audio_source: Entity,
    pub text: String,
    pub font_path: Option<PathBuf>,
    pub font_size: Option<f32>,
    pub color: Option<[f32; 4]>,
}

#[derive(Component)]
pub struct AudioSourceEntity(pub Entity);

pub(super) struct SpeakSubtitlesPlugin;

impl Plugin for SpeakSubtitlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestShowSubtitle>()
            .add_systems(Update, (request_subtitles, check_finished_speak));
    }
}

fn request_subtitles(
    mut commands: Commands,
    mut er: EventReader<RequestShowSubtitle>,
    cameras: Cameras,
    asset_server: Res<AssetServer>,
    primary_camera: Query<Entity, With<PrimaryCamera>>,
    transforms: Query<&Transform>,
    heads: Query<&HeadBoneEntity>,
) {
    for event in er.read() {
        let Ok(vrm_tf) = transforms.get(event.vrm) else {
            continue;
        };
        let Some(camera_entity) = cameras
            .find_by_world(vrm_tf.translation)
            .or_else(|| {
                let head = heads.get(event.vrm).ok()?;
                let head_tf = transforms.get(head.0).ok()?;
                cameras.find_by_world(head_tf.translation)
            })
            .map(|(entity, _, _, _)| entity)
            .or_else(|| primary_camera.single().ok())
        else {
            continue;
        };

        let mut font = TextFont::default();
        if let Some(path) = &event.font_path {
            font.font = asset_server.load(path.as_path());
        } else {
            font.font = asset_server.load("fonts/NotoSansJP-SemiBold.ttf");
        }
        if let Some(size) = event.font_size {
            font.font_size = size;
        } else {
            font.font_size = 30.0;
        }
        let text_color = event
            .color
            .map(|rgba| TextColor(Color::srgba(rgba[0], rgba[1], rgba[2], rgba[3])))
            .unwrap_or_default();
        commands.spawn((
            Pickable::IGNORE,
            AudioSourceEntity(event.audio_source),
            UiTargetCamera(camera_entity),
            Text::new(event.text.clone()),
            font,
            text_color,
            TextShadow {
                color: Color::BLACK.with_alpha(text_color.0.alpha()),
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                bottom: Val::Px(10.),
                ..default()
            },
        ));
    }
}

fn check_finished_speak(
    mut commands: Commands,
    players: Query<&AudioSink>,
    subtitles: Query<(Entity, &AudioSourceEntity)>,
) {
    for (entity, audio_source_entity) in subtitles.iter() {
        let Ok(sink) = players.get(audio_source_entity.0) else {
            commands.entity(audio_source_entity.0).try_despawn();
            commands.entity(entity).try_despawn();
            continue;
        };
        if sink.empty() {
            commands.entity(audio_source_entity.0).try_despawn();
            commands.entity(entity).try_despawn();
        }
    }
}
