# docs/website レビューレポート

**対象**: `docs/website/docs/` 配下 118ファイル（自動生成API リファレンス 59ファイルを除く）
**実施日**: 2026-03-02
**レビュー体制**: 4エージェント（User Reviewer / Developer Reviewer / Integration & Reference Reviewer / Team Lead）

---

## サマリー（重要度別問題数）

| 重要度 | 件数 |
|--------|------|
| High   | 2    |
| Medium | 11   |
| Low    | 16   |
| **合計** | **29** |

*(2026-03-02 更新: 15件を実装差異として修正済み — 詳細は下記 "修正済み" セクション参照)*

---

## セクション別問題点

### Getting Started / Mods

#### [Medium] Quick Start と Installation での MOD 一覧の不整合
- **ファイル**: `docs/getting-started/installation.md:81` vs `docs/getting-started/quick-start.md:46-52`
- **問題**: installation では4MOD、quick-start では voicevox を加えた5MODを紹介。voicevox が突然登場してユーザーが混乱する。
- **推奨**: Quick Start の MOD テーブルに「Recommended / Optional」区分を追加するか、voicevox を "Additional MODs" セクションに分離する。

#### [Medium] MOD 紹介ページの構成が不統一
- **ファイル**: `docs/mods/` 各ファイル
- **問題**: assets / elmer / menu / settings / voicevox で見出し構成がバラバラ。特に voicevox のみ Prerequisites と Troubleshooting を持つが、他ページに共通構造がない。
- **推奨**: 共通テンプレートを定義: `Overview → Usage/Features → Prerequisites（任意）→ Notes → Troubleshooting（任意）`


#### [Low] Assets / Elmer の役割分担の説明が不明確
- **ファイル**: `docs/mods/index.md:14` vs `docs/mods/elmer.md:8`
- **問題**: Assets は「Default VRM character model」を提供し Elmer がそれを使うという役割分担がユーザーに伝わりにくい。
- **推奨**: Assets の説明に「Elmer MOD がこのデータを使ってキャラクターを表示します」を追記する。

#### [Low] getting-started/index.md の「Next Steps」に Quick Start リンクがない
- **ファイル**: `docs/getting-started/index.md:36-38`
- **推奨**: `[Quick Start](/docs/getting-started/quick-start)` へのリンクを追加する。

#### [Low] mods/settings.md の設定項目が具体的でない
- **ファイル**: `docs/mods/settings.md:17-19`
- **問題**: 設定可能な項目の一覧が皆無。
- **推奨**: 設定可能な項目（キャラクター位置、表示サイズ、音量など）を具体的に記載する。

#### [Low] installation.md の CLI インストール時の Windows 対応注記
- **ファイル**: `docs/getting-started/installation.md`
- **問題**: `@hmcs/cli` はネイティブバイナリ配布だが、Windows での挙動が不明確。
- **推奨**: Windows 注意書きで、CLI も含めて Windows 未対応であることを明記する。

---

### MOD Development / SDK

#### [High] mods-api.md vs app.md — `ModInfo` のフィールド名 snake_case / camelCase 混在
- **ファイル**: `mods.ts:48-53` vs `app.ts:51-67`
- **問題**: `mods.ts` の `ModInfo` は `has_main`, `bin_commands`, `asset_ids` (snake_case)。`app.ts` の `InfoMod` は `hasMain`, `binCommands`, `assetIds` (camelCase)。同じ「MOD情報」が2つの API から異なるフィールド名で返される。CLAUDE.md の規約では「HTTP structs は camelCase」であり、mods.ts の snake_case 自体が問題の可能性がある。
- **推奨**: ドキュメント上でこの相違を明記する。長期的には HTTP レスポンスを camelCase に統一することを推奨。

#### [Medium] sdk/index.md — `shadowPanel` モジュールが direct-http.md に埋め込まれている
- **ファイル**: `docs/mod-development/sdk/direct-http.md:111-128`
- **問題**: SDK Overview のモジュールマップでは `shadowPanel` を独立モジュールとして記載しているが、direct-http.md に埋め込まれており独立ページがない。
- **推奨**: `sdk/shadow-panel.md` を作成し、direct-http.md からはリンクのみにする。

#### [Medium] preferences.md — `delete` 操作の欠如と未記載
- **ファイル**: `docs/mod-development/sdk/preferences.md`
- **問題**: `save`, `load`, `list` のみ記載。CLI では `hmcs prefs delete` が存在するが SDK レベルでは未実装の可能性がある。この機能ギャップへの言及がない。
- **推奨**: SDK での deletion 未サポートを明記し、代替手段 (`hmcs prefs delete`) を案内する。

