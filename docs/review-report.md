# docs/website レビューレポート

**対象**: `docs/website/docs/` 配下 118ファイル（自動生成API リファレンス 59ファイルを除く）
**実施日**: 2026-03-02
**レビュー体制**: 4エージェント（User Reviewer / Developer Reviewer / Integration & Reference Reviewer / Team Lead）

---

## サマリー（重要度別問題数）

| 重要度 | 件数 |
|--------|------|
| High   | 1    |
| Medium | 4    |
| Low    | 2    |
| **合計** | **7** |

*(2026-03-03 更新: 22件を追加修正 — 詳細は下記 "修正済み" セクション参照)*

---

## セクション別問題点

### Getting Started / Mods

#### [Medium] Quick Start と Installation での MOD 一覧の不整合
- **ファイル**: `docs/getting-started/installation.md:81` vs `docs/getting-started/quick-start.md:46-52`
- **問題**: installation では4MOD、quick-start では voicevox を加えた5MODを紹介。voicevox が突然登場してユーザーが混乱する。
- **推奨**: Quick Start の MOD テーブルに「Recommended / Optional」区分を追加するか、voicevox を "Additional MODs" セクションに分離する。

#### [Low] mods/settings.md の設定項目が具体的でない
- **ファイル**: `docs/mods/settings.md:17-19`
- **問題**: 設定可能な項目の一覧が皆無。
- **推奨**: 設定可能な項目（キャラクター位置、表示サイズ、音量など）を具体的に記載する。

---

### MOD Development / SDK

#### [Medium] sdk/index.md — `shadowPanel` モジュールが direct-http.md に埋め込まれている
- **ファイル**: `docs/mod-development/sdk/direct-http.md:111-128`
- **問題**: SDK Overview のモジュールマップでは `shadowPanel` を独立モジュールとして記載しているが、direct-http.md に埋め込まれており独立ページがない。
- **推奨**: `sdk/shadow-panel.md` を作成し、direct-http.md からはリンクのみにする。

---

### AI Integration / Reference

#### [Medium] Codex 設定: `--mcp-config` のインライン JSON 形式の検証
- **ファイル**: `docs/ai-integration/setup/codex.md:20`
- **問題**: `codex --mcp-config '{"homunculus":...}'` のインライン JSON 形式が実際の Codex CLI でサポートされているか未検証。バージョンによってはファイルパス形式のみ対応の可能性がある。
- **推奨**: Codex 公式ドキュメントで確認し、ファイルパス形式の代替例も併記する。

---

### Contributing / Project

#### [High] project/ 配下 4ページ全てがスタブ（フッターから直接リンクあり）
- **ファイル**: `docs/project/changelog.md`, `license.md`, `security.md`, `code-of-conduct.md`
- **問題**: 全て "under construction" のみだが、`docusaurus.config.ts` のフッターから直接リンクされており、ユーザーが空ページに到達する。特に `security.md` が空のままプロダクション公開されているのは不適切。
- **推奨**: `security.md` から優先的に内容を整備する（脆弱性報告先・対応プロセス）。`license.md` には Three-Lane Permissive Model の内容を記載する（SYNTHESIS.md 参照）。

---

## 横断的問題点

### 構成・一貫性

#### [Medium] MOD 紹介ページの構成が不統一
- **ファイル**: `docs/mods/` 各ファイル
- **問題**: assets / elmer / menu / settings / voicevox で見出し構成がバラバラ。特に voicevox のみ Prerequisites と Troubleshooting を持つが、他ページに共通構造がない。
- **推奨**: 共通テンプレートを定義: `Overview → Usage/Features → Prerequisites（任意）→ Notes → Troubleshooting（任意）`

#### [Low] `project/` セクションがサイドバーに存在せずフッターのみからアクセス可能
- **ファイル**: `docs/website/sidebars.ts`, `docs/website/docusaurus.config.ts`
- **問題**: changelog, license, security, code-of-conduct はフッターリンクからのみアクセス可能。サイドバーから発見できない。
- **推奨**: サイドバー末尾に `project` セクションを追加することを検討する。

---

## 推奨対応優先順位

### P1 — 即時対応（High / ユーザー体験への直接影響）

1. ~~**CLI `prefs.db` 誤記修正**~~ ✅ 修正済み
2. ~~**bin-commands.md エンドポイント修正**~~ ✅ 修正済み
3. ~~**bin-commands.md タイムアウト値修正**~~ ✅ 修正済み
4. ~~**CLI 実装タイポ修正**~~ ✅ 修正済み
5. **security.md の整備** — 脆弱性報告先・対応プロセスを最低限記載
6. ~~**SDK Node.js バージョン修正**~~ ✅ 修正済み

### P2 — 短期対応（Medium / 開発者・ユーザーの混乱源）

7. ~~**Asset ID フォーマット統一**~~ ✅ 修正済み（MCP ツール describe テキストを `mod-name:asset-id` 形式に統一）
8. ~~**`HOMUNCULUS_HOST` 環境変数をセットアップページに追記**~~ ✅ 修正済み
9. ~~**commands.md の shebang 修正**~~ ✅ 修正済み
10. ~~**Contributing ガイドの充実**~~ ✅ 修正済み（Conventional Commits / DCO / 開発環境セットアップ）
11. **MOD ページ構成の統一** — 共通テンプレートを適用
12. **`shadowPanel` の独立ドキュメントページ作成**

### P3 — 中長期対応（Low / 品質向上）

