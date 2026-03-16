---
sidebar_position: 1
---

# mods

インストール済み MOD の検出、バッファリングまたはストリーミング出力による bin コマンドの実行、登録済みコンテキストメニューエントリの照会を行います。

## インポート

```typescript
import { mods } from "@hmcs/sdk";
```

:::warning フィールド名の規則
`mods.get()` は **snake_case** フィールド名（`has_main`、`bin_commands`、`asset_ids`）の `ModInfo` を返します。これはアプリケーション情報エンドポイントが camelCase フィールド名を返すのとは異なります。フィールド名は将来のリリースで統一される予定です。
:::

## 関数一覧

| 関数 | 説明 |
|------|------|
| [list](./list) | 起動時に検出されたすべての MOD のメタデータを返す |
| [get](./get) | 名前で単一の MOD を取得する |
| [executeCommand](./executeCommand) | bin コマンドを実行しバッファリングされた結果を収集する |
| [streamCommand](./streamCommand) | bin コマンドを実行しリアルタイム出力イベントをストリームする |
| [menus](./menus) | インストール済み MOD 全体のコンテキストメニューエントリをすべて返す |
