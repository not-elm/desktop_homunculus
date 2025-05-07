# desktop_homunculus

[itch.io](https://notelm.itch.io/desktop-homunculus)

> [!CAUTION]
> This crate is in an early stage of development and may undergo breaking changes.

## Overview

Desktop Homunculus is an application that summons [VRM](https://vrm.dev/en/vrm/vrm_about/) models on your desktop.

## Features

- By using [VRMA](https://vrm.dev/en/vrma/) files, you can freely animate mascots.
- Multiple VRM models can be placed simultaneously.
- Supports multi-monitor

## Supported Platforms

Currently, **Windows is supported**, and macOS support is planned.

| Platform   | Status              |
|------------|---------------------|
| ✅ MacOS    | Supported           |
| ⚠️ Windows | Partially Supported |

> [!WARNING]
> On Windows, due to bug in `winit` or `wgpu`,
> it is currently not possible to create a transparent window with backends both `vulkan` and `dx12`.
>
>This application uses `open-gl` to avoid this bug, but on some devices, the application may crash during rendering.

## Animation Actions & Transitions

TODO

## Credits

- [VRM Sample Model](https://vroid.pixiv.help/hc/ja/articles/4402394424089-AvatarSample-A-Z)
- Character animation credits to **pixiv Inc.'s VRoid Project**
- Uses [bevy_game_template](https://github.com/NiklasEi/bevy_game_template) to ci and build packages.

## License

This project is released under the **MIT License**.

## Contact

- **Discord:** `@not_not_elm`
