# SDK Reference Heading Normalization & Page Split

## Problem

SDK reference documentation uses inconsistent heading styles. Some pages use descriptive text as headings (e.g., "Rotation Tween" / "回転トゥイーン") while others use actual function names (e.g., `tweenRotation`). Japanese pages translate function headings into Japanese, making it impossible to look up functions by their API name.

## Goals

- Every SDK function is discoverable by its actual API name
- Consistent 1-function-per-page structure across all modules
- Japanese pages use original function names in headings (no translation)
- Type definitions are separated into per-module `types.md` pages with cross-links from function pages

## Rules

1. **All modules** become `module-name/` directories (replacing single `.md` files)
2. **1 function = 1 page** (filename = function name, e.g., `tweenRotation.md`)
3. **Headings** = function name as-is, no backticks (e.g., `# tweenRotation`)
4. **Japanese pages**: headings use the English function name; only body text is in Japanese
5. **Type definitions** → per-module `types.md`, with links from function pages
6. **`index.md`** per module = import instructions + function list table (name + 1-line description + link)
7. **`tweening.md` abolished** — tween functions consolidated into `entities/`

## Module Mapping

| Module | Current File | New Directory | Function Pages | Types |
|--------|-------------|---------------|----------------|-------|
| app | `app.md` | `app/` | health, info, exit | AppInfo, PlatformInfo, InfoMod |
| audio | `audio.md` | `audio/` | se-play, bgm-play, bgm-stop, bgm-pause, bgm-resume, bgm-update, bgm-status | SeOptions, BgmPlayOptions, BgmStopOptions, BgmUpdateOptions, FadeTween, BgmStatus |
| assets | `assets-api.md` | `assets/` | list | AssetType, AssetInfo, AssetFilter |
| commands | `commands.md` | `commands/` | input-parse, input-parseMenu, input-read, output-succeed, output-fail, output-write, output-writeError | StdinParseError |
| coordinates | `coordinates.md` | `coordinates/` | toWorld, toViewport | GlobalDisplay |
| displays | `displays.md` | `displays/` | findAll | GlobalDisplay |
| effects | `effects.md` | `effects/` | stamp | StampOptions |
| entities | `entities.md` + `tweening.md` | `entities/` | findByName, name, transform, setTransform, move, tweenPosition, tweenRotation, tweenScale | FindOptions, MoveTarget, TweenPositionRequest, TweenRotationRequest, TweenScaleRequest, EasingFunction |
| host | `direct-http.md` | `host/` | get, post, put, deleteMethod, createUrl, streamNdjson | HomunculusApiError, HomunculusStreamError |
| mods | `mods-api.md` | `mods/` | list, get, executeCommand, streamCommand, menus | ModInfo, ExecuteCommandRequest, CommandEvent, CommandResult, ModMenuMetadata |
| preferences | `preferences.md` | `preferences/` | list, load, save (+ delete if it exists in SDK) | (none — uses generic JSON) |
| settings | `settings.md` | `settings/` | fps, setFps | (none) |
| signals | `signals.md` | `signals/` | list, stream, send | SignalChannelInfo |
| speech | `speech.md` | `speech/` | fromPhonemes | TimelineKeyframe |
| webviews | `webviews.md` | `webviews/` | open, list, current, close, isClosed, info, patch, setOffset, setSize, setViewportSize, navigate, reload, navigateBack, navigateForward, linkedVrm, setLinkedVrm, unlinkVrm + webviewSource helpers (local, url, html) | WebviewSource, WebviewInfo, WebviewPatchRequest, WebviewOpenOptions |

## Directory Structure Example (entities)

```
entities/
├── _category_.json       # { "label": "entities", "position": N }
├── index.md              # Import + function list table
├── findByName.md
├── name.md
├── transform.md
├── setTransform.md
├── move.md
├── tweenPosition.md      # consolidated from tweening.md
├── tweenRotation.md
├── tweenScale.md
└── types.md
```

Both `docs/website/docs/mod-development/sdk/` (EN) and `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/sdk/` (JA) get the same directory structure.

## Page Templates

### index.md

```markdown
---
sidebar_position: 1
---

# entities

## Import

\`\`\`typescript
import { entities } from "@hmcs/sdk";
\`\`\`

## Functions

| Function | Description |
|----------|-------------|
| [findByName](./findByName) | Find an entity by name |
| [name](./name) | Get entity name |
| [transform](./transform) | Get entity transform |
| [setTransform](./setTransform) | Update entity transform |
| [move](./move) | Move entity to target coordinates |
| [tweenPosition](./tweenPosition) | Animate position |
| [tweenRotation](./tweenRotation) | Animate rotation |
| [tweenScale](./tweenScale) | Animate scale |

See also: [Type Definitions](./types)
```

### Function page (e.g., tweenRotation.md)

```markdown
---
sidebar_position: 8
---

# tweenRotation

Animates the rotation of an entity to a target quaternion over a specified duration.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| entityId | `string` | Target entity ID |
| request | [`TweenRotationRequest`](./types#tweenrotationrequest) | Tween configuration |

## Returns

`Promise<void>`

## Example

\`\`\`typescript
import { entities } from "@hmcs/sdk";

await entities.tweenRotation("character", {
  target: { x: 0, y: 0.707, z: 0, w: 0.707 },
  duration: 1.0,
  easing: "easeInOutSine",
});
\`\`\`
```

### types.md

```markdown
---
sidebar_position: 100
---

# Type Definitions

## FindOptions

\`\`\`typescript
interface FindOptions {
  // ...
}
\`\`\`

## TweenRotationRequest

\`\`\`typescript
interface TweenRotationRequest {
  target: Quaternion;
  duration: number;
  easing?: EasingFunction;
}
\`\`\`

...
```

## Files to Delete

- `docs/website/docs/mod-development/sdk/tweening.md` (EN)
- `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/sdk/tweening.md` (JA)

All other single-file modules (e.g., `app.md`, `audio.md`) are replaced by their directory equivalents and should be deleted after migration.

## Open Items

- **vrm/**: Already split into sub-pages. Needs review against the new heading convention (function names without backticks, no Japanese translation of headings).
- **math.md**: Contains only type definitions (Transform, Vec2, Vec3, Quaternion, Rect), no functions. May remain as a single page or become `math/types.md`.
- **shadow-panel.md**: Minimal content. Needs verification of what it documents.
- **quick-start.md**: Tutorial page, not a module reference. Excluded from this change.
- **index.md** (SDK overview): Needs link updates to point to new directory paths.

## Scope

- ~15 modules to split (EN + JA = ~30 directory conversions)
- ~80 function pages to create per language (~160 total)
- ~15 types.md pages per language (~30 total)
- ~15 index.md pages per language (~30 total)
- Total: ~220 new files, ~32 files deleted (16 old .md files x 2 languages)
