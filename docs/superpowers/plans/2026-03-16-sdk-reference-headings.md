# SDK Reference Heading Normalization & Page Split

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restructure all SDK reference docs so every function has its own page with its API name as the heading, enabling reverse-lookup by function name in both EN and JA.

**Architecture:** Each SDK module's single `.md` file is replaced by a directory containing `_category_.json`, `index.md` (import + function table), one `.md` per function, and a `types.md`. The same structure is mirrored in the JA i18n directory. Content is extracted from existing docs; JA headings use original English function names.

**Tech Stack:** Docusaurus 3, Markdown, TypeScript SDK source as reference

**Spec:** `docs/superpowers/specs/2026-03-16-sdk-reference-headings-design.md`

---

## Key Paths

- **EN docs:** `docs/website/docs/mod-development/sdk/`
- **JA docs:** `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/sdk/`
- **SDK source:** `packages/sdk/src/`

## Conventions (apply to ALL tasks)

### _category_.json

```json
{
  "label": "<module-name>",
  "position": <same sidebar_position as the old .md file>
}
```

### index.md template

```markdown
---
sidebar_position: 1
---

# <module-name>

<1-line module description from old file>

## Import

\`\`\`typescript
import { <module> } from "@hmcs/sdk";
\`\`\`

## Functions

| Function | Description |
|----------|-------------|
| [functionName](./functionName) | 1-line description |

See also: [Type Definitions](./types)
```

JA index.md: same structure, `# <module-name>` heading stays English, description and table descriptions in Japanese, `## Import` stays as-is (code blocks are never translated).

### Function page template

```markdown
---
sidebar_position: N
---

# functionName

<description paragraph>

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| param | `type` | description |

## Returns

`Promise<ReturnType>`

## Example

\`\`\`typescript
// code example
\`\`\`
```

JA function page: `# functionName` stays English. Description, parameter descriptions, and comments in code examples are in Japanese.

### types.md template

```markdown
---
sidebar_position: 100
---

# Type Definitions

## TypeName

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| field | `type` | `default` | description |
```

JA types.md: `# Type Definitions` → `# 型定義`. Type names stay English. Field descriptions in Japanese.

### Naming conventions

| Pattern | Filename | Heading |
|---------|----------|---------|
| Namespace function | `functionName.md` | `# functionName` |
| Nested namespace (audio.bgm.play) | `bgm-play.md` | `# bgm.play` |
| Class static method (Webview.open) | `open.md` | `# Webview.open` |
| Class instance method (close) | `close.md` | `# close` |
| webviewSource helper | `webviewSource-local.md` | `# webviewSource.local` |
| Type guard | `isWebviewSourceLocal.md` | `# isWebviewSourceLocal` |

### sidebar_position for function pages

Function pages within a module are numbered starting at 2, in the same order as listed in the task's function list. `types.md` always gets `sidebar_position: 100`. Example for `app/`: `health.md` = 2, `info.md` = 3, `exit.md` = 4, `types.md` = 100.

### Handling missing content

Some functions (especially in the VRM module) may not have corresponding sections in existing documentation. For these, write content from the JSDoc comments in the SDK source file (`packages/sdk/src/<module>.ts`). Include: description, parameters, return type, and a minimal code example.

### Cross-link breakage between chunks

Broken cross-links are expected during execution. Old `.md` files are deleted per-task, but `sdk/index.md` and `quick-start.md` links are not updated until Task 20. This is by design — do not attempt to fix cross-links within individual module tasks.

### Process for each module

1. Read the existing EN `.md` file for content
2. Read the existing JA `.md` file for translated content
3. Read the SDK source file (`packages/sdk/src/<module>.ts`) for accurate signatures
4. Create EN directory: `_category_.json`, `index.md`, function pages, `types.md`
5. Create JA directory: same structure, JA body text, EN function-name headings
6. Commit: `docs: split <module> SDK reference into per-function pages`

---

## Chunk 1: Small Namespace Modules

These modules have 1-3 functions each. Each task creates a directory for both EN and JA.

### Task 1: app

**Files:**
- Read: `sdk/app.md` (EN), `sdk/app.md` (JA), `packages/sdk/src/app.ts`
- Create EN: `sdk/app/_category_.json`, `sdk/app/index.md`, `sdk/app/health.md`, `sdk/app/info.md`, `sdk/app/exit.md`, `sdk/app/types.md`
- Create JA: same structure under JA path
- Delete: `sdk/app.md` (EN), `sdk/app.md` (JA)

