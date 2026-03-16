# Documentation Review Skill Design

## Overview

`review-docs` は `docs/website/` 内のドキュメントを4つの観点から並列レビューし、チェックボックス形式のレポートを出力するスキル。指摘点の発見のみを目的とし、修正は行わない。

## Trigger

手動呼び出し（`/review-docs`）。ドキュメント変更後に実行する想定。

## Scope

**対象:**
- `docs/website/docs/` 配下の手書きドキュメント（約75ファイル）
- `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/` 配下の日本語ドキュメント

**除外:**
- `docs/website/docs/reference/api/**`（OpenAPI 自動生成）
- `docs/website/node_modules/**`, `build/**`, `.docusaurus/**`

## Architecture

```
.claude/skills/review-docs/
  SKILL.md                          # オーケストレータ
  reviewers/
    code-consistency-reviewer.md    # CC: コード⇔ドキュメント整合性
    structure-reviewer.md           # ST: 構成のわかりやすさ
    writing-quality-reviewer.md     # WQ: 文章の自然さ
    i18n-sync-reviewer.md           # I18N: 多言語同期チェック
```

### SKILL.md（オーケストレータ）

SKILL.md は Claude Code に対するプロンプトテンプレートとして機能する。Claude Code が Agent ツールを使って4つのレビュアーを並列実行する。

1. 4つのレビュアープロンプトファイルを Read ツールで読み込む
2. Agent ツールで4つのエージェントを同一メッセージ内で並列起動する（各エージェントにはレビュアープロンプトの内容をそのまま渡す）
3. 各エージェントの返却テキストを収集
4. findings を重要度別に分類し、レビュアー間で重複する指摘を統合（同一ファイル・同一箇所への指摘は、より適切な観点のレビュアーの指摘を採用する。特に WQ と I18N の境界では、翻訳の古さに起因する問題は I18N、翻訳済みだが文章が不自然な場合は WQ とする）
5. `docs/reviews/YYYY-MM-DD-docs-review.md` に書き出す（同日に複数回実行した場合は `-2`, `-3` のサフィックスを追加）

### レビュアープロンプトファイルの構造

各 `reviewers/*.md` ファイルは、Agent ツールに渡す完全なプロンプトとして機能する。以下の構造を持つ:

```markdown
# {Reviewer Name}

## Role
{このレビュアーの責務の1行説明}

## Scope
{対象ファイルと除外ファイルの明示}

## Check Items
{具体的なチェック項目のリスト}

## Strategy
{ファイル探索の戦略: Glob/Grep で候補を絞り、必要なファイルのみ Read する}

## Output Format
{Finding の出力形式テンプレート}
```

### Reviewer Agent Details

#### CC: Code Consistency Reviewer

**責務:** ソースコードとドキュメントの記述が一致しているかを検証する。

**比較対象:**

| ソースコード | ドキュメント |
|-------------|------------|
| `packages/sdk/src/` | `docs/website/docs/mod-development/sdk/` |
| `packages/mcp-server/src/` | `docs/website/docs/reference/mcp-tools/` |
| `engine/crates/homunculus_cli/` | `docs/website/docs/reference/cli/` |
| `mods/*/package.json` の `homunculus` フィールド | `docs/website/docs/mods/` |

**対象外:** `getting-started/`, `ai-integration/`, `contributing/` はコード整合性チェックの対象外とする。これらのページは概念的な説明が中心であり、特定の API シグネチャとの1対1対応がないため。

**チェック項目:**
- 関数・メソッドのシグネチャ（引数名、型、オプション）
- コマンド名、サブコマンド、フラグ
- 設定キー名
- ドキュメントに記載されているが実コードに存在しない機能
- 実コードに存在するがドキュメントに記載されていない公開API

**探索戦略:** ドキュメント内のコードブロックや API 名を Grep で抽出し、対応するソースファイルで存在確認する。全ファイルの全文読みは行わない。

#### ST: Structure Reviewer

**責務:** エンドユーザーの視点でドキュメントの構成を評価する。

**対象:** `docs/website/docs/` 配下全体（自動生成 API を除く）。`contributing/` と `reference/`（非API）も含む。

**チェック項目:**
- ページ間の論理的な流れ（前提知識が先に説明されているか）
- Getting Started → Mods → MOD Development → AI Integration → Reference → Contributing の全セクション間の導線
- 各ページの見出し構成の一貫性
- `_category_.json` の `position` 値による並び順の妥当性
- 内部リンクの有無（関連ページへの誘導が適切か）
- マークダウン内のリンクが実在するファイルを指しているか

**探索戦略:** `_category_.json` と各ファイルの frontmatter・見出し構造を読み、全体の構成マップを作成してから評価する。ページ本文は見出しと冒頭のみ確認する。

#### WQ: Writing Quality Reviewer

