---
title: "Core Concepts"
sidebar_position: 3
---

# Core Concepts

Before diving into the Quick Start, here are three key concepts you'll encounter throughout Desktop Homunculus.

## Persona

A **persona** is a character that lives on your desktop. Each persona has:

- **Identity** — A name, age, gender, personality description, and profile text that define who the character is
- **VRM Model** — A 3D avatar rendered in a transparent window that floats above your other applications. You can use the built-in model or bring your own [VRM](https://vrm.dev/en/) file
- **Persistent Settings** — Appearance tweaks (bone scale adjustments) and preferences that are saved between sessions

You can create multiple personas, each with its own look and personality. Enable **Auto-Spawn** on a persona to have it appear automatically every time you launch the app.

Personas are managed through the **Persona Management** dashboard, accessible from the system tray.

## MOD

A **MOD** is a package that adds features to Desktop Homunculus. The app's core functionality — including persona management, animations, and the right-click menu — is provided by official MODs that ship with the installer.

Official MODs use the `@hmcs/` prefix:

| MOD | What it does |
|---|---|
| `@hmcs/persona` | Persona management and default character behavior |
| `@hmcs/assets` | Built-in VRM model, animations, and sound effects |
| `@hmcs/menu` | Right-click context menu |
| `@hmcs/settings` | Application settings panel |
| `@hmcs/app-exit` | Exit option in the system tray |

Additional MODs — such as text-to-speech (`@hmcs/voicevox`) and speech recognition (`@hmcs/stt`) — can be installed at any time using the `hmcs` CLI. For AI-powered interaction, see [AI Integration](../ai-integration) (requires running [OpenClaw](https://docs.openclaw.ai) externally).

## Asset

An **asset** is a file bundled with a MOD — VRM models, animations (VRMA), sound effects, images, or HTML UIs. Assets are referenced by ID in the format **`mod-name:asset-id`**.

For example:
- `vrm:elmer` — the built-in VRM character model
- `vrma:idle-maid` — the default idle animation
- `se:open` — a UI open sound effect

The `@hmcs/assets` MOD provides the default set of assets that power the persona system out of the box.

## Next Steps

Ready to get started? Head over to the [Quick Start](./quick-start) guide to create your first persona.
