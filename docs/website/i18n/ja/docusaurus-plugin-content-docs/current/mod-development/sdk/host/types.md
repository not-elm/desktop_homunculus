---
sidebar_position: 100
---

# 型定義

### HomunculusApiError

HTTP API が非 OK ステータス（400 以上）を返した場合にスローされます。

```typescript
class HomunculusApiError extends Error {
  readonly statusCode: number;  // HTTP ステータスコード（例：404、500）
  readonly endpoint: string;    // リクエストエンドポイント URL
  readonly body: string;        // レスポンスボディのテキスト
}
```

```typescript
import { HomunculusApiError } from "@hmcs/sdk";

try {
  await host.get(host.createUrl("vrm/999"));
} catch (err) {
  if (err instanceof HomunculusApiError) {
    console.error(err.statusCode); // 404
    console.error(err.endpoint);   // リクエスト URL
    console.error(err.body);       // レスポンスボディのテキスト
  }
}
```

### HomunculusStreamError

NDJSON ストリームの行が JSON としてパースできない場合にスローされます。

```typescript
class HomunculusStreamError extends Error {
  readonly rawLine: string;  // パースできなかった生の行
}
```

```typescript
import { HomunculusStreamError } from "@hmcs/sdk";

// err.rawLine にはパースできない行が含まれます
```
