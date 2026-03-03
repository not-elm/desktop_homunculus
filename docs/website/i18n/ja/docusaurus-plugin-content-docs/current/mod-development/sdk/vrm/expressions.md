---
title: "表情"
sidebar_position: 3
---

# 表情

VRM キャラクターの表情を制御します。表情は名前付きのブレンドシェイプ -- `happy`、`sad`、`angry`、`blink`、および `aa`、`ih`、`oh` などの口の形状 -- で、ウェイト値は 0.0 から 1.0 です。

## インポート

```typescript
import { Vrm } from "@hmcs/sdk";
```

## 表情の設定

`setExpressions(weights)` は現在の表情オーバーライドを**すべて**置換します。レコードに含まれない表情は VRMA アニメーションの制御に戻ります。

```typescript
const character = await Vrm.findByName("MyAvatar");

// happy と blink をオーバーライド -- 他のすべての表情はアニメーションに戻る
await character.setExpressions({ happy: 1.0, blink: 0.5 });
```

## 表情の変更

`modifyExpressions(weights)` は**部分的な更新**を行います -- 指定された表情のみが変更されます。呼び出しに記載されていない既存のオーバーライドはそのまま維持されます。

```typescript
// まず happy を設定
await character.modifyExpressions({ happy: 1.0 });

// 次に happy のオーバーライドを除去せずに blink を追加
await character.modifyExpressions({ blink: 1.0 });
// 結果：happy=1.0、blink=1.0
```

:::tip
どの表情をオーバーライドするか完全に制御したい場合は `setExpressions` を使用してください。他のオーバーライドを変更せずに変更をレイヤリングしたい場合は `modifyExpressions` を使用してください。
:::

## 表情のクリア

`clearExpressions()` はすべての表情オーバーライドを削除し、VRMA アニメーションに完全な制御を戻します。

```typescript
await character.clearExpressions();
```

これは、スクリプトによる表情シーケンスからアニメーション駆動の表情に戻すときに便利です。

## 口の表情

`modifyMouth(weights)` はリップシンク用の口の表情を設定します。未指定の口の表情は 0.0 にリセットされますが、口以外のオーバーライド（`happy` や `blink` など）は維持されます。

```typescript
// 「あ」の音の口の形状を設定
await character.modifyMouth({ aa: 0.8 });

// 「お」の音に変更 -- aa は 0 にリセット、他のオーバーライドは維持
await character.modifyMouth({ oh: 1.0 });

// 口を閉じる -- すべての口の表情が 0 にリセット
await character.modifyMouth({});
```

この分離により、リップシンクを感情表現とは独立して制御できます。

## 表情のクエリ

`expressions()` はすべての表情の現在の状態（ウェイトとメタデータを含む）を返します。

```typescript
const { expressions } = await character.expressions();
for (const expr of expressions) {
  if (expr.weight > 0) {
    console.log(`${expr.name}: weight=${expr.weight}, binary=${expr.isBinary}`);
  }
}
```

各 `ExpressionInfo` には以下が含まれます：
- `name` -- 表情名（例：`"happy"`、`"aa"`）
- `weight` -- 現在のウェイト値（0.0--1.0）
- `isBinary` -- 表情が 0 か 1 にスナップするかどうか（中間値なし）
- `overrideBlink` -- この表情がまばたきとどう相互作用するか（`"none"`、`"blend"`、または `"block"`）
- `overrideLookAt` -- この表情が視線追従とどう相互作用するか
- `overrideMouth` -- この表情が口の表情とどう相互作用するか

## 利用可能な表情

ほとんどのモデルで利用可能な標準 VRM 表情：

| カテゴリ | 表情 |
|---|---|
| **感情** | `happy`、`angry`、`sad`、`relaxed`、`surprised`、`neutral` |
| **口** | `aa`、`ih`、`ou`、`ee`、`oh` |
| **目** | `blink`、`blinkLeft`、`blinkRight` |
| **視線** | `lookUp`、`lookDown`、`lookLeft`、`lookRight` |

:::note
利用可能な表情は VRM モデルによって異なります。すべてのモデルがすべての表情を含んでいるわけではありません。特定のモデルがサポートする表情を確認するには `expressions()` を使用してください。
:::

## 例：感情リアクションシーケンス

```typescript
const character = await Vrm.findByName("MyAvatar");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// 驚きリアクション
await character.setExpressions({ surprised: 1.0 });
await sleep(1000);

// 嬉しい表情に移行
await character.setExpressions({ happy: 1.0 });
await sleep(2000);

// アニメーション制御に戻す
await character.clearExpressions();
```

## 型定義

```typescript
interface ExpressionInfo {
  name: string;
  weight: number;
  isBinary: boolean;
  overrideBlink: OverrideType;
  overrideLookAt: OverrideType;
  overrideMouth: OverrideType;
}

interface ExpressionsResponse {
  expressions: ExpressionInfo[];
}

type OverrideType = "none" | "blend" | "block";
```

## 次のステップ

- **[スピーチタイムライン](./speech-timeline)** -- 口の表情をオーディオと合わせてリップシンクに使用します。
- **[アニメーション](./animations)** -- 表情と VRMA アニメーションを組み合わせます。
- **[VRM 概要](./)** -- 完全な API リファレンス表。
