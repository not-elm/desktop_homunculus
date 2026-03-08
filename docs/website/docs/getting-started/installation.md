---
title: "Installation"
sidebar_position: 2
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Installation

Follow these steps to install Desktop Homunculus and set up your MOD environment.

## System Requirements

| | macOS | Windows |
|---|---|---|
| **OS** | macOS 12 (Monterey) or later | Windows 10 or later |
| **CPU** | Apple Silicon or Intel | x86_64 |
| **Node.js** | 22 or later | 22 or later |
| **Disk Space** | 500 MB or more | 500 MB or more |

## Step 1: Install Desktop Homunculus

Download the latest release from the [GitHub Releases page](https://github.com/not-elm/desktop_homunculus/releases).

<Tabs>
<TabItem value="macos" label="macOS" default>

1. Download the `.dmg` file
2. Open the DMG and drag **Desktop Homunculus** into your `/Applications` folder
3. Launch the app from Applications

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

This must be done **before first launch**. See [bevyengine/bevy#7544](https://github.com/bevyengine/bevy/issues/7544) for details.
:::

1. Download the `.msi` file
2. Run the installer and follow the prompts
3. Launch **Desktop Homunculus** from the Start Menu

</TabItem>
</Tabs>

## Step 2: Install Node.js

Desktop Homunculus MODs require **Node.js 22 or later** to run TypeScript scripts directly using tsx.

1. Visit the [Node.js download page](https://nodejs.org/en/download)
2. Download and install the **LTS** version (22 or later)
3. Verify the installation:

```shell
node -v
```

The output should show `v22.0.0` or higher.

:::tip
If you already have Node.js installed, check the version with `node -v`. If it's below v22, update to the latest LTS version.
:::

## Step 3: Install MOD Management Tools

Install **pnpm** (package manager) and **@hmcs/cli** (MOD management CLI) globally:

```shell
npm install -g pnpm @hmcs/cli
```

Verify the installation:

```shell
pnpm -v
hmcs --version
```

:::warning[Permission Error]
If you see `EACCES` permission errors, see the [npm docs on resolving permission errors](https://docs.npmjs.com/resolving-eacces-permissions-errors-when-installing-packages-globally).
:::

## Step 4: Install Official MODs

Install the recommended set of official MODs:

```shell
hmcs mod install @hmcs/assets @hmcs/elmer @hmcs/menu @hmcs/character-settings @hmcs/settings
```

| MOD | Description |
|---|---|
| `@hmcs/assets` | Default animations and sound effects |
| `@hmcs/elmer` | Default character model |
| `@hmcs/menu` | Right-click context menu |
| `@hmcs/character-settings` | Per-character settings panel |
| `@hmcs/settings` | Application settings (frame rate, shadow opacity) via system tray |

## Step 5: Verify Installation

1. Launch **Desktop Homunculus**
2. A character should appear on your desktop
3. Right-click the character to open the context menu

If everything works, you're all set!

:::tip[Next Steps]
Head over to the [Quick Start](./quick-start.md) guide to learn how to interact with your character.
:::

## Troubleshooting

### `hmcs: command not found`

Your terminal doesn't recognize the `hmcs` command.

- **Restart your terminal** — the `PATH` may not have been updated
- Verify the global npm bin directory is in your PATH:
  ```shell
  npm bin -g
  ```

### Node.js version is below 22

MODs require Node.js 22+ to run TypeScript scripts directly via tsx. Download the latest LTS from [nodejs.org](https://nodejs.org/download).

### App blocked by Gatekeeper

See [Step 1](#step-1-install-desktop-homunculus) for instructions on allowing the app.

### Black/opaque window background on Windows

If the window background appears black instead of transparent on Windows, you likely have an NVIDIA GPU that requires a configuration change. See the [NVIDIA GPU setup instructions](#step-1-install-desktop-homunculus) in Step 1 (Windows tab).

### No character appears after installing MODs

Restart Desktop Homunculus. MOD changes take effect after a restart.

### MOD installation fails

Check your network connection and try again.
