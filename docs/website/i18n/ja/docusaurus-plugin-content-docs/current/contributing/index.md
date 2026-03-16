---
title: コントリビューティング
sidebar_position: 1
---

# Desktop Homunculus へのコントリビューション

コントリビューションに興味を持っていただきありがとうございます！Desktop Homunculus は活発に開発中であり、アセット作成から MOD 開発、バグ報告まで、あらゆる種類のコントリビューションを歓迎しています。

## 開発環境のセットアップ

前提条件と環境構築の手順については[開発環境のセットアップ](./development-setup)をご覧ください。

## コントリビューションの方法

- **バグ報告**: [GitHub Issues](https://github.com/not-elm/desktop_homunculus/issues) で Issue を作成
- **機能の提案**: [GitHub Discussions](https://github.com/not-elm/desktop_homunculus/discussions) でスレッドを開始
- **コードの提出**: リポジトリをフォークしてブランチを作成し、プルリクエストを開く
- **質問**: [GitHub Discussions](https://github.com/not-elm/desktop_homunculus/discussions) を利用

### プルリクエストのガイドライン

1. リポジトリをフォークしてフィーチャーブランチを作成
2. 明確で分かりやすいコミットメッセージで変更を行う
   [Conventional Commits](https://www.conventionalcommits.org/) スタイルを使用：`feat:`、`fix:`、`docs:`、`refactor:` など
3. CI チェックが通ることを確認
4. PR を開く前に、ローカルで `make test`（テスト）と `make fix-lint`（リント）を実行
5. 変更内容とその理由を記載した PR を開く
6. レビューのフィードバックに対応

## 求められるコントリビューション

Desktop Homunculus は現在多くの機能が不足しており、以下の分野でのコントリビューションを積極的に求めています。

### アセットの提供

`@hmcs/assets` MOD は公式アセットを提供していますが、コレクションはまだ少数です。以下のコントリビューションを歓迎しています：

- **効果音** — UI サウンド、リアクション、アンビエントエフェクト
- **BGM** — バックグラウンドミュージック
- **VRM モデル** — VRM フォーマットの 3D キャラクターモデル
- **VRMA アニメーション** — VRM キャラクター用のモーションアニメーション

:::warning[ライセンス要件]
コントリビューションするすべてのアセットは互換性のあるライセンス（例：CC0、CC-BY）が必要です。提出時にライセンス情報を含めてください。
:::

### MOD 開発

公式 MOD のアイデアがあれば、プルリクエストを歓迎します。コードを書く前に Issue や Discussions で提案することもできます。

MOD の作成方法については [MOD 開発ガイド](/mod-development/quick-start)をご覧ください。

### Agent SKILLS

Desktop Homunculus キャラクターとのエンドユーザー体験を向上させる Claude Code スキルのコントリビューションを歓迎しています。

利用可能なスキル、インストール方法、独自のスキルの作成方法については、[スキルカタログとコントリビューションガイド](https://github.com/not-elm/desktop_homunculus/tree/main/skills)をご覧ください。

### `@hmcs/ui` の改善

MOD の WebView UI で使用される共有 `@hmcs/ui` コンポーネントライブラリの改善コントリビューションを歓迎しています。

- **新しい再利用可能なコンポーネント** — MOD の設定やゲーム内ツールに広く役立つコンポーネントの追加
- **アクセシビリティの改善** — キーボードナビゲーション、フォーカス状態、ARIA セマンティクス、スクリーンリーダーサポートの改善
- **デザインとインタラクションの洗練** — 視覚的な一貫性、間隔、状態、モーションの改良によるユーザビリティの向上
- **ドキュメントと例** — コンポーネントのドキュメント、使用ガイダンス、MOD 作者向けの実践的な例の改善

現在の使用方法と API の例については [コンポーネントライブラリガイド](/mod-development/webview-ui/component-library)をご覧ください。

## Developer Certificate of Origin（DCO）

コントリビューションを行うことで、そのコントリビューションがあなたのオリジナルの作品であり、プロジェクトのライセンスの下で提出する権利を持っていることを証明します（[Developer Certificate of Origin](https://developercertificate.org/)）。

## ライセンス

コントリビューションを行うことで、あなたのコントリビューションがコントリビューション先のコンポーネントと同じライセンスの下でライセンスされることに同意したものとします（Rust コードは MIT/Apache-2.0、TypeScript コードは MIT、ドキュメントとクリエイティブアセットは CC-BY-4.0）。

## その他機能の提案や質問事項

[GitHub Discussion](https://github.com/not-elm/desktop_homunculus/discussions) でスレッドを作成してください。