- [ ] **Step 1: Read source files** — Read EN `app.md`, JA `app.md`, and `packages/sdk/src/app.ts`
- [ ] **Step 2: Create EN directory** — Create `sdk/app/` with `_category_.json` (position: 15), `index.md`, `health.md`, `info.md`, `exit.md`, `types.md` (types: AppInfo, PlatformInfo, InfoMod). Extract content from existing `app.md`.
- [ ] **Step 3: Create JA directory** — Same structure. JA body text, EN headings.
- [ ] **Step 4: Delete old files** — Remove `sdk/app.md` (EN and JA)
- [ ] **Step 5: Commit** — `docs: split app SDK reference into per-function pages`

### Task 2: assets

**Files:**
- Read: `sdk/assets-api.md` (EN/JA), `packages/sdk/src/assets.ts`
- Create: `sdk/assets/` (EN/JA) — `_category_.json` (position: 13), `index.md`, `list.md`, `types.md` (types: AssetType, AssetInfo, AssetFilter)
- Delete: `sdk/assets-api.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split assets SDK reference into per-function pages`

### Task 3: coordinates

**Files:**
- Read: `sdk/coordinates.md` (EN/JA), `packages/sdk/src/coordinates.ts`
- Create: `sdk/coordinates/` (EN/JA) — `_category_.json` (position: 10), `index.md`, `toWorld.md`, `toViewport.md`, `types.md` (types: GlobalDisplay, GlobalViewport). Note: `coordinates` owns these types; other modules link here.
- Delete: `sdk/coordinates.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split coordinates SDK reference into per-function pages`

### Task 4: displays

**Files:**
- Read: `sdk/displays.md` (EN/JA), `packages/sdk/src/displays.ts`
- Create: `sdk/displays/` (EN/JA) — `_category_.json` (position: 12), `index.md`, `findAll.md`, no `types.md` (link to `../coordinates/types#globaldisplay` instead)
- Delete: `sdk/displays.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split displays SDK reference into per-function pages`

### Task 5: effects

**Files:**
- Read: `sdk/effects.md` (EN/JA), `packages/sdk/src/effects.ts`
- Create: `sdk/effects/` (EN/JA) — `_category_.json` (position: 8), `index.md`, `stamp.md`, `types.md` (types: StampOptions, StampRequestBody)
- Delete: `sdk/effects.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split effects SDK reference into per-function pages`

### Task 6: math

**Files:**
- Read: `sdk/math.md` (EN/JA), `packages/sdk/src/math.ts`
- Create: `sdk/math/` (EN/JA) — `_category_.json` (position: 17), `index.md` (import + link to types), `types.md` (types: Transform, TransformArgs, Vec2, Vec3, Quat, Rect). No function pages — math has no functions.
- Delete: `sdk/math.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split math SDK reference into per-type pages`

### Task 7: preferences

**Files:**
- Read: `sdk/preferences.md` (EN/JA), `packages/sdk/src/preferences.ts`
- Create: `sdk/preferences/` (EN/JA) — `_category_.json` (position: 11), `index.md`, `list.md`, `load.md`, `save.md`. No `types.md` (uses generic JSON).
- Delete: `sdk/preferences.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split preferences SDK reference into per-function pages`

### Task 8: settings

**Files:**
- Read: `sdk/settings.md` (EN/JA), `packages/sdk/src/settings.ts`
- Create: `sdk/settings/` (EN/JA) — `_category_.json` (position: 16.3), `index.md`, `fps.md`, `setFps.md`, `types.md` (types: SetFpsBody)
- Delete: `sdk/settings.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split settings SDK reference into per-function pages`

### Task 9: shadowPanel

**Files:**
- Read: `sdk/shadow-panel.md` (EN/JA), `packages/sdk/src/shadowPanel.ts`
- Create: `sdk/shadow-panel/` (EN/JA) — `_category_.json` (position: 16.5), `index.md`, `alpha.md`, `setAlpha.md`, `types.md` (types: ShadowPanelPutBody)
- Delete: `sdk/shadow-panel.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split shadowPanel SDK reference into per-function pages`

### Task 10: signals

**Files:**
- Read: `sdk/signals.md` (EN/JA), `packages/sdk/src/signals.ts`
- Create: `sdk/signals/` (EN/JA) — `_category_.json` (position: 9), `index.md`, `list.md`, `stream.md`, `send.md`, `types.md` (types: SignalChannelInfo)
- Delete: `sdk/signals.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split signals SDK reference into per-function pages`

