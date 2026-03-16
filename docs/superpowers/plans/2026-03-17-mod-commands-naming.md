# MOD Commands Naming Unification Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Unify all references to "bin command", "on-demand command", "bin script", etc. to "MOD command(s)" across documentation and code comments.

**Architecture:** Pure text replacement across docs, SDK JSDoc, and Rust doc comments. File rename `bin-commands.md` → `commands.md`. No code identifier changes. Run `make gen-open-api` after Rust changes to update auto-generated files.

**Tech Stack:** Markdown, TypeScript (JSDoc comments), Rust (doc comments), Docusaurus

**Spec:** `docs/superpowers/specs/2026-03-17-mod-commands-naming-design.md`

---

## Replacement Rules Reference

### English
| Current | Replacement |
|---|---|
| "bin command(s)" | "MOD command(s)" |
| "Bin Command(s)" (headings) | "MOD Command(s)" |
| "on-demand command(s)" | "MOD command(s)" |
| "on-demand script(s)" | "MOD command(s)" |
| "bin script(s)" | "MOD command(s)" |

### Japanese
| Current | Replacement |
|---|---|
| "bin コマンド" / "Bin コマンド" | "MOD コマンド" |
| "bin スクリプト" | "MOD コマンド" |
| "オンデマンドコマンド" | "MOD コマンド" |
| "オンデマンドスクリプト" | "MOD コマンド" |

### Exceptions
- "on-demand" OK as supplementary descriptor (e.g., "MOD commands are on-demand scripts that...")
- Text inside code blocks is NOT modified
- Code identifiers are NOT modified
- Comments in code blocks (e.g., `// Find mods that expose bin commands`) inside docs ARE inside code blocks — do NOT modify

### Link & Anchor Rules
| Current | Replacement |
|---|---|
| `./bin-commands.md` | `./commands.md` |
| `./bin-commands` | `./commands` |
| `/mod-development/bin-commands` | `/mod-development/commands` |
| `#bin-commands` | `#mod-commands` |
| `{#bin-commands}` | `{#mod-commands}` |

---

## Chunk 1: File Rename + English Core Docs

### Task 1: Rename bin-commands.md → commands.md (English)

**Files:**
- Rename: `docs/website/docs/mod-development/bin-commands.md` → `commands.md`

- [ ] **Step 1: Rename file**

```bash
cd docs/website/docs/mod-development
git mv bin-commands.md commands.md
```

- [ ] **Step 2: Update frontmatter and headings in commands.md**

In `docs/website/docs/mod-development/commands.md`, apply these replacements:
- Line 2: `title: "Bin Commands"` → `title: "MOD Commands"`
- Line 6: `# Bin Commands` → `# MOD Commands`
- Line 8: `Bin commands are on-demand scripts that MODs expose` → `MOD commands are on-demand scripts that MODs expose`; also second occurrence on the same line: `bin commands run only when explicitly invoked` → `MOD commands run only when explicitly invoked`
- Line 10: link anchor `#bin-commands` → `#mod-commands`
- Line 12: `## Writing a Bin Command Script` → `## Writing a MOD Command Script`
- Line 16: `Every TypeScript bin command must start` → `Every TypeScript MOD command must start`
- Line 32: `Bin commands receive input via stdin as JSON` → `MOD commands receive input via stdin as JSON`
- Line 71: `Bin commands communicate results through stdout and stderr` → `MOD commands communicate results through stdout and stderr`
- Line 177: `Bin commands are invoked via the HTTP API` → `MOD commands are invoked via the HTTP API`
- Line 216: `two convenience functions for calling bin commands` → `two convenience functions for calling MOD commands`
- Line 225: `a complete bin command that builds` → `a complete MOD command that builds`

- [ ] **Step 3: Commit**

```bash
git add -A docs/website/docs/mod-development/commands.md
git commit -m "docs: rename bin-commands.md to commands.md and update headings"
```

### Task 2: Update English docs linking to bin-commands

