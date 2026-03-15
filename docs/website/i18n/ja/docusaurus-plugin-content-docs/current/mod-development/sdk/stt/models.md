---
title: "モデル"
sidebar_position: 3
---

# モデル

音声認識用の Whisper モデルをダウンロード・管理します。モデルはローカルに保存され、セッション間で永続化されます。

## モデル一覧の取得

`stt.models.list()` はダウンロード済みで検証済みのモデルをすべて返します。

```typescript
const models = await stt.models.list();
for (const m of models) {
  console.log(`${m.modelSize}: ${(m.sizeBytes / 1e6).toFixed(1)} MB at ${m.path}`);
}
```

### `ModelInfo`

| フィールド | 型 | 説明 |
|-----------|------|------|
| `modelSize` | `SttModelSize` | モデルサイズ（`"tiny"`、`"base"`、`"small"`、`"medium"`、`"large-v3-turbo"`、`"large-v3"`） |
| `sizeBytes` | `number` | ファイルサイズ（バイト） |
| `path` | `string` | 相対ファイルパス |

## モデルのダウンロード

`stt.models.download(options)` は `DownloadStream` を返します。これは `AsyncIterable<DownloadEvent>` と `PromiseLike<ModelDownloadResponse>` の両方を実装したオブジェクトです。この二重インターフェースにより、1 回の呼び出しで次の 2 通りの使い方ができます。

### Await モード（進捗なし）

直接 `await` すると、ダウンロードが完了して最終結果を返します。進捗イベントは発行されません。

```typescript
const result = await stt.models.download({ modelSize: "small" });
console.log(result.status); // "downloaded" | "alreadyExists"
```

### ストリーミングモード（進捗あり）

`for await...of` で反復すると、ダウンロードの進捗イベントが順次返されます。

```typescript
for await (const event of stt.models.download({ modelSize: "medium" })) {
  if (event.type === "progress") {
    console.log(`${event.percentage.toFixed(1)}%`);
  } else if (event.type === "complete") {
    console.log(`ダウンロード先: ${event.path}`);
  } else if (event.type === "error") {
    console.error(`ダウンロード失敗: ${event.message}`);
  }
}
```

### キャンセル

`AbortSignal` を渡すことで、進行中のダウンロードをキャンセルできます。

```typescript
const controller = new AbortController();

// 30 秒後にキャンセル
setTimeout(() => controller.abort(), 30_000);

try {
  await stt.models.download({
    modelSize: "medium",
    signal: controller.signal,
  });
} catch (e) {
  if (stt.isSttError(e)) {
    console.log("ダウンロードがキャンセルまたは失敗しました");
  }
}
```

### `ModelDownloadResponse`

`stt.models.download()` を `await` した場合に返されます。

| フィールド | 型 | 説明 |
|-----------|------|------|
| `modelSize` | `SttModelSize` | ダウンロードしたモデルサイズ |
| `status` | `"downloaded" \| "alreadyExists" \| "downloading"` | ダウンロード結果 |
| `path` | `string \| undefined` | ファイルパス（`downloaded` または `alreadyExists` のとき存在） |

### `DownloadEvent`

ストリーミングダウンロード中に順次返されます。

| バリアント | フィールド | 説明 |
|-----------|----------|------|
| `{ type: "progress" }` | `downloadedBytes: number`, `totalBytes: number`, `percentage: number` | ダウンロード進捗の更新 |
| `{ type: "complete" }` | `modelSize: SttModelSize`, `path: string` | ダウンロード完了 |
| `{ type: "error" }` | `message: string` | ダウンロード失敗 |

## モデルサイズリファレンス

| サイズ | ダウンロードサイズ | ユースケース |
|--------|-------------------|-------------|
| `"tiny"` | 32.2 MB | プロトタイプ検証向け・低リソースデバイス向け |
| `"base"` | 59.7 MB | 単純なタスク・中程度の精度 |
| `"small"` | 190 MB | **推奨。** 多くの言語で十分な精度 |
| `"medium"` | 539 MB | 高精度・リソース使用量が多い |
| `"large-v3-turbo"` | 574 MB | Large v3 に近い精度・高速な推論 |
| `"large-v3"` | 1.08 GB | 最高精度・メモリと CPU の使用量が最も多い |

## 例：進捗付きダウンロード

```typescript
import { stt } from "@hmcs/sdk";

async function ensureModel(size: stt.SttModelSize) {
  const models = await stt.models.list();
  if (models.some((m) => m.modelSize === size)) {
    console.log(`モデル ${size} は利用可能です`);
    return;
  }

  console.log(`${size} モデルをダウンロード中...`);
  for await (const event of stt.models.download({ modelSize: size })) {
    if (event.type === "progress") {
      const mb = (event.downloadedBytes / 1e6).toFixed(1);
      const total = (event.totalBytes / 1e6).toFixed(1);
      console.log(`  ${mb}/${total} MB (${event.percentage.toFixed(1)}%)`);
    } else if (event.type === "complete") {
      console.log(`  完了: ${event.path}`);
    }
  }
}

await ensureModel("small");
```

## 次のステップ

- **[セッションとストリーミング](./session-and-streaming)** -- 文字起こしの開始と結果のストリーミング
- **[STT 概要](./)** -- クイックスタートとアーキテクチャの概要
