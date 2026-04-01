//! Texture sampling utilities for hit testing and transparency detection.

use bevy::image::Image;
use bevy::math::Vec2;
use bevy::render::render_resource::TextureFormat;

/// Alpha threshold for click-through transparency.
///
/// Pixels with alpha at or below this value are treated as transparent
/// and allow interaction to pass through to entities behind them.
pub const TRANSPARENT_ALPHA_THRESHOLD: f32 = 0.0;

/// Samples the alpha value from a texture at the given UV coordinates.
///
/// Handles UV wrapping and various texture formats. Returns 1.0 (fully opaque)
/// if the texture format doesn't have an alpha channel or is unsupported.
pub fn sample_texture_alpha(image: &Image, uv: Vec2) -> f32 {
    let width = image.width();
    let height = image.height();

    if width == 0 || height == 0 {
        return 1.0;
    }

    let Some(data) = &image.data else {
        return 1.0;
    };

    let u = uv.x.rem_euclid(1.0);
    let v = uv.y.rem_euclid(1.0);

    let x = ((u * width as f32) as usize).min(width as usize - 1);
    let y = ((v * height as f32) as usize).min(height as usize - 1);

    match image.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            let idx = (y * width as usize + x) * 4 + 3;
            data.get(idx).map(|&a| a as f32 / 255.0).unwrap_or(1.0)
        }
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            let idx = (y * width as usize + x) * 4 + 3;
            data.get(idx).map(|&a| a as f32 / 255.0).unwrap_or(1.0)
        }
        _ => 1.0,
    }
}
