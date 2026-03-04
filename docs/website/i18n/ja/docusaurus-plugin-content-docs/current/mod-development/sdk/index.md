---
title: "SDK 概要"
sidebar_position: 1
---

# SDK 概要

`@hmcs/sdk` は、Desktop Homunculus MOD を構築するための公式 TypeScript SDK です。エンジンの HTTP API（`localhost:3100`）を型安全なメソッド、リアルタイムイベントストリーミング、キャラクター制御・オーディオ・UI などの高レベル抽象化でラップしています。

## インストール

```shell
pnpm add @hmcs/sdk
```

:::info[Node.js の要件]
Desktop Homunculus MOD は、`tsx` を使用して TypeScript ファイルを直接実行するために **Node.js 22 以降** が必要です。MOD スクリプトにビルドステップは不要です。
:::

## モジュールマップ

SDK は 18 のモジュールで構成されており、すべてメインの `@hmcs/sdk` エントリーポイントから利用できます。また、bin スクリプトユーティリティ用に別途 `@hmcs/sdk/commands` エントリーポイントがあります。

| モジュール | インポート | 説明 |
|---|---|---|
| **Vrm** | `import { Vrm } from "@hmcs/sdk"` | VRM 3D キャラクターのスポーン、検索、アニメーション、制御。SDK のコアモジュールです。 |
| **entities** | `import { entities } from "@hmcs/sdk"` | ECS エンティティの検索と操作 -- 名前による検索、トランスフォームの取得/設定、トゥイーンアニメーション。 |
| **audio** | `import { audio } from "@hmcs/sdk"` | 効果音（`audio.se`）と BGM（`audio.bgm`）の再生。フェード/音量コントロール付き。 |
| **Webview** | `import { Webview } from "@hmcs/sdk"` | 3D 空間に埋め込まれた HTML インターフェースの作成と管理。キャラクターにリンクまたはフリーフローティング。 |
| **signals** | `import { signals } from "@hmcs/sdk"` | SSE（Server-Sent Events）によるクロスプロセス pub/sub 通信。 |
| **preferences** | `import { preferences } from "@hmcs/sdk"` | JSON シリアライゼーションによる永続的なキーバリューストレージ。ユーザー設定や MOD データ用。 |
| **effects** | `import { effects } from "@hmcs/sdk"` | 画面上にビジュアルスタンプエフェクトを表示（位置、サイズ、表示時間を指定可能な画像）。 |
| **displays** | `import { displays } from "@hmcs/sdk"` | 接続されたモニターの情報を取得 -- 寸法、位置、フレーム矩形。 |
| **coordinates** | `import { coordinates } from "@hmcs/sdk"` | スクリーン空間（ビューポート）と 3D ワールド空間の座標変換。 |
| **speech** | `import { speech } from "@hmcs/sdk"` | 音素データをリップシンク用のタイムラインキーフレームに変換するユーティリティ。 |
| **app** | `import { app } from "@hmcs/sdk"` | アプリケーションライフサイクル -- ヘルスチェック、プラットフォーム情報、エンジンバージョン、読み込み済み MOD。 |
| **mods** | `import { mods } from "@hmcs/sdk"` | インストール済み MOD の一覧、bin コマンドの実行、コマンド出力のストリーミング、メニューの取得。 |
| **assets** | `import { assets } from "@hmcs/sdk"` | アセットレジストリの検索 -- タイプ（`vrm`、`vrma`、`sound`、`image`、`html`）または MOD によるアセット一覧。 |
| **settings** | `import { settings } from "@hmcs/sdk"` | アプリケーション設定の読み取りと更新（フレームレート、レンダリング設定）。 |
| **shadowPanel** | `import { shadowPanel } from "@hmcs/sdk"` | 雰囲気演出用のシャドウオーバーレイパネルの透明度を制御。 |
| **host** | `import { host } from "@hmcs/sdk"` | 直接 API 呼び出し用の低レベル HTTP クライアント。他のすべてのモジュールが内部的に使用。 |
| **Math types** | `import { type Transform, type Vec3 } from "@hmcs/sdk"` | Transform、Vec2、Vec3、Quat、Rect の型定義。 |
| **utils** | `import { sleep } from "@hmcs/sdk"` | ユーティリティヘルパー — ノンブロッキング遅延のための `sleep(ms)`。 |

### Commands サブエントリーポイント

`@hmcs/sdk/commands` は、bin コマンドスクリプトで使用するユーティリティのための **別エントリーポイント** です。Node.js 固有の API（`process.stdin`）に依存するため、メインの `@hmcs/sdk` インポートからは意図的に除外されています。

| エクスポート | 説明 |
|---|---|
| `input.parse(schema)` | stdin から JSON を読み取り、Zod スキーマでバリデーションします。 |
| `input.parseMenu()` | メニューコマンドの stdin をパースし、リンクされたキャラクターの `Vrm` インスタンスを返します。 |
| `input.read()` | stdin 全体を生の UTF-8 文字列として読み取ります。 |
| `output.succeed(data)` | JSON 結果を stdout に書き込み、終了コード 0 で終了します。 |
| `output.fail(code, message)` | 構造化エラーを stderr に書き込み、非ゼロの終了コードで終了します。 |
| `output.write(data)` | JSON 結果を stdout に書き込みます（終了しません）。 |
| `output.writeError(code, message)` | 構造化エラーを stderr に書き込みます（終了しません）。 |
| `StdinParseError` | stdin のパースまたはバリデーションが失敗した場合にスローされるエラークラス。 |

詳細な API ドキュメントは [Commands](./commands) ページを参照してください。

```typescript
// 別エントリーポイントからインポート
import { input } from "@hmcs/sdk/commands";
import { z } from "zod";

const data = await input.parse(
  z.object({
    speaker: z.number().default(0),
    text: z.string(),
  })
);
```

:::warning
MOD のメインスクリプトやブラウザ側のコードから `@hmcs/sdk/commands` をインポート **しないでください**。`process.stdin` を使用しており、Node.js の bin スクリプトでのみ利用可能です。
:::

## クイック例

```typescript
import { Vrm, preferences, repeat } from "@hmcs/sdk";

// 保存された位置を読み込み、VRM キャラクターをスポーン
const transform = await preferences.load("transform::my-mod:vrm");
const character = await Vrm.spawn("my-mod:character", { transform });

// ループするアイドルアニメーションを再生
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// キャラクターがマウスカーソルを追従するように設定
await character.lookAtCursor();

// 状態変化を監視（idle、drag、sitting など）
character.events().on("state-change", async (e) => {
  console.log("状態が変わりました:", e.state);
});
```

## 次のステップ

- **[VRM モジュール](./vrm/)** -- キャラクターのスポーン、アニメーション再生、イベント処理、スピーチの詳細ガイド。
