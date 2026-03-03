---
title: "settings"
sidebar_position: 16.3
---

# settings

`settings` モジュールは、アプリケーション全体のレンダリング設定の読み取りと更新を行います。

```typescript
import { settings } from "@hmcs/sdk";

// 現在のフレームレートを読み取り
const fps = await settings.fps();

// 新しいフレームレートを設定
await settings.setFps(30);
```

## API

### `settings.fps()`

現在のレンダリングフレームレートを返します。

- **戻り値:** `Promise<number>` -- 現在の FPS 値
- **HTTP:** `GET /settings/fps`

### `settings.setFps(fps)`

レンダリングフレームレートを更新します。

- **パラメータ:**
  - `fps` (`number`) -- ターゲットフレームレート（最小 1）
- **戻り値:** `Promise<void>`
- **HTTP:** `PUT /settings/fps`（ボディ: `{ "fps": <number> }`）

## 例

```typescript
import { settings, shadowPanel } from "@hmcs/sdk";

// 省電力モード: フレームレートを下げてオーバーレイを暗くする
await settings.setFps(15);
await shadowPanel.setAlpha(0.3);
```
