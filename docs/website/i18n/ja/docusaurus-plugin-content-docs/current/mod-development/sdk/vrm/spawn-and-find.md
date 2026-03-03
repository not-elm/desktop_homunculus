---
title: "スポーンと検索"
sidebar_position: 2
---

# スポーンと検索

新しい VRM キャラクターを作成し、既存のキャラクターを検索します。`Vrm` クラスは VRM インスタンスのスポーン、クエリ、ストリーミングのための静的メソッドを提供します。

## インポート

```typescript
import { Vrm } from "@hmcs/sdk";
```

## キャラクターのスポーン

`Vrm.spawn(asset, options?)` は MOD アセット ID から新しい VRM キャラクターを作成し、スポーンされたエンティティにバインドされた `Vrm` インスタンスを返します。

```typescript
const character = await Vrm.spawn("my-mod:character");
```

### スポーンオプション

オプションオブジェクトを渡して初期トランスフォームとペルソナを設定できます：

```typescript
import { Vrm, type Persona } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character", {
  transform: {
    translation: [0, 0.5, 0],
    scale: [1.2, 1.2, 1.2],
    rotation: [0, 0, 0, 1],
  },
  persona: {
    profile: "A cheerful virtual assistant who loves to help.",
    ocean: { openness: 0.8, extraversion: 0.7 },
    metadata: {},
  },
});
```

`transform` フィールドは部分的な `TransformArgs` を受け付けます -- オーバーライドしたいフィールドのみ指定すれば大丈夫です。未指定のフィールドはエンジンのデフォルト値が使用されます。

```typescript
// 位置のみ設定し、スケールと回転はデフォルトのまま
const character = await Vrm.spawn("my-mod:character", {
  transform: { translation: [2, 0, 0] },
});
```

## 名前で検索

`Vrm.findByName(name)` はすでに読み込まれているキャラクターの `Vrm` インスタンスを返します。その名前のキャラクターが存在しない場合はエラーをスローします。

```typescript
try {
  const character = await Vrm.findByName("MyAvatar");
  console.log("Found entity:", character.entity);
} catch (e) {
  console.log("Character not found");
}
```

### 読み込み待機

`Vrm.waitLoadByName(name)` は指定した名前のキャラクターの読み込みが完了するまでブロックします。あなたの MOD がキャラクターの MOD よりも先に開始される場合に使用してください。

```typescript
// "MyAvatar" が完全に読み込まれるまで待機
const character = await Vrm.waitLoadByName("MyAvatar");
```

:::tip
別の MOD がスポーンするキャラクターに依存する場合は、MOD サービスで `waitLoadByName` を使用してください。キャラクターがすでに存在していると想定できる場合は `findByName` を使用してください。
:::

## すべてのキャラクターを一覧表示

```typescript
// 読み込み済みのすべてのキャラクターの Vrm インスタンスを取得
const characters = await Vrm.findAll();
for (const vrm of characters) {
  const name = await vrm.name();
  console.log(`${name} (entity: ${vrm.entity})`);
}
```

### エンティティ ID のみ

`Vrm` インスタンスにラップせずにエンティティ ID のみが必要な場合：

```typescript
const entityIds = await Vrm.findAllEntities();
console.log(`Found ${entityIds.length} VRM entities`);
```

### 詳細スナップショット

`Vrm.findAllDetailed()` は読み込み済みのすべての VRM の完全なランタイム状態を返します -- トランスフォーム、表情、アニメーション、ペルソナ、リンクされた WebView を含みます。

```typescript
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  console.log(`${s.name}: state=${s.state}`);
  console.log(`  Position: (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
  console.log(`  Animations: ${s.animations.length} active`);
  console.log(`  Expressions: ${s.expressions.expressions.length} defined`);
}
```

## ストリーミング

`Vrm.stream(callback)` は現在存在するすべての VRM と、今後作成される VRM に対してコールバックを実行します。完了時に閉じることができる `EventSource` を返します。

```typescript
const es = Vrm.stream(async (vrm) => {
  const name = await vrm.name();
  console.log(`VRM appeared: ${name} (entity: ${vrm.entity})`);
});

// ストリーミングを停止
es.close();
```

これは、どの MOD がスポーンしたかに関係なく、シーンに現れるすべてのキャラクターに反応する必要がある MOD に便利です。

## デスポーン

`despawn()` でキャラクターをシーンから削除します：

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.despawn();
```

## 位置

画面座標とワールド座標の両方でキャラクターの位置をクエリします：

```typescript
const character = await Vrm.findByName("MyAvatar");
const pos = await character.position();

// 画面座標（マルチモニターのグローバルビューポート）
if (pos.globalViewport) {
  console.log(`Screen: (${pos.globalViewport[0]}, ${pos.globalViewport[1]})`);
}

// Bevy ワールド座標
console.log(`World: (${pos.world[0]}, ${pos.world[1]}, ${pos.world[2]})`);
```

## 状態

キャラクターには状態文字列があります（例：`"idle"`、`"drag"`、`"sitting"`）。他のシステムが読み書きできます：

```typescript
const state = await character.state();
console.log("Current state:", state);

await character.setState("custom-state");
```

## ボーン

オブジェクトのアタッチや視線追従ターゲットなどの高度な操作のために、名前付きボーンのエンティティ ID を取得します：

```typescript
const headEntity = await character.findBoneEntity("head");
const leftHandEntity = await character.findBoneEntity("leftHand");
```

利用可能なボーン名：`hips`、`spine`、`chest`、`neck`、`head`、`leftShoulder`、`leftArm`、`leftForeArm`、`leftHand`、`rightShoulder`、`rightArm`、`rightForeArm`、`rightHand`、`leftUpLeg`、`leftLeg`、`leftFoot`、`rightUpLeg`、`rightLeg`、`rightFoot`。

## 型定義

```typescript
interface SpawnVrmOptions {
  transform?: TransformArgs;
  persona?: Persona;
}

interface PositionResponse {
  /** グローバルな画面座標。表示されていない場合は null。 */
  globalViewport: [number, number] | null;
  /** Bevy ワールド座標。 */
  world: Vec3;
}

interface VrmSnapshot {
  entity: number;
  name: string;
  state: string;
  transform: Transform;
  globalViewport: [number, number] | null;
  expressions: ExpressionsResponse;
  animations: VrmaInfo[];
  lookAt: LookAtState | null;
  linkedWebviews: number[];
  persona: Persona;
}
```

## 次のステップ

- **[表情](./expressions)** -- 表情とブレンドシェイプを制御します。
- **[アニメーション](./animations)** -- リピートとトランジションオプション付きで VRMA アニメーションを再生します。
- **[イベント](./events)** -- ポインタ、ドラッグ、状態変更イベントを購読します。
- **[VRM 概要](./)** -- すべての VRM メソッドの完全な API リファレンス表。
