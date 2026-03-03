---
title: "VRMA アニメーションの作成"
sidebar_position: 2
---

# VRMA アニメーションの作成

Blender と VRM Add-on for Blender を使って VRMA クリップを作成し、Desktop Homunculus の MOD で使用します。

このページはワークフローガイドであり、DCC ツールの完全なチュートリアルではありません。
正確な UI 操作については、最新の公式 Blender および VRM Add-on ドキュメントを常に参照してください。

## このページの内容

- VRMA クリップの推奨オーサリングワークフロー
- DCC オーサリングから HMCS アセット使用への移行方法
- エクスポート後の再生確認方法

## 推奨ワークフロー

VRMA オーサリングのデフォルトパスとして **Blender + VRM Add-on for Blender** を使用します。
このワークフローでは、エクスポートされたアニメーションクリップに表情トラックを含めることができます。

## 始める前に

- アニメーション対象の VRM モデルを準備してください。
- 安全に反復できるよう、ソースファイルをバージョン管理してください。
- アセットをパッケージ化する前に、ライセンスと再配布条件を確認してください。

## Blender と VRM Add-on のセットアップ

**目標**
VRM ベースのアニメーション作業用のオーサリング環境をインストール・設定します。

**公式ドキュメントを参照**

- [Blender マニュアル](https://docs.blender.org/manual/en/latest/)
- [VRM Add-on for Blender ドキュメント](https://vrm-addon-for-blender.info/en/)
- [VRM Add-on for Blender リリース](https://github.com/saturday06/VRM-Addon-for-Blender/releases)

**期待される結果**
Blender と VRM Add-on がインストールされ、VRM モデルのインポート準備が整った状態。

## アニメーションクリップと表情の作成

**目標**
DCC シーンでスケルタルモーションと表情トラックをオーサリングします。

**公式ドキュメントを参照**

- [VRM Add-on for Blender ドキュメント](https://vrm-addon-for-blender.info/en/)
- [Blender アニメーション＆リギング マニュアル](https://docs.blender.org/manual/en/latest/animation/index.html)

**期待される結果**
シーンに、出荷したいモーションと表情を含む再利用可能なアニメーションクリップが含まれている状態。

## VRMA としてエクスポート

**目標**
オーサリングしたクリップをランタイム再生用の VRMA 形式にエクスポートします。

**公式ドキュメントを参照**

- [VRM Add-on for Blender ドキュメント](https://vrm-addon-for-blender.info/en/)
- [VRM 仕様ポータル](https://vrm.dev/en/)

**期待される結果**
HMCS アセットとして登録可能な `.vrma` ファイルが準備できた状態。

## Desktop Homunculus での確認

**目標**
エクスポートされた VRMA がランタイム環境で期待通りに動作することを確認します。

**公式ドキュメントを参照**

- [SDK: VRM アニメーション](../sdk/vrm/animations.md)

**期待される結果**
ターゲットキャラクターでアニメーションが正しく再生され、期待されるポーズと表情の動作が確認できた状態。

## MOD での VRMA の使用

エクスポートした `.vrma` ファイルを MOD の `package.json` に登録し、SDK から再生します。

- [パッケージ設定](../project-setup/package-json.md)
- [SDK: VRM アニメーション](../sdk/vrm/animations.md)

## トラブルシューティングと更新

- エクスポートや再生の動作がセットアップと異なる場合は、まず最新の Blender および VRM Add-on ドキュメントを確認してください。
- 更新後に結果が異なる場合は、エクスポートオプションを再確認し、HMCS でランタイム検証を再実行してください。