**Files:**
- Modify: `docs/website/docs/mod-development/menus.md`
- Modify: `docs/website/docs/mod-development/tray-menus.md`
- Modify: `docs/website/docs/mods/voicevox.md`

- [ ] **Step 1: Update menus.md**

In `docs/website/docs/mod-development/menus.md`:
- Line 8: `[bin command](./bin-commands.md)` → `[MOD command](./commands.md)`
- Line 44: `Bin command to execute when selected` → `MOD command to execute when selected`
- Line 52: `the corresponding bin command runs` → `the corresponding MOD command runs`
- Line 78: `[Bin Commands](./bin-commands.md)` → `[MOD Commands](./commands.md)`
- Line 208: `[Bin Commands](./bin-commands.md)` → `[MOD Commands](./commands.md)`, also change link description `Writing and invoking on-demand scripts` → `Writing and invoking MOD commands`

- [ ] **Step 2: Update tray-menus.md**

In `docs/website/docs/mod-development/tray-menus.md`:
- Line 42: `Bin command to execute when selected` → `MOD command to execute when selected`
- Line 169: `[Bin Commands](./bin-commands.md)` → `[MOD Commands](./commands.md)`, also change link description `Writing and invoking on-demand scripts` → `Writing and invoking MOD commands`

- [ ] **Step 3: Update voicevox.md**

In `docs/website/docs/mods/voicevox.md`:
- Line 26: `[bin commands](/mod-development/bin-commands)` → `[MOD commands](/mod-development/commands)`

- [ ] **Step 4: Commit**

```bash
git add docs/website/docs/mod-development/menus.md docs/website/docs/mod-development/tray-menus.md docs/website/docs/mods/voicevox.md
git commit -m "docs: update links from bin-commands to commands"
```

### Task 3: Update English project-setup docs

**Files:**
- Modify: `docs/website/docs/mod-development/project-setup/package-json.md`
- Modify: `docs/website/docs/mod-development/project-setup/directory-structure.md`

- [ ] **Step 1: Update package-json.md**

In `docs/website/docs/mod-development/project-setup/package-json.md`:
- Line 18: `On-demand commands (invoked via HTTP API)` → `MOD commands (invoked via HTTP API)`
- Line 113: `the `open-ui` bin command` → `the `open-ui` MOD command`
- Line 165: `## Bin Commands` → `## MOD Commands`
- Line 167: `on-demand scripts that can be invoked` → `MOD commands that can be invoked`
- Line 177: `Bin commands are invoked via` → `MOD commands are invoked via`
- Line 192: `Bin command names are conventionally prefixed` → `MOD command names are conventionally prefixed`

- [ ] **Step 2: Update directory-structure.md**

In `docs/website/docs/mod-development/project-setup/directory-structure.md`:
- Line 34: `on-demand bin commands` → `MOD commands`
- Lines 38, 40: These are annotations inside a directory-tree code block (`├── package.json # ...`). Although technically inside a fenced block, they are descriptive labels (not executable code), so update them:
  - Line 38: `Declares assets, menus, and bin commands` → `Declares assets, menus, and MOD commands`
  - Line 40: `Bin command script (invoked via HTTP API)` → `MOD command script (invoked via HTTP API)`
- Line 54: `On-demand scripts exposed via the "bin" field` → `MOD commands exposed via the "bin" field`
- Line 65: `on-demand commands and a UI panel. A MOD can also combine both patterns: a service, a UI app, and bin commands` → `MOD commands and a UI panel. A MOD can also combine both patterns: a service, a UI app, and MOD commands`
- Line 83: `Place bin command scripts in` → `Place MOD command scripts in`

- [ ] **Step 3: Commit**

```bash
git add docs/website/docs/mod-development/project-setup/
git commit -m "docs: update project-setup docs to use MOD commands"
```

### Task 4: Update English SDK reference docs

