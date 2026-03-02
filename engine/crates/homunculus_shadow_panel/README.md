# homunculus_shadow_panel

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

The shadow panel represents the shadow displayed behind the character.
It internally places the transparent plane behind avatars and applies a special shader to project only the shadow of
avatars.

## Features

- **Custom Shadow Material**: Specialized material for rendering shadows
- **WGSL Shader**: Custom shader implementation for shadow rendering
- **Directional Light**: Automatic setup of directional light for shadow casting
- **Shadow Panel Mesh**: Plane mesh that receives shadows from VRM models
- **Alpha Control**: Adjustable transparency for the shadow panel
- **Camera Integration**: Works with all camera layers in the application
- **Bevy Material System**: Leverages Bevy's material system for efficient rendering
- **Transparent Background**: Blends seamlessly with the desktop environment