### Task 11: speech

**Files:**
- Read: `sdk/speech.md` (EN/JA), `packages/sdk/src/speech.ts`
- Create: `sdk/speech/` (EN/JA) — `_category_.json` (position: 6), `index.md`, `fromPhonemes.md`, `types.md` (types: TimelineKeyframe)
- Delete: `sdk/speech.md` (EN/JA)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split speech SDK reference into per-function pages`

### Task 12: utils

**Files:**
- Read: `packages/sdk/src/utils.ts` (no existing docs page)
- Create: `sdk/utils/` (EN/JA) — `_category_.json` (position: 18.5), `index.md`, `sleep.md`. No `types.md`.
- Special: `sleep` is a bare export, not a namespace function. Import is `import { sleep } from "@hmcs/sdk"`. Heading is `# sleep`.

- [ ] **Step 1: Read SDK source** — Read `packages/sdk/src/utils.ts` for the function signature and JSDoc. There is no existing docs page — derive description, parameters, return type, and example entirely from the source JSDoc.
- [ ] **Step 2: Create EN directory** — Write new docs from SDK source
- [ ] **Step 3: Create JA directory** — Write JA translation
- [ ] **Step 4: Commit** — `docs: add utils SDK reference pages`

---

## Chunk 2: Medium/Large Namespace Modules

These modules have 4+ functions and require more careful content extraction.

### Task 13: audio

**Files:**
- Read: `sdk/audio.md` (EN/JA), `packages/sdk/src/audio.ts`
- Create: `sdk/audio/` (EN/JA) — `_category_.json` (position: 5), `index.md`, `se-play.md` (heading: `# se.play`), `bgm-play.md` (heading: `# bgm.play`), `bgm-stop.md`, `bgm-pause.md`, `bgm-resume.md`, `bgm-update.md`, `bgm-status.md`, `types.md` (types: SeOptions, BgmPlayOptions, BgmStopOptions, BgmUpdateOptions, FadeTween, BgmStatus)
- Delete: `sdk/audio.md` (EN/JA)

Content mapping from existing `audio.md`:
- `## Sound Effects` section → `se-play.md`
- `### Play` (under BGM) → `bgm-play.md`
- `### Stop` → `bgm-stop.md`
- `### Pause and Resume` → split into `bgm-pause.md` and `bgm-resume.md`
- `### Update` → `bgm-update.md`
- `### Status` → `bgm-status.md`
- `## Types` section → `types.md`

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory** — Extract and split content per the mapping above
- [ ] **Step 3: Create JA directory** — Same split, JA body text, EN headings (e.g., `# bgm.play` not `# 再生`)
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split audio SDK reference into per-function pages`

### Task 14: commands

**Files:**
- Read: `sdk/commands.md` (EN/JA), `packages/sdk/src/commands.ts`
- Create: `sdk/commands/` (EN/JA) — `_category_.json` (position: 18), `index.md`, `input-parse.md` (heading: `# input.parse`), `input-parseMenu.md` (heading: `# input.parseMenu`), `input-read.md` (heading: `# input.read`), `output-succeed.md` (heading: `# output.succeed`), `output-fail.md` (heading: `# output.fail`), `output-write.md` (heading: `# output.write`), `output-writeError.md` (heading: `# output.writeError`), `types.md` (types: StdinParseError)
- Delete: `sdk/commands.md` (EN/JA)
- **Special import:** `import { input, output } from "@hmcs/sdk/commands"` (not `@hmcs/sdk`)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split commands SDK reference into per-function pages`

### Task 15: entities (+ tweening merge)

**Files:**
- Read: `sdk/entities.md` (EN/JA), `sdk/tweening.md` (EN/JA), `packages/sdk/src/entities.ts`
- Create: `sdk/entities/` (EN/JA) — `_category_.json` (position: 3), `index.md`, `findByName.md`, `name.md`, `transform.md`, `setTransform.md`, `move.md`, `tweenPosition.md`, `tweenRotation.md`, `tweenScale.md`, `types.md` (types: FindOptions, MoveTarget, MoveTargetWorld, MoveTargetViewport, TweenPositionRequest, TweenRotationRequest, TweenScaleRequest, EasingFunction)
- Delete: `sdk/entities.md` (EN/JA), `sdk/tweening.md` (EN/JA)

Content mapping:
- `entities.md` `### findByName` → `findByName.md`
- `entities.md` `### Entity Names` section → merge into `name.md` context
- `entities.md` `### Reading` (Transform) → `transform.md`
- `entities.md` `### Writing` (Transform) → `setTransform.md`
- `entities.md` `### World Coordinates` + `### Viewport Coordinates` → `move.md`
- `entities.md` `### tweenPosition/tweenRotation/tweenScale` → respective pages
- `tweening.md` `## Position Tween` / `## Rotation Tween` / `## Scale Tween` → merge into `tweenPosition.md` / `tweenRotation.md` / `tweenScale.md` (tweening.md has more detailed examples)
- Both entities.md and tweening.md Types sections → `types.md`