**Files:**
- Modify: `docs/website/docs/mod-development/sdk/index.md`
- Modify: `docs/website/docs/mod-development/sdk/quick-start.md`
- Modify: `docs/website/docs/mod-development/sdk/commands/index.md`
- Modify: `docs/website/docs/mod-development/sdk/commands/output-succeed.md`
- Modify: `docs/website/docs/mod-development/sdk/mods/index.md`
- Modify: `docs/website/docs/mod-development/sdk/mods/executeCommand.md`
- Modify: `docs/website/docs/mod-development/sdk/mods/list.md`
- Modify: `docs/website/docs/mod-development/sdk/mods/types.md`
- Modify: `docs/website/docs/mod-development/sdk/app/types.md`
- Modify: `docs/website/docs/mod-development/sdk/signals/index.md`

- [ ] **Step 1: Update sdk/index.md**

- Line 22: `bin script utilities` → `MOD command utilities`
- Line 37: `execute bin commands` → `execute MOD commands`
- Line 47: `bin command scripts` → `MOD command scripts`
- Line 76: `Node.js bin scripts` → `Node.js MOD command scripts`

- [ ] **Step 2: Update sdk/quick-start.md**

- Line 8: `the commands entry point for bin scripts` → `the commands entry point for MOD command scripts`
- Line 74: `for bin scripts (on-demand commands declared in` → `for MOD command scripts (declared in`
- Line 77: `Node.js bin scripts` → `Node.js MOD command scripts`

- [ ] **Step 3: Update sdk/commands/index.md**

- Line 7: `Stdin/stdout utilities for bin scripts` → `Stdin/stdout utilities for MOD command scripts`; also `output helpers for on-demand commands` → `output helpers for MOD commands`
- Line 10: `bin script contexts` → `MOD command script contexts`

- [ ] **Step 4: Update sdk/commands/output-succeed.md**

- Line 7: `successful bin command` → `successful MOD command`

- [ ] **Step 5: Update sdk/mods/index.md**

- Line 7: `execute bin commands` → `execute MOD commands`
- Line 25: `Run a bin command and collect` → `Run a MOD command and collect`
- Line 26: `Run a bin command and stream` → `Run a MOD command and stream`

- [ ] **Step 6: Update sdk/mods/executeCommand.md**

- Line 7: `Runs a bin command` → `Runs a MOD command`

- [ ] **Step 7: Update sdk/mods/list.md**

- Line 23 is inside a code block (`// Find mods that expose bin commands`) — do NOT modify.

- [ ] **Step 8: Update sdk/mods/types.md, sdk/app/types.md, sdk/signals/index.md**

- `sdk/mods/types.md` line 26: inside a code block showing TypeScript interface — do NOT modify.
- `sdk/app/types.md` line 49: inside a code block showing TypeScript interface — do NOT modify.
- `sdk/signals/index.md` line 7: `bin commands` → `MOD commands`

- [ ] **Step 9: Commit**

```bash
git add docs/website/docs/mod-development/sdk/
git commit -m "docs: update SDK reference docs to use MOD commands"
```

### Task 5: Update remaining English docs

**Files:**
- Modify: `docs/website/docs/mod-development/index.md`
- Modify: `docs/website/docs/mod-development/publishing.md`
- Modify: `docs/website/docs/mod-development/webview-ui/setup-and-build.md`
- Modify: `docs/website/docs/mods/menu.md`
- Modify: `docs/website/docs/reference/mcp-tools/resources.md`
- Modify: `docs/website/docs/reference/mcp-tools/mod.md`

- [ ] **Step 1: Update index.md**

- Line 32: `Expose on-demand commands` → `Expose MOD commands`

- [ ] **Step 2: Update publishing.md**

- Line 25: `bin commands` → `MOD commands`

- [ ] **Step 3: Update setup-and-build.md**

- Line 235: `### Opening via a Bin Command` → `### Opening via a MOD Command`
- Line 237: `Create a bin command to open` → `Create a MOD command to open`
- Line 286: `the `open-ui` bin command` → `the `open-ui` MOD command`

- [ ] **Step 4: Update mods/menu.md**

- Line 18: `a bin command to execute` → `a MOD command to execute`

