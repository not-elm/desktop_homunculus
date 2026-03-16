---
sidebar_position: 8
---

# bgm.status

現在の BGM 再生ステータスを取得します。

## パラメーター

なし。

## 戻り値

`Promise<BgmStatus>`

## 例

```typescript
const status = await audio.bgm.status();
if (status.state === "playing") {
  console.log(`再生中: ${status.asset}、音量: ${status.volume}`);
}
```
