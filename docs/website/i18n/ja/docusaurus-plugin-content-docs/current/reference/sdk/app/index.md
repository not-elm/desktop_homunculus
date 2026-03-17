---
sidebar_position: 1
---

# app

アプリケーションライフサイクル、ヘルスチェック、プラットフォーム情報を提供します。`app` を使用して、エンジンが実行中かどうかの確認、バージョンや機能の照会、アプリケーションのシャットダウンを行えます。

## インポート

```typescript
import { app } from "@hmcs/sdk";
```

## Functions

| Function | 説明 |
|----------|-------------|
| [health](./health) | エンジンが到達可能で正常であれば `true` を返す |
| [info](./info) | 実行中のエンジンインスタンスのメタデータを返す |
| [exit](./exit) | Desktop Homunculus アプリケーションをグレースフルにシャットダウンする |

See also: [型定義](./types)