- [ ] **Step 5: Update reference/mcp-tools/resources.md**

- Line 28: `available bin commands` → `available MOD commands`

- [ ] **Step 6: Update reference/mcp-tools/mod.md**

- Line 8: `bin commands` → `MOD commands`
- Line 12: `MOD bin command` → `MOD command`

- [ ] **Step 7: Commit**

```bash
git add docs/website/docs/mod-development/index.md docs/website/docs/mod-development/publishing.md docs/website/docs/mod-development/webview-ui/setup-and-build.md docs/website/docs/mods/menu.md docs/website/docs/reference/mcp-tools/
git commit -m "docs: update remaining English docs to use MOD commands"
```

### Task 6: Update CLAUDE.md

**Files:**
- Modify: `CLAUDE.md` (desktop_homunculus root)

- [ ] **Step 1: Update CLAUDE.md**

- Line 137: `On-demand scripts are exposed via "bin"` → `MOD commands are exposed via "bin"`

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md to use MOD commands"
```

---

## Chunk 2: Japanese Documentation

### Task 7: Rename bin-commands.md → commands.md (Japanese)

**Files:**
- Rename: `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/bin-commands.md` → `commands.md`

- [ ] **Step 1: Rename file**

```bash
git mv docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/bin-commands.md docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/commands.md
```

- [ ] **Step 2: Update frontmatter and headings in Japanese commands.md**

Apply these replacements in the renamed file:
- Line 2: `title: "Bin コマンド"` → `title: "MOD コマンド"`
- Line 6: `# Bin コマンド` → `# MOD コマンド`
- Line 8: `Bin コマンドは、MOD が...公開するオンデマンドスクリプトです` → `MOD コマンドは、MOD が...公開するオンデマンドスクリプトです`
- Line 10: anchor `#bin-commands` → `#mod-commands`
- Line 12: `## Bin コマンドスクリプトの作成` → `## MOD コマンドスクリプトの作成`
- Line 16: `TypeScript bin コマンドは` → `TypeScript MOD コマンドは`
- Line 32: `Bin コマンドは stdin 経由で` → `MOD コマンドは stdin 経由で`
- Line 71: `Bin コマンドは stdout と stderr を通じて` → `MOD コマンドは stdout と stderr を通じて`
- Line 172: `Bin コマンドは HTTP API 経由で` → `MOD コマンドは HTTP API 経由で`
- Line 211: `bin コマンドを呼び出すための` → `MOD コマンドを呼び出すための`
- Line 220: `bin コマンドです` → `MOD コマンドです`

- [ ] **Step 3: Commit**

```bash
git add docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/commands.md
git commit -m "docs(ja): rename bin-commands.md to commands.md and update headings"
```

### Task 8: Update Japanese docs linking to bin-commands

**Files:**
- Modify: `docs/website/i18n/ja/.../mod-development/menus.md`
- Modify: `docs/website/i18n/ja/.../mod-development/tray-menus.md`
- Modify: `docs/website/i18n/ja/.../mods/voicevox.md`

All paths are under `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/`.

- [ ] **Step 1: Update menus.md (Japanese)**

- Line 8: `[bin コマンド](./bin-commands.md)` → `[MOD コマンド](./commands.md)`
- Line 44: `bin コマンド` → `MOD コマンド`
- Line 52: `bin コマンドが実行され` → `MOD コマンドが実行され`
- Line 78: `[Bin コマンド](./bin-commands.md)` → `[MOD コマンド](./commands.md)`
- Line 208: `[Bin コマンド](./bin-commands.md)` → `[MOD コマンド](./commands.md)`, `オンデマンドスクリプトの作成と呼び出し` → `MOD コマンドの作成と呼び出し`

- [ ] **Step 2: Update tray-menus.md (Japanese)**

- Line 42: `bin コマンド` → `MOD コマンド`
- Line 169: `[Bin コマンド](./bin-commands.md)` → `[MOD コマンド](./commands.md)`, `オンデマンドスクリプトの作成と呼び出し` → `MOD コマンドの作成と呼び出し`

