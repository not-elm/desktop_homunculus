---
title: "リソース"
sidebar_position: 7
---

# リソース

リソースは Desktop Homunculus の状態の読み取り専用スナップショットを提供します。

### `homunculus://info`

バージョンやプラットフォームを含むアプリケーション情報です。

**MIME タイプ：** `application/json`

---

### `homunculus://characters`

現在読み込まれているすべての VRM キャラクターの詳細な状態です。`get_character_snapshot` の出力と同じデータ構造です。

**MIME タイプ：** `application/json`

---

### `homunculus://mods`

インストール済み MOD の一覧と、利用可能な MOD コマンドおよび宣言されたアセットです。

**MIME タイプ：** `application/json`

`execute_command` で利用可能なコマンドを確認するために使用してください。

---

### `homunculus://assets`

すべてのインストール済み MOD にわたる利用可能なすべてのアセット（VRM モデル、VRMA アニメーション、サウンド、画像、HTML）とそのアセット ID の一覧です。

**MIME タイプ：** `application/json`

`spawn_character`、`play_animation`、`play_sound`、`control_bgm` のアセット ID を確認するために使用してください。
