---
title: "@hmcs/character-settings"
sidebar_position: 5
---

# @hmcs/character-settings

キャラクター設定 MOD（`@hmcs/character-settings`）は、個々のキャラクターを構成するための WebView ベースの設定パネルを提供します。

## 概要

以下の方法で設定パネルを開きます：

- **キャラクターを右クリック**して、[コンテキストメニュー](./menu)から **「Character Settings」** を選択
- パネルはキャラクターの横にフローティング WebView ウィンドウとして開きます

## 機能

設定パネルは 3 つのタブに分かれています。変更は **Save** をクリックした後に反映されます。すべての設定は `~/.homunculus/preferences.db` に保存されます。

### Basic

| 設定 | 説明 | 範囲/タイプ |
|------|------|-------------|
| Name | キャラクターの表示名（読み取り専用） | - |
| Scale | キャラクターの表示サイズ | 0.10 - 3.00 |

### Persona

| 設定 | 説明 |
|------|------|
| Profile | キャラクターの背景とプロフィールの説明（自由テキスト） |
| Personality | 自然言語で記述する性格特性（自由テキスト） |

### OCEAN

キャラクターのビッグファイブ（Big Five）パーソナリティの各次元を調整します。各特性はスライダーで設定し、レーダーチャートで視覚化されます。

| 特性 | 低い | 高い |
|------|------|------|
| 開放性（Openness） | 保守的 | 好奇心旺盛 |
| 誠実性（Conscientiousness） | 自発的 | 計画的 |
| 外向性（Extraversion） | 内向的 | 外向的 |
| 協調性（Agreeableness） | 独立的 | 協力的 |
| 神経症傾向（Neuroticism） | 安定的 | 敏感 |

## 備考

- キャラクター設定 MOD は[コンテキストメニュー](./menu)に「Character Settings」エントリを追加します。
- 設定パネルの UI は共有の `@hmcs/ui` コンポーネント（component）ライブラリを使用しています。