- [ ] **Step 3: Update voicevox.md (Japanese)**

- Line 26: `[bin コマンド](/mod-development/bin-commands)` → `[MOD コマンド](/mod-development/commands)`

- [ ] **Step 4: Commit**

```bash
git add docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/menus.md docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/tray-menus.md docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mods/voicevox.md
git commit -m "docs(ja): update links from bin-commands to commands"
```

### Task 9: Update Japanese project-setup docs

**Files:**
- Modify: `docs/website/i18n/ja/.../mod-development/project-setup/package-json.md`
- Modify: `docs/website/i18n/ja/.../mod-development/project-setup/directory-structure.md`

- [ ] **Step 1: Update package-json.md (Japanese)**

- Line 18: `オンデマンドコマンド` → `MOD コマンド`
- Lines 90, 125: `bin コマンド名` inside code blocks — do NOT modify
- Line 113: `bin コマンドを呼び出します` → `MOD コマンドを呼び出します`
- Line 165: `## Bin コマンド {#bin-commands}` → `## MOD コマンド {#mod-commands}`
- Line 167: `オンデマンドスクリプトを公開します` → `MOD コマンドを公開します`
- Line 177: `Bin コマンドは JSON ボディを持つ` → `MOD コマンドは JSON ボディを持つ`
- Line 192: `Bin コマンド名は慣例的に` → `MOD コマンド名は慣例的に`

- [ ] **Step 2: Update directory-structure.md (Japanese)**

- Line 34: `オンデマンド bin コマンド` → `MOD コマンド`
- Line 38: `bin コマンドを宣言` → `MOD コマンドを宣言`
- Line 40: `bin コマンドスクリプト` → `MOD コマンドスクリプト`
- Line 54: `オンデマンドスクリプトです` → `MOD コマンドです`
- Line 65: `オンデマンドコマンドと UI パネル...bin コマンドの両方` → `MOD コマンドと UI パネル...MOD コマンドの両方`
- Line 83: `bin コマンドスクリプトは` → `MOD コマンドスクリプトは`

- [ ] **Step 3: Commit**

```bash
git add docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/project-setup/
git commit -m "docs(ja): update project-setup docs to use MOD コマンド"
```

### Task 10: Update Japanese SDK reference docs

**Files:**
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/index.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/quick-start.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/commands/index.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/commands/output-succeed.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/mods/index.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/mods/executeCommand.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/mods/list.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/mods/types.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/app/types.md`
- Modify: `docs/website/i18n/ja/.../mod-development/sdk/signals/index.md`

All paths under `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/`.

- [ ] **Step 1: Update sdk/index.md (Japanese)**

- Line 22: `bin スクリプトユーティリティ` → `MOD コマンドユーティリティ`
- Line 37: `bin コマンドの実行` → `MOD コマンドの実行`
- Line 47: `bin コマンドスクリプトで使用する` → `MOD コマンドスクリプトで使用する`
- Line 76: `bin スクリプトでのみ` → `MOD コマンドスクリプトでのみ`

- [ ] **Step 2: Update sdk/quick-start.md (Japanese)**

- Line 8: `bin スクリプト用の commands エントリーポイント` → `MOD コマンドスクリプト用の commands エントリーポイント`
- Line 74: `bin スクリプト（...オンデマンドコマンド）` → `MOD コマンドスクリプト（...MOD コマンド）`
- Line 77: `bin スクリプトでのみ` → `MOD コマンドスクリプトでのみ`

- [ ] **Step 3: Update sdk/commands/index.md (Japanese)**

- Line 7: `bin スクリプト用の stdin/stdout ユーティリティ` → `MOD コマンドスクリプト用の stdin/stdout ユーティリティ`; also `オンデマンドコマンド用の` → `MOD コマンド用の`
- Line 10: `bin スクリプトのコンテキストでのみ` → `MOD コマンドスクリプトのコンテキストでのみ`

- [ ] **Step 4: Update sdk/commands/output-succeed.md (Japanese)**

- Line 7: `bin コマンドの最後の` → `MOD コマンドの最後の`

- [ ] **Step 5: Update sdk/mods/index.md (Japanese)**

- Line 7: `bin コマンドの実行` → `MOD コマンドの実行`
- Line 25: `bin コマンドを実行しバッファリング` → `MOD コマンドを実行しバッファリング`
- Line 26: `bin コマンドを実行しリアルタイム` → `MOD コマンドを実行しリアルタイム`

- [ ] **Step 6: Update sdk/mods/executeCommand.md (Japanese)**

- Line 7: `bin コマンドを実行し` → `MOD コマンドを実行し`

- [ ] **Step 7: Update sdk/mods/list.md (Japanese)**

- Line 23: inside code block (`// bin コマンドを公開している MOD を検索`) — do NOT modify.