#### [Medium] mods-api.md — `mods.get()` サンプルで snake_case プロパティを使用
- **ファイル**: `docs/mod-development/sdk/mods-api.md:29`
- **問題**: `elmer.asset_ids` を使用しているが、`mods.ts` の `ModInfo` は snake_case であり doc は実装と一致済み。上位の ModInfo snake_case/camelCase 混在問題（High 指摘）として追跡中。
- **ステータス**: 変更不要（実装と一致）

#### [Low] sdk/index.md — `sleep` utility のドキュメント欠如
- **ファイル**: `packages/sdk/src/utils.ts:7`
- **問題**: `sleep()` 関数が SDK から export されているが、どのドキュメントページにも記載がない。
- **推奨**: SDK Overview のモジュールマップに追加する。


#### [Low] displays.md — `GlobalDisplay` の参照先が不正確
- **ファイル**: `docs/mod-development/sdk/displays.md:51`
- **問題**: "See Coordinates for `GlobalDisplay`" と案内しているが、`coordinates.md` には `GlobalDisplay` の型定義がない。実際の定義は `coordinates.ts:79-86`。
- **推奨**: `coordinates.md` に `GlobalDisplay` の型情報を追加するか参照先の説明を修正する。

#### [Low] component-library.md — Storybook コマンド (`pnpm storybook`) の存在確認
- **ファイル**: `docs/mod-development/webview-ui/component-library.md:334`
- **推奨**: `packages/ui/package.json` の scripts に storybook が含まれることを確認し、なければ記載を削除する。

#### [Low] mod-development/index.md — `@hmcs/elmer` の package.json サンプルが実際と乖離
- **ファイル**: `docs/mod-development/index.md:44-63`
- **問題**: サンプルに `elmer:vrm` アセット宣言があるが、実際の `mods/elmer/package.json` の `homunculus` フィールドは空 (`{}`)。
- **推奨**: サンプルコードであることを明記するか、実際の実装と合わせて修正する。


---

### AI Integration / Reference


#### [Medium] AI Integration: セットアップページに `HOMUNCULUS_HOST` 環境変数の設定例が欠落
- **ファイル**: `claude-desktop.md`, `claude-code.md`, `codex.md` (いずれも env 設定なし)
- **問題**: カスタムポート使用時の設定方法が主要クライアントページから欠落。`other-clients.md` のみ記載あり。
- **推奨**: 各セットアップページに「カスタムポート設定」セクションを追加し `env: { "HOMUNCULUS_HOST": "localhost:4000" }` の例を示す。

#### [Medium] Codex 設定: `--mcp-config` のインライン JSON 形式の検証
- **ファイル**: `docs/ai-integration/setup/codex.md:20`
- **問題**: `codex --mcp-config '{"homunculus":...}'` のインライン JSON 形式が実際の Codex CLI でサポートされているか未検証。バージョンによってはファイルパス形式のみ対応の可能性がある。
- **推奨**: Codex 公式ドキュメントで確認し、ファイルパス形式の代替例も併記する。

#### [Medium] MCP Reference: `reference/mcp-tools/index.md` のカテゴリリンクパス検証
- **ファイル**: `docs/reference/mcp-tools/index.md:21-28`
- **問題**: リンクが `./mcp-tools/character` 形式になっており、スラッグオーバーライド (`slug: /reference/mcp-tools`) との組み合わせでリンク解決が正しくない可能性がある。
- **推奨**: `pnpm build` でリンク検証を実施し、壊れている場合は `./character` に修正する。

#### [Medium] Claude Code 設定: 設定ファイルパスの公式ドキュメントとの整合性確認
- **ファイル**: `docs/ai-integration/setup/claude-code.md:32`
- **問題**: `~/.claude/settings.json` の `mcpServers` キーを案内しているが、Claude Code のバージョンアップで設定方法が変わる可能性がある。
- **推奨**: Claude Code 公式ドキュメントへのリンクを追記し、バージョン変更への耐性を持たせる。


#### [Low] AI Integration: MCP サーバーのバージョンがハードコードされている
- **ファイル**: `claude-desktop.md:27`, `claude-code.md:27,39`, `codex.md:20`, `other-clients.md:20`, `troubleshooting.md:59`
- **問題**: すべてのページで `@hmcs/mcp-server@0.1.0` がハードコード。バージョン更新時に全ページの修正が必要。
- **推奨**: `@latest` の使用を検討する。または Docusaurus の変数機能 (MDX 等) でバージョンを一元管理する。


---

### Contributing / Project