**責務:** 英語・日本語の文章品質を検証する。翻訳の古さに起因する問題は I18N レビュアーの管轄とし、本レビュアーでは扱わない。

**対象:**
- `docs/website/docs/` 配下の英語ドキュメント
- `i18n/ja/docusaurus-plugin-content-docs/current/` 配下の日本語ドキュメント

**チェック項目:**
- 主語の欠落、曖昧な指示語（「これ」「それ」が何を指すか不明）
- 用語の不統一（同じ概念に異なる用語を使用）
- 受動態の過用による分かりにくさ
- 過度に長い文
- コードサンプルと前後の説明の不整合

**探索戦略:** 各ドキュメントを順に読み、文章品質の問題を検出する。ファイル数が多いため、セクションごとに区切って処理する（Getting Started → Mods → MOD Development → AI Integration の順）。

#### I18N: i18n Sync Reviewer

**責務:** 英語版と日本語版の同期状態を検証する。

**比較:**
- `docs/website/docs/` ⇔ `i18n/ja/docusaurus-plugin-content-docs/current/`

**チェック項目:**
- ファイルの1対1対応（片方にしか存在しないファイル）
- セクション構成の差分（見出し構造が一致しているか）
- 英語版のみ更新されており日本語版に反映されていない内容

**探索戦略:** まず Glob で両ディレクトリのファイル一覧を取得し、差分を検出する。次に、対応するファイルペアの見出し構造を比較する。内容の同期チェックは、英語版の各セクション見出しが日本語版にも存在するかで判定する。

## Output Format

レポートは `docs/reviews/YYYY-MM-DD-docs-review.md` に保存する。同日に複数回実行した場合は `YYYY-MM-DD-docs-review-2.md` のようにサフィックスを追加する。

### Finding ID 体系

`[D-{CODE}-NNN]` 形式（CODE は可変長）:

| Code | Perspective |
|------|------------|
| CC | Code Consistency |
| ST | Structure |
| WQ | Writing Quality |
| I18N | i18n Sync |

### Severity 定義

| Level | 定義 |
|-------|------|
| **Critical** | コードと明確に矛盾している、またはユーザーを誤った操作に導く |
| **Major** | 情報の欠落や構成上の問題で、ユーザーが目的を達成しにくい |
| **Minor** | 文章の不自然さや軽微な改善提案 |

### レポートテンプレート

```markdown
# Documentation Review Report

> Date: {YYYY-MM-DD}
> Target: `docs/website/docs/` (excluding `reference/api/`)

## Summary

| Perspective | Critical | Major | Minor |
|-------------|----------|-------|-------|
| Code Consistency | {n} | {n} | {n} |
| Structure | {n} | {n} | {n} |
| Writing Quality | {n} | {n} | {n} |
| i18n Sync | {n} | {n} | {n} |

## Findings

### Critical

- [ ] [D-{CODE}-NNN] **Critical** | `{Perspective}` | `{doc file path}` | {Description}
  > Evidence: {具体的な不一致の内容}
  > Source: `{対応するソースコードのパス:行番号}`（CC のみ）

### Major

- [ ] [D-{CODE}-NNN] **Major** | `{Perspective}` | `{doc file path}` | {Description}
  > Evidence: {具体的な問題の内容}
  > Source: `{対応するソースコードのパス:行番号}`（CC のみ）

### Minor

- [ ] [D-{CODE}-NNN] **Minor** | `{Perspective}` | `{doc file path}` | {Description}
  > Evidence: {具体的な問題の内容}

## Top Recommendations

1. **[D-{CODE}-NNN]** — {対応アクションの概要}
2. ...（最大5件、Critical を優先し、次に Major から影響範囲の広い順に選出）

---
*Generated by Documentation Review | {YYYY-MM-DD}*
```

### エージェント返却形式

各レビュアーエージェントは以下のテキスト形式で findings を返す:

```
[D-CC-001] Critical | `docs/mod-development/sdk/audio.md` | playSound() の引数がソースと不一致
  > Evidence: ドキュメント playSound(assetId) → 実コード playSound(assetId, options?)
  > Source: `packages/sdk/src/audio.ts:42`

[D-CC-002] Major | `docs/reference/cli/mod.md` | hmcs mod install の --global オプションが未記載
  > Evidence: CLI ソースに --global フラグあり、ドキュメントに記載なし
  > Source: `engine/crates/homunculus_cli/src/mod_cmd.rs:78`
```

Findings がない場合は `No findings.` を返す。

## Constraints

- レビューのみ行い、ファイルの修正は一切行わない
- 自動生成 API ドキュメント (`reference/api/`) は対象外
- 各レビュアーは自分の担当 ID プレフィックスのみ使用する
- レポートの findings は重要度順（Critical → Major → Minor）で記載する
- Top Recommendations は最大5件
- 各レビュアーは Glob/Grep で候補を絞り込んでから必要なファイルのみ Read する（全ファイルの全文読みは避ける）
