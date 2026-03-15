---
title: "セッションとストリーミング"
sidebar_position: 2
---

# セッションとストリーミング

STT セッションのライフサイクルを管理し、Server-Sent Events（SSE）経由でリアルタイムの文字起こし結果を受信します。

## セッション制御

一度にアクティブにできる STT セッションは 1 つだけです。

### 開始

`stt.session.start(options?)` で文字起こしを開始します。新しいセッションの状態を返します。

```typescript
// デフォルトで開始（言語自動検出、small モデル）
const state = await stt.session.start();

// オプションを指定して開始
const state = await stt.session.start({
  language: "ja",
  modelSize: "medium",
});
```

**暗黙的な再起動：** セッションが `listening` 状態のときに `start()` を呼び出すと、自動的に停止してから新しいセッションを開始します。この場合、コンソールに警告が出力されます。

**Loading 中の拒否：** セッションが `loading` 状態（モデルの読み込み中）のときに `start()` を呼び出すと、`session_loading` エラーがスローされます。読み込みの完了を待つか、先に `stop()` を呼び出してください。

#### `SttStartOptions`

| フィールド | 型 | デフォルト | 説明 |
|-----------|------|----------|------|
| `language` | `string` | `"auto"` | 言語コード（ISO 639-1）または `"auto"`（自動検出） |
| `modelSize` | `SttModelSize` | `"small"` | Whisper モデルサイズ：`"tiny"`、`"base"`、`"small"`、`"medium"`、`"large-v3-turbo"`、`"large-v3"` |

### 停止

`stt.session.stop()` で現在のセッションを終了します。冪等（べきとう）— セッションがアクティブでなくても安全に呼び出せます。

```typescript
await stt.session.stop();
// 常に { state: "idle" } を返します
```

### ステータス

`stt.session.status()` は状態を変更せずに現在のセッション状態を返します。

```typescript
const status = await stt.session.status();
if (status.state === "listening") {
  console.log(`${status.language} で ${status.modelSize} モデルを使用して認識中`);
}
```

### 状態遷移

```
         start()          モデル読み込み完了
  Idle ─────────► Loading ──────────► Listening
   ▲                │                    │
   │                │ エラー              │ stop() または
   │                ▼                    │ start()（再起動）
   │              Error                  │
   │                │                    │
   └────────────────┴────────────────────┘
                  stop()
```

- **Idle** — アクティブなセッションなし
- **Loading** — モデルをメモリに読み込み中
- **Listening** — 音声のキャプチャと文字起こしを実行中
- **Error** — エラーが発生。`stop()` または `start()` でクリアされるまで持続

## 結果のストリーミング

`stt.stream(callbacks)` は永続的な SSE 接続を開き、`SttStream` インスタンスを返します。サーバーは接続時に現在のセッション状態を即座に送信するため、後から接続した場合でも `onStatus` がすぐに発火します。

```typescript
const stream = stt.stream({
  onResult: (result) => {
    console.log(`[${result.language}] ${result.text}`);
  },
  onStatus: (state) => {
    console.log("状態:", state.state);
  },
  onSessionError: (err) => {
    console.error(`エラー: ${err.error} — ${err.message}`);
  },
  onStopped: () => {
    console.log("セッション終了");
  },
});

// 完了時にクローズ
stream.close();
```

すべてのコールバックはオプションです。必要なイベントのみ購読してください。コールバックは `async` にできます。コールバック内のエラーはキャッチされ、コンソールに出力されます。

### `StreamCallbacks`

| コールバック | 引数 | 説明 |
|------------|------|------|
| `onResult` | `SttResult` | 文字起こし結果の受信時に呼び出されます |
| `onStatus` | `SttState` | セッション状態の変化時に呼び出されます（接続時にも発火） |
| `onSessionError` | `SttSessionError` | セッションレベルのエラー発生時に呼び出されます |
| `onStopped` | — | セッション停止時に呼び出されます |

