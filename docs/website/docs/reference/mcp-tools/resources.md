---
title: "Resources"
sidebar_position: 7
---

# Resources

Resources provide read-only snapshots of Desktop Homunculus state.

### `homunculus://info`

Application info including version and platform.

**MIME type:** `application/json`

---

### `homunculus://characters`

Detailed state of all currently loaded VRM characters. Same data structure as `get_character_snapshot` output.

**MIME type:** `application/json`

---

### `homunculus://mods`

List of installed MODs with their available MOD commands and declared assets.

**MIME type:** `application/json`

Use this to discover what commands are available for `execute_command`.

---

### `homunculus://assets`

List of all available assets (VRM models, VRMA animations, sounds, images, HTML) across all installed MODs, with their asset IDs.

**MIME type:** `application/json`

Use this to discover asset IDs for `spawn_character`, `play_animation`, `play_sound`, and `control_bgm`.