13. ~~**MCP サーバーバージョンのハードコード解消**~~ ✅ 修正済み（@latest に統一）
14. **project/ ページ（changelog, license, code-of-conduct）の整備**
15. ~~**`sleep` utility のドキュメント追加**~~ ✅ 修正済み
16. ~~**`speakOnVoiceVox()` への誤った言及を削除**~~ ✅ 修正済み
17. ~~**skills/README.md の空テーブルをプレースホルダーに置換**~~ ✅ 修正済み
18. ~~**preferences.md に delete 操作の代替案（CLI）を追記**~~ ✅ 修正済み
19. ~~**MOD バージョン出力例を `1.0.0` に更新**~~ ✅ 修正済み
20. ~~**mcp-tools/index.md のリンクパス検証**~~ ✅ 修正済み
21. ~~**mods/settings.md の設定項目が具体的でない** の修正~~ ✅ 修正済み

---

## 修正済み一覧（2026-03-02 / 2026-03-03）

| # | 重要度 | 対象ファイル | 内容 |
|---|--------|-------------|------|
| 1 | High | `docs/reference/cli/index.md` | `preferences.db` → `prefs.db` |
| 2 | High | `docs/mod-development/bin-commands.md` | タイムアウトデフォルト 10,000 → 30,000ms |
| 3 | High | `docs/mod-development/project-setup/directory-structure.md` | エンドポイント `POST /mods/{mod_name}/bin/{command}` → `POST /commands/execute` |
| 4 | High | `packages/sdk/src/preferences.ts` | JSDoc `null` → `undefined` (load() の戻り値) |
| 5 | Medium | `docs/reference/cli/config.md` | config list 出力例を `key=value` → `KEY VALUE` テーブル形式に修正 |
| 6 | Medium | `docs/reference/cli/prefs.md` | 型推論 integer/float → number に統一 |
| 7 | Medium | `docs/mod-development/sdk/commands.md` | shebang `#!/usr/bin/env node` → `#!/usr/bin/env -S node --experimental-strip-types` |
| 8 | Medium | `docs/mod-development/sdk/commands.md` | Next Steps リンク `../project-setup/package-json#bin-commands` → `../bin-commands` |
| 9 | Medium | `docs/mod-development/sdk/index.md` | "16 modules" → "17 modules"、utils 行追加 |
| 10 | Medium | `docs/mod-development/project-setup/package-json.md` | dependencies Required: Yes → No、文法修正 |
| 11 | Medium | `packages/sdk/package.json` | engines `>=20.0.0` → `>=22.0.0` |
| 12 | Medium | MCP ツール 3ファイル | Asset ID を `mod-name:asset-id` 形式に統一 (`vrm:elmer`, `vrma:idle-maid`, `se:open`) |
| 13 | Low | `engine/crates/homunculus_cli/src/config/get.rs` | タイポ `validd keys` → `valid keys` |
| 14 | Low | `engine/crates/homunculus_cli/src/config/set.rs` | タイポ `Valueid keys` → `valid keys` |
| 15 | Low | `docs/mod-development/sdk/speech.md` | 存在しない `speakOnVoiceVox()` への言及を削除、VoiceVox 連携案内に差し替え |
| 16 | Low | `docs/reference/mcp-tools/prompts.md` | description を実装と一致させる |
| 17 | Low | `docs/mods/index.md`, `docs/reference/cli/mod.md` | hmcs mod list 出力例のバージョン `0.1.0` → `1.0.0` |
| 18 | High | `docs/mod-development/sdk/mods-api.md` | ModInfo の snake_case/camelCase 混在を Warning ノートで文書化 |
| 19 | Medium | `docs/mod-development/sdk/preferences.md` | SDK での delete 未サポートを明記し `hmcs prefs delete` を案内 |
| 20 | Medium | `docs/ai-integration/setup/claude-desktop.md` | Custom Port セクション追加（HOMUNCULUS_HOST env 設定例） |
| 21 | Medium | `docs/ai-integration/setup/claude-code.md` | Custom Port セクション追加（HOMUNCULUS_HOST env 設定例） |
| 22 | Medium | `docs/ai-integration/setup/codex.md` | Custom Port セクション追加（HOMUNCULUS_HOST env 設定例） |
| 23 | Medium | `docs/ai-integration/setup/claude-code.md` | `~/.claude/settings.json` 近くに Claude Code 公式ドキュメントリンク追加 |
| 24 | Medium | `docs/reference/mcp-tools/index.md` | カテゴリリンクパス `./mcp-tools/XXX` → `./XXX` に修正（8件） |
| 25 | Medium | `docs/contributing/index.md` | PR ガイドラインに Conventional Commits スタイル追記 |
| 26 | Medium | `docs/contributing/index.md` | Development Setup セクション追加（`make setup` / `make debug`） |
| 27 | Medium | `docs/contributing/index.md` | DCO セクション追加（Developer Certificate of Origin） |
| 28 | Low | 5ファイル（ai-integration/setup/） | `@hmcs/mcp-server@0.1.0` → `@hmcs/mcp-server@latest` に統一 |
| 29 | Low | `docs/mods/assets.md` | Elmer MOD が Assets MOD に依存する旨を Notes セクションに追記 |
| 30 | Low | `docs/getting-started/index.md` | Next Steps に Quick Start リンク追加 |
| 31 | Low | `docs/getting-started/installation.md` | Step 3 に macOS 専用 CLI の注記（Platform Support）追加 |
| 32 | Low | `docs/mod-development/index.md` | サンプルの asset ID `elmer:vrm` → `vrm:elmer` 修正 + 説明ノート追加 |
| 33 | Low | `docs/contributing/index.md` | PR 前の `make test` / `make fix-lint` 手順を追記 |
| 34 | Low | `skills/README.md` | Available Skills 空テーブルにプレースホルダー追加 |
| 35 | Low | `docs/mods/settings.md` | 設定項目をタブ別テーブルで具体的に記載（Basic / Persona / OCEAN） |
