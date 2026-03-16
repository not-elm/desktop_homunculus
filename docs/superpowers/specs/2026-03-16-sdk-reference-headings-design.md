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
8. **Nested namespaces** (e.g., `audio.se`, `audio.bgm`): filename uses hyphen (`se-play.md`), heading uses dot notation (`# se.play`)
9. **Class static methods** (e.g., `Webview.open`): filename = `open.md`, heading = `# Webview.open`
10. **Class instance methods** (e.g., `webview.close()`): filename = `close.md`, heading = `# close`
11. **Cross-module type references**: link to the owning module's types.md (e.g., `[GlobalDisplay](../coordinates/types#globaldisplay)`)

## Module Mapping

| Module | Current File | New Directory | Function Pages | Types |
|--------|-------------|---------------|----------------|-------|
| app | `app.md` | `app/` | health, info, exit | AppInfo, PlatformInfo, InfoMod |
| audio | `audio.md` | `audio/` | se-play, bgm-play, bgm-stop, bgm-pause, bgm-resume, bgm-update, bgm-status | SeOptions, BgmPlayOptions, BgmStopOptions, BgmUpdateOptions, FadeTween, BgmStatus |
| assets | `assets-api.md` | `assets/` | list | AssetType, AssetInfo, AssetFilter |
| commands | `commands.md` | `commands/` | input-parse, input-parseMenu, input-read, output-succeed, output-fail, output-write, output-writeError | StdinParseError |
| coordinates | `coordinates.md` | `coordinates/` | toWorld, toViewport | GlobalDisplay, GlobalViewport |
| displays | `displays.md` | `displays/` | findAll | (none — uses [GlobalDisplay](../coordinates/types)) |
| effects | `effects.md` | `effects/` | stamp | StampOptions, StampRequestBody |
| entities | `entities.md` + `tweening.md` | `entities/` | findByName, name, transform, setTransform, move, tweenPosition, tweenRotation, tweenScale | FindOptions, MoveTarget, MoveTargetWorld, MoveTargetViewport, TweenPositionRequest, TweenRotationRequest, TweenScaleRequest, EasingFunction |
| host | `direct-http.md` | `host/` | configure, base, baseUrl, createUrl, get, post, put, patch, deleteMethod, postStream | HomunculusApiError, HomunculusStreamError |
| math | `math.md` | `math/` | (none — types only) | Transform, TransformArgs, Vec2, Vec3, Quaternion, Rect |
| mods | `mods-api.md` | `mods/` | list, get, executeCommand, streamCommand, menus | ModInfo, ExecuteCommandRequest, CommandEvent, CommandResult, ModMenuMetadata |
| preferences | `preferences.md` | `preferences/` | list, load, save | (none — uses generic JSON) |
| settings | `settings.md` | `settings/` | fps, setFps | SetFpsBody |
| shadowPanel | `shadow-panel.md` | `shadow-panel/` | alpha, setAlpha | ShadowPanelPutBody |
| signals | `signals.md` | `signals/` | list, stream, send | SignalChannelInfo |
| speech | `speech.md` | `speech/` | fromPhonemes | TimelineKeyframe |
| utils | (new) | `utils/` | sleep | (none) |
| vrm | `vrm/` (existing) | `vrm/` (restructure) | See [VRM section](#vrm-module-restructure) | See [VRM section](#vrm-module-restructure) |
| webviews | `webviews.md` | `webviews/` | See [Webviews section](#webviews-module-detail) | WebviewSource, WebviewSourceLocal, WebviewSourceUrl, WebviewSourceHtml, WebviewInfo, WebviewPatchRequest, WebviewOpenOptions |

### Special import: commands

The `commands` module uses a separate entry point. Import is `@hmcs/sdk/commands` (not `@hmcs/sdk`). The `commands/index.md` must reflect this.

### math module

`math` has no functions — only type definitions. Its directory contains only `index.md` (overview + import) and `types.md`.

### Webviews module detail

The `webviews` module exports a `Webview` class (static + instance methods) and a `webviewSource` helper namespace.

**Static methods** (heading format: `# Webview.open`):
- Webview.open, Webview.list, Webview.current

**Instance methods** (heading format: `# close`):
- close, isClosed, info, patch, setOffset, setSize, setViewportSize, navigate, reload, navigateBack, navigateForward, linkedVrm, setLinkedVrm, unlinkVrm

**webviewSource helpers** (heading format: `# webviewSource.local`):
- webviewSource.local, webviewSource.url, webviewSource.html

**Type guard functions** (heading format: `# isWebviewSourceLocal`):
- isWebviewSourceLocal, isWebviewSourceUrl, isWebviewSourceHtml, isWebviewSourceInfoLocal, isWebviewSourceInfoUrl, isWebviewSourceInfoHtml

### VRM module restructure

The `vrm/` directory already exists with sub-pages but needs restructuring to match the new convention. The VRM module exports a `Vrm` class, a `repeat` namespace, and a `VrmEventSource` class.

**Static methods** (heading format: `# Vrm.spawn`):
- Vrm.spawn, Vrm.findByName, Vrm.waitLoadByName, Vrm.findAllEntities, Vrm.findAllDetailed, Vrm.streamMetadata, Vrm.stream, Vrm.findAll

**Instance methods** (heading format: `# playVrma`):
- events, state, setState, persona, setPersona, name, findBoneEntity, despawn, position, expressions, setExpressions, modifyExpressions, clearExpressions, modifyMouth, springBones, springBone, setSpringBone, listVrma, playVrma, stopVrma, vrmaState, setVrmaSpeed, speakWithTimeline, lookAtCursor, lookAtTarget, unlook

**repeat namespace** (heading format: `# repeat.forever`):
- repeat.forever, repeat.never, repeat.count

**VrmEventSource** (heading format: `# VrmEventSource.on`):
- on, close

**Types**: Bones, PositionResponse, ExpressionsResponse, SpringBoneChainsResponse, SpringBoneChain, SpringBoneProps, VrmaInfo, VrmaPlayRequest, VrmaRepeat, VrmaState, SpawnVrmOptions, VrmSnapshot, VrmMetadata, Persona, SpeakTimelineOptions, TimelineKeyframe, VrmEventSource (EventMap)

## Directory Structure Examples

### entities (namespace-based module)

```
entities/
├── _category_.json
├── index.md
├── findByName.md
├── name.md
├── transform.md
├── setTransform.md
├── move.md
├── tweenPosition.md
├── tweenRotation.md
├── tweenScale.md
└── types.md
```

### audio (nested namespace module)

```
audio/
├── _category_.json
├── index.md
├── se-play.md          # heading: # se.play
├── bgm-play.md         # heading: # bgm.play
├── bgm-stop.md         # heading: # bgm.stop
├── bgm-pause.md        # heading: # bgm.pause
├── bgm-resume.md       # heading: # bgm.resume
├── bgm-update.md       # heading: # bgm.update
├── bgm-status.md       # heading: # bgm.status
└── types.md
```

### webviews (class-based module)

```
webviews/
├── _category_.json
├── index.md
├── open.md              # heading: # Webview.open
├── list.md              # heading: # Webview.list
├── current.md           # heading: # Webview.current
├── close.md             # heading: # close
├── isClosed.md
├── info.md
├── patch.md
├── setOffset.md
├── setSize.md
├── setViewportSize.md
├── navigate.md
├── reload.md
├── navigateBack.md
├── navigateForward.md
├── linkedVrm.md
├── setLinkedVrm.md
├── unlinkVrm.md
├── webviewSource-local.md    # heading: # webviewSource.local
├── webviewSource-url.md      # heading: # webviewSource.url
├── webviewSource-html.md     # heading: # webviewSource.html
├── isWebviewSourceLocal.md
├── isWebviewSourceUrl.md
├── isWebviewSourceHtml.md
├── isWebviewSourceInfoLocal.md
├── isWebviewSourceInfoUrl.md
├── isWebviewSourceInfoHtml.md
└── types.md
```

### math (types-only module)

```
math/
├── _category_.json
├── index.md             # Import + links to types
└── types.md
```

## Page Templates

### index.md (namespace module)

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

### index.md (class-based module)

```markdown
---
sidebar_position: 1
---

# webviews

## Import

\`\`\`typescript
import { Webview, webviewSource } from "@hmcs/sdk";
\`\`\`

## Static Methods

| Method | Description |
|--------|-------------|
| [Webview.open](./open) | Create and open a webview |
| [Webview.list](./list) | Get all open webviews |
| [Webview.current](./current) | Get the current webview |

## Instance Methods

| Method | Description |
|--------|-------------|
| [close](./close) | Close the webview |
| [info](./info) | Get webview info |
| ... | ... |

## Helpers

| Function | Description |
|----------|-------------|
| [webviewSource.local](./webviewSource-local) | Create a local asset source |
| [webviewSource.url](./webviewSource-url) | Create a URL source |
| [webviewSource.html](./webviewSource-html) | Create an inline HTML source |

See also: [Type Definitions](./types)
```

### index.md (commands — separate entry point)

```markdown
---
sidebar_position: 1
---

# commands

## Import

\`\`\`typescript
import { input, output } from "@hmcs/sdk/commands";
\`\`\`

## Input

| Function | Description |
|----------|-------------|
| [input.parse](./input-parse) | Parse and validate JSON from stdin |
| [input.parseMenu](./input-parseMenu) | Parse menu command input |
| [input.read](./input-read) | Read all stdin as string |

## Output

| Function | Description |
|----------|-------------|
| [output.succeed](./output-succeed) | Write result and exit |
| [output.fail](./output-fail) | Write error and exit |
| [output.write](./output-write) | Write JSON to stdout |
| [output.writeError](./output-writeError) | Write error to stderr |

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
| entityId | `number` | Target entity ID |
| request | [`TweenRotationRequest`](./types#tweenrotationrequest) | Tween configuration |

## Returns

`Promise<void>`

## Example

\`\`\`typescript
import { entities } from "@hmcs/sdk";

await entities.tweenRotation(entityId, {
  target: [0, 0.707, 0, 0.707],
  durationMs: 1000,
  easing: "sineInOut",
});
\`\`\`
```

### types.md

```markdown
---
sidebar_position: 100
---

# Type Definitions

## TweenRotationRequest

\`\`\`typescript
interface TweenRotationRequest {
  /** Target rotation as quaternion [x, y, z, w] */
  target: [number, number, number, number];
  /** Duration in milliseconds */
  durationMs: number;
  /** Easing function (default: "linear") */
  easing?: EasingFunction;
  /** Whether to wait for completion (default: false) */
  wait?: boolean;
}
\`\`\`

...
```

## Files to Delete

- `docs/website/docs/mod-development/sdk/tweening.md` (EN)
- `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/sdk/tweening.md` (JA)

All other single-file modules (e.g., `app.md`, `audio.md`) are replaced by their directory equivalents and should be deleted after migration.

## Excluded from This Change

- **quick-start.md**: Tutorial page, not a module reference.
- **index.md** (SDK overview): Content unchanged, but internal links need updating to point to new directory paths.

## Scope

- ~18 modules to split (EN + JA = ~36 directory conversions)
- ~110 function pages to create per language (~220 total)
- ~18 types.md pages per language (~36 total)
- ~18 index.md pages per language (~36 total)
- Total: ~290 new files, ~34 files deleted (17 old .md files x 2 languages)
