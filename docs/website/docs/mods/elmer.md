---
title: "@hmcs/elmer"
sidebar_position: 3
---

# @hmcs/elmer

The Elmer MOD (`@hmcs/elmer`) is the default character MOD. It spawns the Elmer VRM character on your desktop and manages its animations and behavior.

## Overview

When Desktop Homunculus starts, the Elmer MOD automatically spawns the Elmer character using the `vrm:elmer` model from the [@hmcs/assets](./assets) MOD and plays the idle animation on loop.

## Features

1. **Plays idle animation** (`vrma:idle-maid`) on loop
2. **Follows cursor** — Elmer's eyes track your mouse position
3. **Responds to interactions:**
   - **Dragging** — Switches to the grabbed pose (`vrma:grabbed`) and stops cursor tracking
   - **Sitting on a window edge** — Switches to the sitting animation (`vrma:idle-sitting`)
   - **Releasing** — Returns to idle and resumes cursor tracking

## Notes

- The Elmer MOD requires the [@hmcs/assets](./assets) MOD for its VRM model and animations.
- Character position is saved automatically via preferences and restored on next launch.
