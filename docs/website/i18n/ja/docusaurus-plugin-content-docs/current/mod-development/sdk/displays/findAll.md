---
sidebar_position: 2
---

# findAll

接続されたモニターごとに 1 つの [`GlobalDisplay`](../coordinates/types#globaldisplay) オブジェクトの配列を返します。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| _(なし)_ | — | — |

## Returns

`Promise<`[`GlobalDisplay`](../coordinates/types#globaldisplay)`[]>`

## Example

```typescript
const allDisplays = await displays.findAll();
console.log(`${allDisplays.length} 台のディスプレイが見つかりました`);

for (const d of allDisplays) {
  console.log(`${d.title} (id: ${d.id})`);
  console.log(`  フレーム: [${d.frame.min}] - [${d.frame.max}]`);
}
```

`GlobalDisplay` 型の定義は [Coordinates 型定義](../coordinates/types#globaldisplay) を参照してください。
