---
sidebar_position: 100
---

# Type Definitions

## StampOptions

Configuration options for stamp visual effects.

| Field | Type | Description |
|-------|------|-------------|
| `x` | `number` | X position on screen (pixels) |
| `y` | `number` | Y position on screen (pixels) |
| `width` | `number` | Width in pixels |
| `height` | `number` | Height in pixels |
| `alpha` | `number` | Opacity (0--1) |
| `duration` | `number` | Duration in seconds before the stamp disappears |

## StampRequestBody

Request body for creating a stamp effect.

| Field | Type | Description |
|-------|------|-------------|
| `asset` | `string` | Asset ID of the stamp image |
| `x` | `number` | X position on screen (pixels) |
| `y` | `number` | Y position on screen (pixels) |
| `width` | `number` | Width in pixels |
| `height` | `number` | Height in pixels |
| `alpha` | `number` | Opacity (0--1) |
| `duration` | `number` | Duration in seconds |
