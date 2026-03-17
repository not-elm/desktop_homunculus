---
title: "VRM"
sidebar_position: 1
---

# VRM モジュール

`Vrm` クラスは `@hmcs/sdk` のコアモジュールです。VRM 3D キャラクターの完全なライフサイクルを管理します -- スポーン、検索、アニメーション、表情制御、ポインタ／ドラッグイベントの処理、リップシンクスピーチなどを提供します。

```typescript
import { Vrm, repeat, VrmEventSource } from "@hmcs/sdk";
```

## staticメソッド

| メソッド                                   | 説明                                                                                                        |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| [`Vrm.spawn`](./spawn)                     | MOD アセット ID から新しい VRM をスポーンします。`Vrm` インスタンスを返します。                             |
| [`Vrm.findByName`](./findByName)           | モデル名で VRM を検索します。見つからない場合はスローします。                                               |
| [`Vrm.waitLoadByName`](./waitLoadByName)   | VRM の読み込み完了を待ってから返します。                                                                    |
| [`Vrm.findAll`](./findAll)                 | 読み込み済みのすべての VRM インスタンスを `Vrm[]` として取得します。                                        |
| [`Vrm.findAllEntities`](./findAllEntities) | 読み込み済みのすべての VRM エンティティ ID を `number[]` として取得します。                                 |
| [`Vrm.findAllDetailed`](./findAllDetailed) | すべての VRM の詳細スナップショット（状態、トランスフォーム、表情、アニメーション、ペルソナ）を取得します。 |
| [`Vrm.streamMetadata`](./streamMetadata)   | 既存および今後作成される VRM インスタンスの生の `VrmMetadata` をストリームします。                          |
| [`Vrm.stream`](./stream)                   | 既存および今後作成される VRM インスタンスを `Vrm` オブジェクトとしてストリームします。                      |

## インスタンスメソッド

| メソッド                                   | 説明                                                                                 |
| ------------------------------------------ | ------------------------------------------------------------------------------------ |
| [`despawn`](./despawn)                     | この VRM をシーンから削除します。                                                    |
| [`position`](./position)                   | 画面座標（`globalViewport`）とワールド座標の両方で位置を取得します。                 |
| [`state`](./state)                         | 現在の状態文字列を取得します（例："idle"、"drag"、"sitting"）。                      |
| [`setState`](./setState)                   | キャラクターの状態を設定します。                                                     |
| [`name`](./name)                           | VRM モデル名を取得します。                                                           |
| [`findBoneEntity`](./findBoneEntity)       | 名前付きボーン（例："head"、"leftHand"）のエンティティ ID を取得します。             |
| [`playVrma`](./playVrma)                   | リピート、トランジション、完了オプション付きで VRMA アニメーションを再生します。     |
| [`stopVrma`](./stopVrma)                   | アセット ID で指定した VRMA アニメーションを停止します。                             |
| [`listVrma`](./listVrma)                   | この VRM にアタッチされたすべての VRMA アニメーションを一覧表示します。              |
| [`vrmaState`](./vrmaState)                 | 特定のアニメーションの再生状態（再生中、速度、経過時間）を取得します。               |
| [`setVrmaSpeed`](./setVrmaSpeed)           | アニメーションの再生速度を変更します。                                               |
| [`springBones`](./springBones)             | すべてのスプリングボーンチェーン（髪、衣服の物理）を取得します。                     |
| [`springBone`](./springBone)               | エンティティ ID で特定のスプリングボーンチェーンを取得します。                       |
| [`setSpringBone`](./setSpringBone)         | スプリングボーンの物理プロパティ（剛性、抵抗、重力）を更新します。                   |
| [`expressions`](./expressions)             | すべての表情とその現在のウェイトを取得します。                                       |
| [`setExpressions`](./setExpressions)       | 表情のウェイトを設定し、以前のすべてのオーバーライドを置換します。                   |
| [`modifyExpressions`](./modifyExpressions) | 表情のウェイトを部分的に更新します（他のオーバーライドは維持）。                     |
| [`clearExpressions`](./clearExpressions)   | すべての表情オーバーライドをクリアし、VRMA アニメーションに制御を戻します。          |
| [`modifyMouth`](./modifyMouth)             | リップシンク用の口の表情を設定します（口以外のオーバーライドは維持）。               |
| [`lookAtCursor`](./lookAtCursor)           | キャラクターの目をマウスカーソルに追従させます。                                     |
| [`lookAtTarget`](./lookAtTarget)           | キャラクターの目を特定のエンティティに向けます。                                     |
| [`unlook`](./unlook)                       | 視線追従動作を無効にします。                                                         |
| [`persona`](./persona)                     | キャラクターのペルソナ（プロフィール、性格、OCEAN 特性、メタデータ）を取得します。   |
| [`setPersona`](./setPersona)               | キャラクターのペルソナデータを設定します。                                           |
| [`speakWithTimeline`](./speakWithTimeline) | WAV オーディオをフレーム同期された表情キーフレームで再生し、リップシンクを行います。 |
| [`events`](./events)                       | リアルタイムイベントストリーミング用の `VrmEventSource` を開きます。                 |

## repeat 名前空間

| 関数                                 | 説明                                    |
| ------------------------------------ | --------------------------------------- |
| [`repeat.forever`](./repeat-forever) | アニメーションを無限にループします。    |
| [`repeat.never`](./repeat-never)     | アニメーションをちょうど1回再生します。 |
| [`repeat.count`](./repeat-count)     | アニメーションを固定回数再生します。    |

## VrmEventSource

| メソッド                                         | 説明                           |
| ------------------------------------------------ | ------------------------------ |
| [`VrmEventSource.on`](./VrmEventSource-on)       | イベントリスナーを登録します。 |
| [`VrmEventSource.close`](./VrmEventSource-close) | SSE 接続を閉じます。           |