- [ ] **Step 1: Read all source files** — Read both `entities.md` and `tweening.md` (EN/JA) plus SDK source
- [ ] **Step 2: Create EN directory** — Merge content from both files per mapping
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files** — Delete `entities.md` AND `tweening.md` (EN/JA)
- [ ] **Step 5: Commit** — `docs: split entities SDK reference into per-function pages, merge tweening`

### Task 16: host

**Files:**
- Read: `sdk/direct-http.md` (EN/JA), `packages/sdk/src/host.ts`
- Create: `sdk/host/` (EN/JA) — `_category_.json` (position: 16), `index.md`, `configure.md`, `base.md`, `baseUrl.md`, `createUrl.md`, `get.md`, `post.md`, `put.md`, `patch.md`, `deleteMethod.md`, `postStream.md`, `types.md` (types: HomunculusApiError, HomunculusStreamError)
- Delete: `sdk/direct-http.md` (EN/JA)
- **Special import:** `import { host, HomunculusApiError, HomunculusStreamError } from "@hmcs/sdk"` — error classes are top-level exports outside the `host` namespace.

Content mapping from `direct-http.md`:
- `## Configuration` → `configure.md`
- `## URL Construction` → `createUrl.md`, `base.md`, `baseUrl.md`
- `## Making Requests` → split into `get.md`, `post.md`, `put.md`, `patch.md`, `deleteMethod.md`
- `## Streaming (NDJSON)` → `postStream.md`
- `## Error Handling` → `types.md`

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split host SDK reference into per-function pages`

### Task 17: mods

**Files:**
- Read: `sdk/mods-api.md` (EN/JA), `packages/sdk/src/mods.ts`
- Create: `sdk/mods/` (EN/JA) — `_category_.json` (position: 14), `index.md`, `list.md`, `get.md`, `executeCommand.md`, `streamCommand.md`, `menus.md`, `types.md` (types: ModInfo, ExecuteCommandRequest, CommandEvent, CommandStdoutEvent, CommandStderrEvent, CommandExitEvent, CommandResult, ModMenuMetadata)
- Delete: `sdk/mods-api.md` (EN/JA)

Content mapping:
- `## Listing MODs` → `list.md` (+ `get.md` if documented, otherwise write from SDK source)
- `## Executing Commands (Buffered)` → `executeCommand.md`
- `## Executing Commands (Streaming)` → `streamCommand.md`
- `## Menu Metadata` → `menus.md`

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory**
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split mods SDK reference into per-function pages`

---

## Chunk 3: Class-Based Modules

### Task 18: webviews

**Files:**
- Read: `sdk/webviews.md` (EN/JA), `packages/sdk/src/webviews.ts`
- Create: `sdk/webviews/` (EN/JA) with these pages:
- `_category_.json` (position: 7)
- `index.md` — grouped table: Static Methods, Instance Methods, Helpers, Type Guards
- Static methods (heading `# Webview.<method>`): `open.md`, `list.md`, `current.md`
- Instance methods (heading `# <method>`): `close.md`, `isClosed.md`, `info.md`, `patch.md`, `setOffset.md`, `setSize.md`, `setViewportSize.md`, `navigate.md`, `reload.md`, `navigateBack.md`, `navigateForward.md`, `linkedVrm.md`, `setLinkedVrm.md`, `unlinkVrm.md`
- webviewSource helpers (heading `# webviewSource.<name>`): `webviewSource-local.md`, `webviewSource-url.md`, `webviewSource-html.md`
- Type guards (heading `# <name>`): `isWebviewSourceLocal.md`, `isWebviewSourceUrl.md`, `isWebviewSourceHtml.md`, `isWebviewSourceInfoLocal.md`, `isWebviewSourceInfoUrl.md`, `isWebviewSourceInfoHtml.md`
- `types.md` (types: WebviewSource, WebviewSourceLocal, WebviewSourceUrl, WebviewSourceHtml, WebviewSourceInfo, WebviewSourceInfoLocal, WebviewSourceInfoUrl, WebviewSourceInfoHtml, WebviewInfo, WebviewPatchRequest, WebviewOpenOptions, WebviewNavigateRequest, SetLinkedVrmRequest)
- Delete: `sdk/webviews.md` (EN/JA)

