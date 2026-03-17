---
sidebar_position: 3
---

# setFps

レンダリングフレームレートを更新します。即座に適用され、設定が保持されます。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `fps` | `number` | ターゲットフレームレート（最小 1） |

## 戻り値

`Promise<void>`

## 例

```typescript
await settings.setFps(30);
```

```typescript
import { settings, shadowPanel } from "@hmcs/sdk";

// 省電力モード: フレームレートを下げてオーバーレイを暗くする
await settings.setFps(15);
await shadowPanel.setAlpha(0.3);
```
