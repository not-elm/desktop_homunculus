---
title: "アセット ID"
sidebar_position: 3
---

# アセット ID

アセット ID は、MOD システム全体でファイルを参照するための一意な識別子です。VRM モデルの生成、アニメーションの再生、WebView のオープンなど、エンジンにどのファイルを読み込むかを伝える際にアセット ID を使用します。唯一の必須要件は、すべてのアセット ID が**グローバルに一意**であることです。重複した ID は警告としてログに記録され、黙ってスキップされます。

## 推奨される命名規約

推奨される命名規約は以下の通りです：

```
<mod-name>:<asset-name>
```

エンジンはアセット ID を不透明な文字列として扱います。このフォーマットのバリデーションや解析は行いません。ただし、MOD 名をプレフィックスとして付けることで、MOD 間の衝突を防ぎ、ID が自己文書化されます。

| 部分 | ソース | 例 |
|---|---|---|
| `mod-name` | `package.json` の `name` フィールドから導出 | `@hmcs/elmer` は `elmer` になる |
| `asset-name` | `homunculus.assets` オブジェクトのキー | `vrm`、`open`、`ui` |

**MOD 名**はパッケージ名からスコーププレフィックスを除去して抽出されます。例えば：

- `@hmcs/elmer` -- MOD 名は `elmer`
- `@hmcs/settings` -- MOD 名は `settings`
- `my-character` -- MOD 名は `my-character`（除去するスコープなし）

**アセット名**は、`package.json` でアセットを宣言する際にキーとして選んだ文字列です。

### 例

以下の `package.json` の場合：

```json
{
  "name": "@hmcs/elmer",
  "homunculus": {
    "assets": {
      "elmer:vrm": {
        "path": "assets/Elmer.vrm",
        "type": "vrm",
        "description": "VRM model named Elmer"
      },
      "elmer:open": {
        "path": "assets/open.mp3",
        "type": "sound",
        "description": "Sound effect for opening action"
      }
    }
  }
}
```

アセット ID は `elmer:vrm` と `elmer:open` です。SDK や API がアセット参照を期待する場所であればどこでもこれらの文字列を使用できます。

## 組み込みアセット

`@hmcs/assets` MOD はすべての MOD が使用できるデフォルトのアニメーションと効果音のセットを提供します。`@hmcs/assets` がインストールされていれば、以下がすぐに利用できます：

| アセット ID | 型 | 説明 |
|---|---|---|
| `vrma:idle-maid` | `vrma` | 手を前で組んだメイド風の立ちアイドル |
| `vrma:grabbed` | `vrma` | ユーザーにドラッグされている間のリアクションポーズ |
| `vrma:idle-sitting` | `vrma` | 足を揃えた座りアイドルループ |
| `se:open` | `sound` | HUD パネルを開く際の効果音 |

:::tip
組み込みアセットは `@hmcs/assets` パッケージの MOD 名 `vrma` と `se` を使用しています。カスタムのものが必要でない限り、独自のアイドルアニメーションを作成する必要はありません。
:::

## コードでのアセット ID の使用

`@hmcs/sdk` はアセットが必要な場所であればどこでもアセット ID を文字列として受け付けます。

### VRM キャラクターの生成

```typescript
import { Vrm } from "@hmcs/sdk";

const character = await Vrm.spawn("elmer:vrm");
```

### VRMA アニメーションの再生

```typescript
import { repeat } from "@hmcs/sdk";

await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

### WebView のオープン

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";

await Webview.open({
  source: webviewSource.local("settings:ui"),
  size: [1, 0.9],
  viewportSize: [900, 700],
});
```

`webviewSource.local("settings:ui")` ソースは、`settings:ui` アセット ID で登録された HTML ファイルを読み込むようエンジンに指示します。

## HTTP API でのアセット ID の使用

アセット ID は HTTP API のリクエストボディにも登場します。例えば、API 経由で VRM モデルを直接生成する場合：

```bash
curl -X POST http://localhost:3100/vrm/spawn \
  -H "Content-Type: application/json" \
  -d '{"asset": "elmer:vrm"}'
```

同じアセット ID 文字列が SDK と HTTP API の両方で一貫して使用されます。

## 他の MOD のアセットの参照

MOD は他のインストール済み MOD のアセットを参照できます。例えば、キャラクターを生成する MOD は `@hmcs/assets` のアニメーションをよく使用します：

```typescript
// この MOD は @hmcs/assets のアニメーションを使用
await character.playVrma({
  asset: "vrma:idle-maid",  // この MOD ではなく @hmcs/assets で定義
});
```

:::warning
アセットを所有する MOD がインストールされていない場合、そのアセット ID を使用しようとするとエンジンはエラーを返します。アセットを提供する MOD がインストールされていることを確認してください（`hmcs mod install <mod-name>`）。
:::