Content mapping from `webviews.md`:
- `## WebView Sources` subsections → `webviewSource-local.md`, `webviewSource-url.md`, `webviewSource-html.md`
- `## Opening a WebView` → `open.md`
- `### List All` → `list.md`
- `### Current WebView` → `current.md`
- `## Navigation` subsections → `navigate.md`, `reload.md`, `navigateBack.md`, `navigateForward.md`
- `### Get Info` → `info.md`
- `### Update Properties` → `patch.md`, `setOffset.md`, `setSize.md`, `setViewportSize.md`
- `## VRM Linking` → `linkedVrm.md`, `setLinkedVrm.md`, `unlinkVrm.md`
- `## Lifecycle` → `close.md`, `isClosed.md`
- Type guard functions: write new content from SDK source (not in current docs)

- [ ] **Step 1: Read source files**
- [ ] **Step 2: Create EN directory** — Split existing content + write new type guard pages from SDK source
- [ ] **Step 3: Create JA directory**
- [ ] **Step 4: Delete old files**
- [ ] **Step 5: Commit** — `docs: split webviews SDK reference into per-function pages`

### Task 19: vrm (restructure)

The VRM module already has sub-pages (`vrm/spawn-and-find.md`, `vrm/animations.md`, etc.) but they group multiple functions under descriptive headings. This task restructures to 1-function-per-page.

**Files:**
- Read: All existing `sdk/vrm/*.md` (EN/JA), `packages/sdk/src/vrm.ts`
- Create: Replace existing sub-pages with per-function pages:
- `index.md` — rewrite with function table (keep sidebar_position: 1)
- Static methods: `spawn.md` (heading: `# Vrm.spawn`), `findByName.md` (`# Vrm.findByName`), `waitLoadByName.md`, `findAllEntities.md`, `findAllDetailed.md`, `streamMetadata.md`, `stream.md`, `findAll.md`
- Instance methods: `events.md`, `state.md`, `setState.md`, `persona.md`, `setPersona.md`, `name.md`, `findBoneEntity.md`, `despawn.md`, `position.md`, `expressions.md`, `setExpressions.md`, `modifyExpressions.md`, `clearExpressions.md`, `modifyMouth.md`, `springBones.md`, `springBone.md`, `setSpringBone.md`, `listVrma.md`, `playVrma.md`, `stopVrma.md`, `vrmaState.md`, `setVrmaSpeed.md`, `speakWithTimeline.md`, `lookAtCursor.md`, `lookAtTarget.md`, `unlook.md`
- repeat namespace: `repeat-forever.md` (heading: `# repeat.forever`), `repeat-never.md`, `repeat-count.md`
- VrmEventSource: `VrmEventSource-on.md` (heading: `# VrmEventSource.on`), `VrmEventSource-close.md`
- `types.md` — consolidated from types sections across all current sub-pages
- Delete: `vrm/spawn-and-find.md`, `vrm/animations.md`, `vrm/expressions.md`, `vrm/events.md`, `vrm/look-at.md`, `vrm/persona.md`, `vrm/speech-timeline.md` (EN/JA)
- Keep: `vrm/_category_.json` (position: 2.5, label: VRM) — do not modify
- Overwrite: `vrm/index.md` — rewrite in place (do not delete and recreate)
- Do same for JA.

