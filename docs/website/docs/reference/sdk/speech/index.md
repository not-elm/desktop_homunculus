---
sidebar_position: 1
---

# speech

Utilities for converting phoneme data into timeline keyframes for VRM lip-sync. The `speech` module produces keyframe arrays that you pass to `vrm.speakWithTimeline()`.

## Import

```typescript
import { speech } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [fromPhonemes](./fromPhonemes) | Converts a list of `[expression_name, duration]` tuples into `TimelineKeyframe[]` |

See also: [Type Definitions](./types)
