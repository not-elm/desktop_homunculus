---
title: "shadowPanel"
sidebar_position: 16.5
---

# shadowPanel

The `shadowPanel` module controls the shadow overlay panel -- a fullscreen transparent layer used for atmospheric effects or focus dimming.

```typescript
import { shadowPanel } from "@hmcs/sdk";

// Dim the background
await shadowPanel.setAlpha(0.7);

// Read the current alpha
const current = await shadowPanel.alpha();

// Remove the overlay
await shadowPanel.setAlpha(0);
```

`alpha` ranges from `0` (fully transparent / invisible) to `1` (fully opaque).