- [ ] **Step 8: Update remaining Japanese SDK docs**

- `sdk/mods/types.md` line 26: inside a code block showing TypeScript interface — do NOT modify.
- `sdk/app/types.md` line 49: inside a code block showing TypeScript interface — do NOT modify.
- `sdk/signals/index.md` line 7: `bin コマンド` → `MOD コマンド`

- [ ] **Step 9: Commit**

```bash
git add docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/sdk/
git commit -m "docs(ja): update SDK reference docs to use MOD コマンド"
```

### Task 11: Update remaining Japanese docs

**Files:**
- Modify: `docs/website/i18n/ja/.../mod-development/index.md`
- Modify: `docs/website/i18n/ja/.../mod-development/publishing.md`
- Modify: `docs/website/i18n/ja/.../mod-development/webview-ui/setup-and-build.md`
- Modify: `docs/website/i18n/ja/.../mods/menu.md`
- Modify: `docs/website/i18n/ja/.../reference/mcp-tools/resources.md`
- Modify: `docs/website/i18n/ja/.../reference/mcp-tools/mod.md`

- [ ] **Step 1: Update index.md (Japanese)**

- Line 19: `オンデマンドコマンド` → `MOD コマンド`
- Line 32: `オンデマンドコマンドの公開` → `MOD コマンドの公開`

- [ ] **Step 2: Update publishing.md (Japanese)**

- Line 25: `bin コマンド` → `MOD コマンド`

- [ ] **Step 3: Update setup-and-build.md (Japanese)**

- Line 235: `### Bin コマンドによるオープン` → `### MOD コマンドによるオープン`
- Line 237: `bin コマンドを作成します` → `MOD コマンドを作成します`
- Line 286: `bin コマンドが実行されます` → `MOD コマンドが実行されます`

- [ ] **Step 4: Update mods/menu.md (Japanese)**

- Line 18: `bin コマンドが指定されます` → `MOD コマンドが指定されます`

- [ ] **Step 5: Update reference/mcp-tools/resources.md (Japanese)**

- Line 28: `bin コマンド` → `MOD コマンド`

- [ ] **Step 6: Update reference/mcp-tools/mod.md (Japanese)**

- Line 8: `bin コマンドを実行します` → `MOD コマンドを実行します`
- Line 12: `bin コマンドを実行します` → `MOD コマンドを実行します`

- [ ] **Step 7: Commit**

```bash
git add docs/website/i18n/ja/docusaurus-plugin-content-docs/current/
git commit -m "docs(ja): update remaining Japanese docs to use MOD コマンド"
```

---

## Chunk 3: SDK + Rust + OpenAPI Regeneration

### Task 12: Update TypeScript SDK JSDoc comments

**Files:**
- Modify: `packages/sdk/src/commands.ts`
- Modify: `packages/sdk/src/mods.ts`
- Modify: `packages/sdk/src/app.ts`

- [ ] **Step 1: Update commands.ts**

- Line 84: `bin command scripts` → `MOD command scripts`
- Line 205: `bin command scripts` → `MOD command scripts`
- Line 292: `bin command encounters` → `MOD command encounters`

