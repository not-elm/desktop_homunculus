---
sidebar_position: 2
---

# fps

現在のレンダリングフレームレートを返します。

## パラメータ

なし。

## 戻り値

`Promise<number>`

## 例

```typescript
const fps = await settings.fps();
console.log(`現在の FPS: ${fps}`);
```
