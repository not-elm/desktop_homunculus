use crate::VowelQueue;
use crate::mfcc::{calc_vowels, create_templates};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use dasp::ring_buffer::Slice;
use hound::WavReader;
use std::sync::Arc;

#[derive(Asset, TypePath, Clone)]
pub struct LipSync {
    queue: VowelQueue,
    audio_source: AudioSource,
}

#[derive(Debug, Component)]
pub struct LipSyncHandle(pub Handle<LipSync>);

pub struct LipSyncLoaderPlugin;

impl Plugin for LipSyncLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LipSync>()
            .init_asset_loader::<LipSyncLoader>()
            .add_systems(Update, load_lip_sync);
    }
}

fn load_lip_sync(
    mut commands: Commands,
    mut audios: ResMut<Assets<AudioSource>>,
    lips: Res<Assets<LipSync>>,
    handles: Query<(Entity, &LipSyncHandle)>,
) {
    for (entity, handle) in handles.iter() {
        let Some(lip_sync) = lips.get(&handle.0) else {
            continue;
        };
        commands
            .entity(entity)
            .insert(lip_sync.queue.clone())
            .insert(AudioPlayer::new(audios.add(lip_sync.audio_source.clone())))
            .remove::<LipSyncHandle>();
    }
}

#[derive(Default)]
struct LipSyncLoader;

impl AssetLoader for LipSyncLoader {
    type Asset = LipSync;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut LoadContext<'_>,
    ) -> std::result::Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        let audio_source = AudioSource {
            bytes: Arc::from(buf.slice()),
        };

        let cursor = std::io::Cursor::new(buf);
        let mut reader = WavReader::new(cursor).unwrap();
        Ok(LipSync {
            queue: VowelQueue {
                queue: calc_vowels(30, &mut reader, &create_templates()),
                timer: Timer::from_seconds(1.0 / 30.0, TimerMode::Repeating),
                velocity: 0.0,
            },
            audio_source,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav"]
    }
}
