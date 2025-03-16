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

This application uses actions to define different mascot behaviors.  
Each action belongs to a group, and animations are stored in `assets/animations`.

📂 Example:

```
assets/animations/
├── idle/
│ ├── idle_1.vrma
│ ├── idle_2.vrma
├── drag/
│ ├── drag_start.vrma
│ ├── drag_loop.vrma
```

➡️ **Each directory is an action group**, and the `.vrma` files inside define individual actions.

![action_group](./docs/action_group.drawio.png)

### Action Transition Types

Actions can transition between each other using different transition modes:

| Transition Type | Description                                                       |
|-----------------|-------------------------------------------------------------------|
| **auto**        | Transitions to another action in the same group after a set time. |
| **manual**      | Transitions to a specified action after playing.                  |
| **none**        | No transition (stays in the current action).                      |

🎛️ **How to Configure Transitions**

- You can change action transitions **from the settings menu**.
- Open the menu by **right-clicking on the mascot**.

## Credits

- [VRM Sample Model](https://vroid.pixiv.help/hc/ja/articles/4402394424089-AvatarSample-A-Z)
- Character animation credits to **pixiv Inc.'s VRoid Project**
- Uses [bevy_game_template](https://github.com/NiklasEi/bevy_game_template) to ci and build packages.

## License

This project is released under the **MIT License**.

## Contact

📢 **For questions & contributions:**

- **Discord:** `@not_not_elm`
