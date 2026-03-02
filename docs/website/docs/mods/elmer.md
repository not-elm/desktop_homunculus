---
title: "Elmer"
sidebar_position: 3
---

# Elmer

The Elmer MOD (`@hmcs/elmer`) is the default character MOD. It spawns the Elmer VRM character on your desktop and manages its animations and behavior.

## What It Does

When Desktop Homunculus starts, the Elmer MOD automatically:

1. **Spawns the Elmer character** using the `vrm:elmer` model from the [Assets](./assets) MOD
2. **Plays the idle animation** (`vrma:idle-maid`) on loop
3. **Follows your cursor** — Elmer's eyes track your mouse position
4. **Responds to interactions:**
   - **Dragging** — Switches to the grabbed pose (`vrma:grabbed`) and stops cursor tracking
   - **Sitting on a window edge** — Switches to the sitting animation (`vrma:idle-sitting`)
   - **Releasing** — Returns to idle and resumes cursor tracking

## Notes

- The Elmer MOD requires the [Assets](./assets) MOD for its VRM model and animations.
- Character position is saved automatically via preferences and restored on next launch.