#### [High] project/ 配下 4ページ全てがスタブ（フッターから直接リンクあり）
- **ファイル**: `docs/project/changelog.md`, `license.md`, `security.md`, `code-of-conduct.md`
- **問題**: 全て "under construction" のみだが、`docusaurus.config.ts` のフッターから直接リンクされており、ユーザーが空ページに到達する。特に `security.md` が空のままプロダクション公開されているのは不適切。
- **推奨**: `security.md` から優先的に内容を整備する（脆弱性報告先・対応プロセス）。`license.md` には Three-Lane Permissive Model の内容を記載する（SYNTHESIS.md 参照）。

#### [Medium] contributing/index.md — Conventional Commits スタイルへの言及がない
- **ファイル**: `docs/contributing/index.md`
- **問題**: CLAUDE.md では `feat:`, `fix:`, `docs:` 等の Conventional Commits が明記されているが、Contributing ガイドに記載がない。
- **推奨**: PR ガイドラインに「コミットメッセージは Conventional Commits スタイルを使用してください (`feat:`, `fix:`, `docs:` 等)」を追加する。

#### [Medium] contributing/index.md — 開発環境セットアップへのリンク・手順が欠落
- **ファイル**: `docs/contributing/index.md`
- **問題**: 「Fork the repo, create a branch」という記載のみで、実際のセットアップ手順 (`make setup`, `make debug` 等) への案内がない。
- **推奨**: 「Development Setup」セクションを追加し、`make setup` および `make debug` への参照を加える。

#### [Medium] contributing/index.md — DCO/CLA 要件の未記載
- **ファイル**: `docs/contributing/index.md`
- **問題**: ライセンス戦略として DCO 採用を決定済みだが、Contributing ガイドに DCO への言及がない。
- **推奨**: 「By contributing, you certify the DCO (Developer Certificate of Origin)」の旨を追記する。

#### [Low] contributing/index.md — PR 前の CI チェック手順が不明確
- **推奨**: PR 前に `make test` (テスト) と `make fix-lint` (lint) を実行することを明記する。

#### [Low] skills/README.md — Available Skills テーブルが空
- **ファイル**: `skills/README.md:9-10`
- **問題**: Contributing ページからリンクされているが、テーブルが `| | |` の空行のみ。
- **推奨**: 実際のスキルが追加されるまで「No skills available yet. Be the first to contribute!」等のプレースホルダーを置くか、テーブルを削除する。

---

## 横断的問題点

### 構成・一貫性

#### [Medium] `project/` セクションがサイドバーに存在せずフッターのみからアクセス可能
- **ファイル**: `docs/website/sidebars.ts`, `docs/website/docusaurus.config.ts`
- **問題**: changelog, license, security, code-of-conduct はフッターリンクからのみアクセス可能。サイドバーから発見できない。
- **推奨**: サイドバー末尾に `project` セクションを追加することを検討する。

#### [Medium] Asset ID のフォーマットがサイト全体で統一されていない（詳細は AI Integration / Reference セクション参照）
- ドキュメントは `mod-name:asset-id` 形式（`vrma:idle-maid`）、MCP ツールの describe テキストは `mod-name::filename.ext` 形式（`elmer::wave.vrma`）が混在。どちらか一方に統一が必要。

### 実装差異

#### [Medium] MCP サーバーのバージョン `0.1.0` が5箇所でハードコード

### 視認性

特定のページで視認性の問題は少ない。全体的なコードブロックの使い方は適切。ただし以下を指摘:
- スタブページ4件がフッターに露出している（見た目上、コンテンツがないページへのリンクになる）

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
8. **`HOMUNCULUS_HOST` 環境変数をセットアップページに追記**
9. ~~**commands.md の shebang 修正**~~ ✅ 修正済み
10. **Contributing ガイドの充実** — Conventional Commits / DCO / 開発環境セットアップ を追加
11. **MOD ページ構成の統一** — 共通テンプレートを適用
12. **`shadowPanel` の独立ドキュメントページ作成**

### P3 — 中長期対応（Low / 品質向上）

13. **MCP サーバーバージョンのハードコード解消**
14. **project/ ページ（changelog, license, code-of-conduct）の整備**
15. ~~**`sleep` utility のドキュメント追加**~~ ✅ 修正済み（sdk/index.md に utils モジュール行追加）
16. ~~**`speakOnVoiceVox()` への誤った言及を削除**~~ ✅ 修正済み
17. **skills/README.md の空テーブルをプレースホルダーに置換**
18. **preferences.md に delete 操作の代替案（CLI）を追記**
19. ~~**MOD バージョン出力例を `1.0.0` に更新**~~ ✅ 修正済み（mods/index.md, reference/cli/mod.md）
20. **mcp-tools/index.md のリンクパス検証**

---

## 修正済み一覧（2026-03-02）

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
