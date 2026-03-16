---
title: バージョン管理
sidebar_position: 3
---

# バージョン管理

このガイドでは、Desktop Homunculus モノレポ全体のバージョン管理の仕組みについて説明します。

## 概要

すべてのパッケージのバージョンは、リポジトリルートの **`version.toml`** で一元管理されています。

```toml
version = "0.1.0-alpha.5-dev"

targets = [
    "engine/Cargo.toml",
    "packages/*/package.json",
    "packages/cli-platform/package.json.tmpl",
    "mods/*/package.json",
]

excludes = [
    "sandbox/package.json",
    "docs/website/package.json",
]
```

| フィールド | 説明 |
|-----------|------|
| `version` | 現在のバージョン文字列 |
| `targets` | バージョン管理対象ファイルのグロブパターン |
| `excludes` | バージョン管理から明示的に除外するファイルのグロブパターン |

## バージョンの更新と同期

### 新しいバージョンへの更新

```shell
make bump-version VERSION=0.2.0
```

`version.toml` を新しいバージョンに更新し、すべての対象ファイルに反映します。

### 既存バージョンの再同期

```shell
make bump-version
```

`VERSION` を指定せずに実行すると、`version.toml` の現在のバージョンをすべての対象ファイルに再反映します。新しい対象ファイルを追加した場合や、対象ファイルが同期されていない場合に使用します。

どちらのコマンドも、反映後に `cargo update --workspace` を自動実行して `Cargo.lock` を同期します。

### バージョン形式

バージョンは `X.Y.Z` または `X.Y.Z-<プレリリース>` の形式に従う必要があります：

- `0.2.0` — 安定版リリース
- `0.2.0-alpha.1` — プレリリース
- `0.1.0-alpha.5-dev` — 開発用バージョン（リリース不可。[リリースと CI](#リリースと-ci) を参照）

## バージョン整合性の確認

```shell
make check-version
```

すべての管理対象ファイルが `version.toml` と同期されているか検証します。

**エラー**（終了コード 1）：

- 対象ファイルのバージョンが `version.toml` と一致しない

**警告**（失敗しない）：

- `pnpm-workspace.yaml` に含まれるパッケージが `targets` にも `excludes` にも含まれていない — 新しく追加されたパッケージをバージョン管理に含める必要がある可能性を示す
- `engine/crates/` 内の Rust crate が `version.workspace = true` を使用していない — crate にバージョンがハードコードされている可能性を示す

`docs/website` と `sandbox` は、公開されないプライベートパッケージのため、意図的に `excludes` に含まれています。

:::note Windows ユーザー向け
`check-version` は Unicode 記号（`✓`、`⚠`、`✗` など）を出力します。CP932 エンコーディングを使用する Windows ターミナルでは、`PYTHONUTF8=1` を設定するか UTF-8 対応のターミナルを使用してください。
:::

## バージョン反映の仕組み

バンプスクリプトはファイルの種類に応じて異なる方法でバージョンを更新します：

### Rust

`engine/Cargo.toml` の `[workspace.package].version` フィールドのみを直接更新します。各 crate は自身の `Cargo.toml` で `version.workspace = true` を指定してバージョンを継承します — スクリプトは crate レベルのファイルを変更しません。

### TypeScript

`packages/*/package.json` と `mods/*/package.json` の `"version"` フィールドを直接更新します。

### テンプレート

`packages/cli-platform/package.json.tmpl` はリリース時にプラットフォーム固有のパッケージを生成するためのテンプレートファイルです。スクリプトは `{{VERSION}}` プレースホルダまたは既存のバージョン文字列を置換します。`check-version` は `{{...}}` プレースホルダを不一致として扱いません。

### Cargo.lock

すべてのファイル更新後、スクリプトは `cargo update --workspace` を実行して `Cargo.lock` を更新します。

## 新しい Crate やパッケージの追加

### Rust crate

crate の `Cargo.toml` に `version.workspace = true` を追加します。`version.toml` の変更は不要です — ワークスペースのバージョン継承が自動的に処理します。`make check-version` は、ワークスペースバージョン継承を使用していない crate を警告で検出します。

### TypeScript パッケージ

パッケージが `targets` の既存グロブパターン（例：`packages/*/package.json`）にマッチする場合、変更は不要です。それ以外の場合は、`version.toml` の `targets` に新しいグロブパターンを追加します。

バージョン管理の対象外にしたいパッケージ（プライベートな内部ツールなど）は、`excludes` に追加します。

## リリースと CI

リリースは `v*` タグ（例：`v0.2.0`）の push で起動されます。

リリースワークフロー（`.github/workflows/release.yml`）は以下を検証します：

1. タグが有効な semver バージョンであること
2. タグのバージョン（`v` プレフィックスを除去）が `version.toml` と一致すること

:::warning
CI はタグと `version.toml` の一致のみを検証します。すべての対象ファイルが同期されているかは**確認しません**。リリースタグを作成する前に、`make check-version` をローカルで実行してください。
:::

### npm dist-tag

npm dist-tag はバージョン文字列から自動的に算出されます（`.github/scripts/compute-tag.sh`）：

| バージョンパターン | dist-tag |
|------------------|----------|
| `-alpha` を含む | `alpha` |
| `-beta` を含む | `beta` |
| `-rc` を含む | `rc` |
| それ以外 | `latest` |

:::caution
`-dev` サフィックスを含むバージョン（例：`0.1.0-alpha.5-dev`）は、リリースワークフローがプレリリースセグメント内のハイフンを許可しないため、CI の semver バリデーションに失敗します。`-dev` サフィックスはローカル開発専用です — リリース用バージョンには使用しないでください。
:::