## エラーハンドリング

`stt.isSttError(e, code?)` で STT 固有のエラーを判定します。型が `HomunculusApiError` に絞り込まれます。

```typescript
try {
  await stt.session.start({ language: "ja" });
} catch (e) {
  if (stt.isSttError(e, "no_microphone")) {
    console.error("マイクが見つかりません");
  } else if (stt.isSttError(e, "model_not_available")) {
    // 先にモデルをダウンロード
    await stt.models.download({ modelSize: "small" });
    await stt.session.start({ language: "ja" });
  } else if (stt.isSttError(e)) {
    console.error("STT エラー:", e.message);
  } else {
    throw e;
  }
}
```

### エラーコード

| コード | HTTP ステータス | 説明 |
|--------|---------------|------|
| `session_already_active` | 409 | 互換性のために定義されていますが、`start()` が暗黙的に再起動するため通常は発生しません |
| `session_loading` | 409 | モデルの読み込み中のため開始できません |
| `model_not_available` | 412 | 指定されたモデルがダウンロードされていません |
| `model_load_failed` | 500 | モデルファイルは存在しますが読み込みに失敗しました |
| `pipeline_failed` | 500 | 音声キャプチャまたは推論パイプラインで失敗が発生しました |
| `no_microphone` | 503 | デバイスにマイクが検出されません |
| `microphone_permission_denied` | 503 | OS によりマイクへのアクセスが拒否されました |
| `download_failed` | 500 | モデルのダウンロードに失敗しました（ネットワークエラーなど） |
| `invalid_model_size` | 422 | 不明なモデルサイズです |
| `invalid_language` | 422 | 不明な言語コードです |

## 型定義

### `SttState`

タグ付きユニオン — `state` フィールドで判別します。

| バリアント | フィールド | 説明 |
|-----------|----------|------|
| `{ state: "idle" }` | — | アクティブなセッションなし |
| `{ state: "loading", language, modelSize }` | `language: string`, `modelSize: SttModelSize` | モデル読み込み中 |
| `{ state: "listening", language, modelSize }` | `language: string`, `modelSize: SttModelSize` | 文字起こし実行中 |
| `{ state: "error", error, message }` | `error: string`, `message: string` | セッションエラーが発生 |

### `SttResult`

| フィールド | 型 | 説明 |
|-----------|------|------|
| `text` | `string` | 文字起こしされたテキスト |
| `timestamp` | `number` | セッション開始からの秒数 |
| `language` | `string` | 検出または指定された言語コード |

### `SttSessionError`

| フィールド | 型 | 説明 |
|-----------|------|------|
| `error` | `string` | エラーコード |
| `message` | `string` | 人間が読めるエラーメッセージ |

## 完全な例

文字起こしをストリーミングし、各結果をログに記録する MOD サービスの例：

```typescript
import { stt, preferences } from "@hmcs/sdk";

// モデルの準備を確認
const models = await stt.models.list();
if (!models.some((m) => m.modelSize === "small")) {
  await stt.models.download({ modelSize: "small" });
}

// セッション開始
await stt.session.start({ language: "auto", modelSize: "small" });

// 結果をストリーミングしてプリファレンスに保存
const stream = stt.stream({
  onResult: async (result) => {
    console.log(`[${result.timestamp.toFixed(1)}s] ${result.text}`);
    await preferences.save("stt::last-result", result);
  },
  onSessionError: (err) => {
    console.error(`STT エラー (${err.error}): ${err.message}`);
  },
  onStopped: () => {
    console.log("STT セッション終了");
  },
});

// グレースフルシャットダウン
process.on("SIGTERM", async () => {
  await stt.session.stop();
  stream.close();
});
```

## 次のステップ

- **[モデル](./models)** -- Whisper モデルのダウンロードと管理
- **[STT 概要](./)** -- クイックスタートとアーキテクチャの概要