- [ ] **Step 2: Update mods.ts**

- Line 4: `run on-demand scripts from installed mods` → `run MOD commands from installed mods`
- Line 50: `Available bin command names` → `Available MOD command names`
- Line 64: `available bin commands` → `available MOD commands`
- Line 74: inside code block (`// Find mods with bin commands`) — do NOT modify

- [ ] **Step 3: Update app.ts**

- Line 63: `Available bin command names` → `Available MOD command names`

- [ ] **Step 4: Verify no other occurrences remain**

```bash
grep -rn "bin command\|bin script\|on-demand command\|on-demand script" packages/sdk/src/
```

Expected: no matches outside code blocks.

- [ ] **Step 5: Commit**

```bash
git add packages/sdk/src/commands.ts packages/sdk/src/mods.ts packages/sdk/src/app.ts
git commit -m "docs: update SDK JSDoc comments to use MOD commands"
```

### Task 13: Update Rust doc comments + regenerate OpenAPI

**Files:**
- Modify: `engine/crates/homunculus_utils/src/schema/mods.rs`
- Modify: `engine/crates/homunculus_mod/src/lib.rs`
- Auto-regenerated: `docs/website/static/api/open-api.yml`

- [ ] **Step 1: Update mods.rs**

- Line 22: `/// Available bin command names (paths are not included).` → `/// Available MOD command names (paths are not included).`

- [ ] **Step 2: Update lib.rs**

- Line 12: `//! - On-demand scripts (`bin`) are executed via HTTP API` → `//! - MOD commands (`bin`) are executed via HTTP API`

- [ ] **Step 3: Regenerate OpenAPI spec**

```bash
cd engine && make gen-open-api
```

This updates `docs/website/static/api/open-api.yml` and triggers `pnpm build` to regenerate API reference docs.

- [ ] **Step 4: Verify OpenAPI updated**

```bash
grep "MOD command" docs/website/static/api/open-api.yml
```

Expected: should show `Available MOD command names`.

- [ ] **Step 5: Commit**

```bash
git add engine/crates/homunculus_utils/src/schema/mods.rs engine/crates/homunculus_mod/src/lib.rs docs/website/static/api/open-api.yml
git commit -m "docs: update Rust doc comments to use MOD commands + regenerate OpenAPI"
```

---

## Chunk 4: Verification

### Task 14: Final verification sweep

- [ ] **Step 1: Search for any remaining old terms in English docs**

```bash
grep -rn "bin command\|Bin Command\|bin script\|Bin Script\|on-demand command\|on-demand script" docs/website/docs/ --include="*.md" --include="*.mdx" | grep -v "node_modules\|\.api\.mdx\|StatusCodes\|superpowers"
```

Expected: no matches (or only inside code blocks).

- [ ] **Step 2: Search for any remaining old terms in Japanese docs**

```bash
grep -rn "bin コマンド\|Bin コマンド\|bin スクリプト\|オンデマンドコマンド\|オンデマンドスクリプト" docs/website/i18n/ --include="*.md" | grep -v "node_modules"
```

Expected: no matches (or only inside code blocks).

- [ ] **Step 3: Search for stale links**

```bash
grep -rn "bin-commands" docs/website/ --include="*.md" --include="*.mdx" | grep -v "node_modules\|\.api\.mdx\|StatusCodes\|superpowers"
```

Expected: no matches.

- [ ] **Step 4: Search for stale anchors**

```bash
grep -rn "#bin-commands" docs/website/ --include="*.md" --include="*.mdx" | grep -v "node_modules\|\.api\.mdx\|StatusCodes\|superpowers"
```

Expected: no matches.

- [ ] **Step 5: Search SDK and Rust for remaining terms**

```bash
grep -rn "bin command\|bin script\|on-demand command\|on-demand script" packages/sdk/src/ engine/crates/
```

Expected: no matches outside code blocks.

- [ ] **Step 6: Build verification**

```bash
cd docs/website && pnpm build
```

Expected: build succeeds with no broken link warnings for `commands` pages.
