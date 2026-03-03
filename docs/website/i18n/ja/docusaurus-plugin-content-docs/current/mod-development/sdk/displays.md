---
title: "displays"
sidebar_position: 12
---

# displays

接続されたモニターの情報を照会します -- 識別子、名前、スクリーン空間のフレーム矩形。

## インポート

```typescript
import { displays } from "@hmcs/sdk";
```

## ディスプレイの一覧取得

`displays.findAll()` は、接続されたモニターごとに 1 つの `GlobalDisplay` オブジェクトの配列を返します。

```typescript
const allDisplays = await displays.findAll();
console.log(`${allDisplays.length} 台のディスプレイが見つかりました`);

for (const d of allDisplays) {
  console.log(`${d.title} (id: ${d.id})`);
  console.log(`  フレーム: [${d.frame.min}] - [${d.frame.max}]`);
}
```

**シグネチャ：**

```typescript
displays.findAll(): Promise<GlobalDisplay[]>
```

## 型

### GlobalDisplay

```typescript
interface GlobalDisplay {
  /** 一意なディスプレイ識別子。 */
  id: number;
  /** 人間が読めるディスプレイ名。 */
  title: string;
  /** スクリーン座標でのディスプレイフレーム矩形。 */
  frame: Rect;
}
```

`GlobalDisplay` と関連する型の定義については [Coordinates](./coordinates) を参照してください。

## 次のステップ

- **[Coordinates](./coordinates)** -- スクリーン空間とワールド空間の座標変換。
