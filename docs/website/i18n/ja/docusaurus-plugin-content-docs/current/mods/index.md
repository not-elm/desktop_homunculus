---
title: "公式 MOD"
sidebar_position: 1
---

# 公式 MOD

MOD は Desktop Homunculus にキャラクター、アニメーション、効果音、UI パネル、連携機能を追加して拡張します。以下はプロジェクトの一部としてメンテナンスされている公式 MOD です。

## MOD 一覧

| MOD | 説明 | インストール |
|---|---|---|
| [@hmcs/assets](./assets) | デフォルトの VRM キャラクターモデル、VRMA アニメーション、効果音 | 推奨 |
| [@hmcs/elmer](./elmer) | アイドル、掴み、座りのアニメーション付きで Elmer を生成するデフォルトキャラクター | 推奨 |
| [@hmcs/menu](./menu) | WebView ベースの HUD オーバーレイによる右クリックコンテキストメニュー | 推奨 |
| [@hmcs/character-settings](./character-settings) | キャラクターごとの設定パネル（名前、スケール、ペルソナ、OCEAN 特性） | 推奨 |
| [@hmcs/settings](./settings) | システムトレイからのアプリケーション設定パネル（フレームレート、影の不透明度） | 推奨 |
| [@hmcs/app-exit](./app-exit) | システムトレイの終了メニュー | 推奨 |
| [@hmcs/voicevox](./voicevox) | VoiceVox エンジンを使用した音声合成連携 | オプション |

## MOD の管理

### インストール済み MOD の一覧表示

```shell
hmcs mod list
```

出力例：

```text
 NAME           VERSION  DESCRIPTION
 @hmcs/elmer    1.0.0    Default character model
 @hmcs/menu     1.0.0    Context menu
```

### MOD のインストール

```shell
hmcs mod install <package>...
```

例えば、VoiceVox MOD をインストールするには：

```shell
hmcs mod install @hmcs/voicevox
```

### MOD のアンインストール

```shell
hmcs mod uninstall <package>...
```

CLI の詳細なリファレンスは [`hmcs mod`](/reference/cli/mod) をご覧ください。