Content mapping from existing sub-pages:
- `spawn-and-find.md` → `spawn.md`, `findByName.md`, `waitLoadByName.md`, `findAllEntities.md`, `findAllDetailed.md`, `streamMetadata.md`, `stream.md`, `findAll.md`, `despawn.md`, `position.md`, `state.md`, `setState.md`, `name.md`, `findBoneEntity.md`
- `animations.md` → `playVrma.md`, `stopVrma.md`, `vrmaState.md`, `setVrmaSpeed.md`, `listVrma.md`, `springBones.md`, `springBone.md`, `setSpringBone.md`, `repeat-forever.md`, `repeat-never.md`, `repeat-count.md`
- `expressions.md` → `expressions.md`, `setExpressions.md`, `modifyExpressions.md`, `clearExpressions.md`, `modifyMouth.md`
- `events.md` → `events.md` (returns VrmEventSource), `VrmEventSource-on.md`, `VrmEventSource-close.md`
- `look-at.md` → `lookAtCursor.md`, `lookAtTarget.md`, `unlook.md`
- `persona.md` → `persona.md`, `setPersona.md`
- `speech-timeline.md` → `speakWithTimeline.md`

Types to consolidate into `types.md`:
- Core: Bones, PositionResponse, SpawnVrmOptions, VrmSnapshot, VrmMetadata, MoveToArgs, LookAtState
- Persona: Persona, Ocean
- Expressions: ExpressionsResponse, ExpressionInfo, OverrideType
- Animation: VrmaInfo, VrmaPlayRequest, VrmaRepeat, VrmaState, VrmaSpeedBody
- Spring Bone: SpringBoneChainsResponse, SpringBoneChain, SpringBoneProps
- Speech: SpeakTimelineOptions, TimelineKeyframe
- Events: VrmEventSource, EventMap, VrmPointerEvent, VrmDragEvent, VrmMouseEvent, Button, VrmStateChangeEvent, PersonaChangeEvent

- [ ] **Step 1: Read all existing VRM sub-pages** (EN/JA) and SDK source
- [ ] **Step 2: Create EN function pages** — Extract content from existing sub-pages per mapping
- [ ] **Step 3: Create EN types.md** — Consolidate all type definitions from sub-pages
- [ ] **Step 4: Rewrite EN index.md** — Overwrite in place. Import block: `import { Vrm, repeat, VrmEventSource } from "@hmcs/sdk"`. Function table grouped as: Static Methods, Instance Methods, repeat namespace, VrmEventSource.
- [ ] **Step 5: Delete old EN sub-pages** — Remove `spawn-and-find.md`, `animations.md`, `expressions.md`, `events.md`, `look-at.md`, `persona.md`, `speech-timeline.md`
- [ ] **Step 6: Repeat steps 2-5 for JA**
- [ ] **Step 7: Commit** — `docs: restructure vrm SDK reference to per-function pages`

---

## Chunk 4: Cleanup & Verification

### Task 20: Update SDK index and cross-references

**Files:**
- Modify: `sdk/index.md` (EN/JA) — Update all module links from `./audio` to `./audio/` etc.
- Modify: `sdk/quick-start.md` (EN/JA) — Update any SDK module links
- Scan: All newly created function pages for broken cross-module links (e.g., `../coordinates/types#globaldisplay`)

- [ ] **Step 1: Update EN sdk/index.md** — Change links to point to new directory paths
- [ ] **Step 2: Update JA sdk/index.md**
- [ ] **Step 3: Check quick-start.md** (EN/JA) for any module links that need updating
- [ ] **Step 4: Verify all cross-module type links** — Check that links like `../coordinates/types#globaldisplay` resolve correctly
- [ ] **Step 5: Commit** — `docs: update SDK index and cross-references for new structure`

### Task 21: Docusaurus build verification

- [ ] **Step 1: Run EN build** — `cd docs/website && pnpm build`
- [ ] **Step 2: Verify EN-JA parity** — Compare EN and JA directory structures to confirm identical file sets: `diff <(cd docs/website/docs/mod-development/sdk && find . -name '*.md' | sort) <(cd docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/sdk && find . -name '*.md' | sort)`
- [ ] **Step 3: Fix any broken links or build errors**
- [ ] **Step 4: Commit fixes if any** — `docs: fix broken links from SDK restructure`

---

## Parallelization Guide

All module tasks (1-19) are fully independent — no task modifies a file that another task reads or writes. They can all run in parallel if sufficient subagents are available.

Recommended batching for practical concurrency:

- **Batch A** (Tasks 1-12): Small modules, 12 tasks in parallel
- **Batch B** (Tasks 13-19): Medium/large modules, 7 tasks in parallel
- **Batch C** (Tasks 20-21): Cleanup, sequential (Task 20 before 21)

Batch A and Batch B can also run simultaneously if desired. Only Batch C must wait for all prior tasks to complete.

Maximum parallelism: 19 concurrent subagents (all module tasks at once).
