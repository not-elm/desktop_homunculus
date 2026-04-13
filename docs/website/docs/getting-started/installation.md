---
title: "Installation"
sidebar_position: 2
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Installation

Download and install Desktop Homunculus. The installer includes everything you need — the engine, Node.js runtime, `hmcs` CLI, and all official MODs are set up automatically.

## System Requirements

|                | macOS                        | Windows             |
| -------------- | ---------------------------- | ------------------- |
| **OS**         | macOS 11 (Big Sur) or later  | Windows 10 or later |
| **CPU**        | Apple Silicon or Intel       | x86_64              |
| **Disk Space** | 600 MB or more               | 600 MB or more      |
| **Network**    | Required for initial setup   | Required for initial setup |

## Install Desktop Homunculus

Download the latest release from the [GitHub Releases page](https://github.com/not-elm/desktop-homunculus/releases).

<Tabs>
<TabItem value="macos" label="macOS" default>

1. Download the `.pkg` file
2. Open the installer and follow the prompts
3. Launch the app from Applications

The installer will:
- Install the Desktop Homunculus app to `/Applications`
- Add the `hmcs` CLI to your PATH (`/usr/local/bin/hmcs`)
- Automatically discover and install all official MODs

:::warning[Gatekeeper Warning]
If macOS shows "Desktop Homunculus can't be opened because it is from an unidentified developer":

1. Open **System Settings** > **Privacy & Security**
2. Scroll down and click **Open Anyway**
3. Click **Open** in the confirmation dialog
:::

</TabItem>
<TabItem value="windows" label="Windows">

:::caution[NVIDIA GPU Users — Required Before First Launch]
If you have an NVIDIA GPU, you **must** configure the following setting before launching Desktop Homunculus, otherwise the window background will be black instead of transparent:

1. Open **NVIDIA Control Panel**
2. Go to **Manage 3D Settings**
3. Find **"Vulkan/OpenGL present method"**
4. Set it to **"Prefer native"**
5. Click **Apply**

This must be done **before first launch**.
:::

1. Download the `.msi` file
2. Run the installer and follow the prompts
3. Launch **Desktop Homunculus** from the Start Menu

The installer will:
- Install the app and bundled Node.js runtime
- Add the `hmcs` CLI to your system PATH
- Automatically discover and install all official MODs

</TabItem>
</Tabs>

## Verify Installation

1. Launch **Desktop Homunculus**
2. The app icon should appear in the **system tray** (notification area)

You can also verify the CLI is available:

```shell
hmcs --version
```

If everything works, you're all set!

:::tip[Next Steps]
Head over to the [Quick Start](./quick-start.md) guide to create your first persona and start interacting with your character.
:::

## For Developers

If you're developing Desktop Homunculus or building MODs from source, install the CLI via Cargo instead:

```shell
# From the engine/ directory
make install-cli
```

This installs `hmcs` to `~/.cargo/bin/hmcs`.

## Troubleshooting

### `hmcs: command not found`

- **macOS**: Restart your terminal. The installer creates a symlink at `/usr/local/bin/hmcs`.
- **Windows**: Restart your terminal or sign out and back in. The installer adds the bin directory to your system PATH.

### App blocked by Gatekeeper (macOS)

See the installation instructions above for allowing the app through Gatekeeper.

### Black/opaque window background on Windows

If the window background appears black instead of transparent on Windows, you likely have an NVIDIA GPU that requires a configuration change. See the NVIDIA GPU setup instructions above.

### No character appears after launch

If official MODs failed to install during setup (e.g., due to network issues), install them manually:

```shell
hmcs mod install @hmcs/assets @hmcs/persona @hmcs/menu @hmcs/settings @hmcs/app-exit
```

Then restart Desktop Homunculus.

### MOD installation fails

Check your network connection and try again. MOD installation requires access to the npm registry.
