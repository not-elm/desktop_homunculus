---
title: "Elmer"
sidebar_position: 3
---

# Elmer

Elmer MOD（`@hmcs/elmer`）はデフォルトのキャラクター MOD です。デスクトップ上に Elmer VRM キャラクターを生成し、アニメーションと動作を管理します。

## 概要

Desktop Homunculus が起動すると、Elmer MOD は [Assets](./assets) MOD の `vrm:elmer` モデルを使用して自動的に Elmer キャラクターを生成し、アイドルアニメーションをループ再生します。

## 機能

1. **アイドルアニメーション**（`vrma:idle-maid`）をループ再生
2. **カーソル追従** — Elmer の目がマウスの位置を追跡します
3. **インタラクションへの応答：**
   - **ドラッグ** — 掴まれたポーズ（`vrma:grabbed`）に切り替わり、カーソル追従を停止
   - **ウィンドウの端に座る** — 座りアニメーション（`vrma:idle-sitting`）に切り替わる
   - **離す** — アイドルに戻り、カーソル追従を再開

## 備考

- Elmer MOD は VRM モデルとアニメーションのために [Assets](./assets) MOD を必要とします。
- キャラクターの位置はプリファレンスを通じて自動的に保存され、次回起動時に復元されます。
