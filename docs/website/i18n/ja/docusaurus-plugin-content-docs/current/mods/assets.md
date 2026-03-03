---
title: "@hmcs/assets"
sidebar_position: 2
---

# @hmcs/assets

Assets MOD（`@hmcs/assets`）は、他の MOD が依存するデフォルトのリソース（VRM キャラクターモデル、VRMA アニメーション、効果音）を提供します。

## 概要

| アセット ID | タイプ | 説明 |
|---|---|---|
| `vrm:elmer` | VRM | デフォルトのキャラクターモデル（Elmer） |
| `vrma:idle-maid` | VRMA | 手を前で組んで立つアイドルアニメーション |
| `vrma:grabbed` | VRMA | ユーザーにドラッグされている間のリアクションポーズ |
| `vrma:idle-sitting` | VRMA | 座ったアイドルループ |
| `se:open` | Sound | HUD オープンの効果音 |
| `se:close` | Sound | HUD クローズの効果音 |

## 機能

これらのアセットは、他の MOD や SDK の呼び出しでアセット ID を使って参照されます。例えば、Elmer MOD は以下のようにデフォルトキャラクターを生成します：

```ts
const elmer = await Vrm.spawn("vrm:elmer");
```

MOD 開発者は同じ ID を使ってこれらのアセットを自分の MOD から参照できます。詳細は [SDK ドキュメント](/mod-development/sdk)をご覧ください。

## 備考

- この MOD にはサービス（service）がありません。静的なアセットファイルを提供するだけです。
- `@hmcs/elmer` MOD はこの MOD に依存しており、デフォルトキャラクターの生成とアニメーションに `vrm:elmer`、`vrma:idle-maid`、`vrma:grabbed`、`vrma:idle-sitting` を使用しています。
- この MOD を削除すると、Elmer MOD およびこれらのアセット ID を参照する他の MOD が動作しなくなります。
