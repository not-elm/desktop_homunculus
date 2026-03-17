---
sidebar_position: 100
---

# Type Definitions

## TimelineKeyframe

Defined in the VRM module. Each keyframe specifies a duration and optional expression targets.

| Field | Type | Description |
|-------|------|-------------|
| `duration` | `number` | Duration in seconds |
| `targets` | `Record<string, number>` | Expression name to weight (0--1) mapping. Omit for silence. |
