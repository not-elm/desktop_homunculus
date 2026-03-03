---
title: "ペルソナ"
sidebar_position: 7
---

# ペルソナ

キャラクターの性格データを取得・設定します。ペルソナにはプロフィール説明、自然言語による性格文字列、OCEAN 性格特性、MOD 拡張用の任意メタデータが含まれます。

## インポート

```typescript
import { Vrm } from "@hmcs/sdk";
```

## ペルソナの取得

`vrm.persona()` はキャラクターの現在のペルソナデータを返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const persona = await character.persona();

console.log("Profile:", persona.profile);
console.log("Personality:", persona.personality);
console.log("Openness:", persona.ocean.openness);
console.log("Metadata:", persona.metadata);
```

## ペルソナの設定

`vrm.setPersona(persona)` はキャラクターのペルソナ全体を置換します。

```typescript
await character.setPersona({
  profile: "A cheerful virtual assistant who loves to help with coding.",
  personality: "Friendly, curious, and always enthusiastic about learning new things.",
  ocean: {
    openness: 0.9,
    conscientiousness: 0.7,
    extraversion: 0.8,
    agreeableness: 0.85,
    neuroticism: 0.2,
  },
  metadata: {
    favoriteColor: "blue",
    skills: ["typescript", "rust"],
  },
});
```

### スポーン時のペルソナ設定

キャラクターをスポーンするときに初期ペルソナを設定できます：

```typescript
const character = await Vrm.spawn("my-mod:character", {
  persona: {
    profile: "A quiet observer who watches the screen.",
    ocean: { extraversion: 0.2, neuroticism: 0.1 },
    metadata: {},
  },
});
```

## OCEAN モデル

ビッグファイブ性格特性で、それぞれ 0.0 から 1.0 の数値です。すべてのフィールドはオプションです。

| 特性 | 低い（0.0） | 高い（1.0） |
|---|---|---|
| `openness` | 保守的、実用的 | 好奇心旺盛、想像力豊か |
| `conscientiousness` | 自発的、柔軟 | 組織的、規律正しい |
| `extraversion` | 内向的、控えめ | 外向的、社交的 |
| `agreeableness` | 独立的、競争的 | 協力的、信頼する |
| `neuroticism` | 安定、冷静 | 繊細、感情的 |

```typescript
const ocean: Ocean = {
  openness: 0.8,
  conscientiousness: 0.6,
  extraversion: 0.7,
  agreeableness: 0.9,
  neuroticism: 0.3,
};
```

## メタデータ

`metadata` フィールドは MOD 固有の拡張用の `Record<string, unknown>` です。キャラクターに関する任意のデータを保存するために使用します。

```typescript
await character.setPersona({
  profile: "A helpful assistant",
  ocean: {},
  metadata: {
    voiceId: "en-US-1",
    greeting: "Hello! How can I help you today?",
    customTraits: {
      humor: 0.7,
      formality: 0.3,
    },
  },
});
```

## 変更のリッスン

[イベントシステム](./events)を使用して、キャラクターのペルソナが更新されたときに反応します：

```typescript
const eventSource = character.events();

eventSource.on("persona-change", (e) => {
  console.log("Persona updated:");
  console.log("  Profile:", e.persona.profile);
  console.log("  Openness:", e.persona.ocean.openness);
});
```

## 型定義

```typescript
interface Persona {
  /** キャラクターのプロフィール／背景説明 */
  profile: string;
  /** 自然言語による性格の説明 */
  personality?: string | null;
  /** ビッグファイブ性格パラメータ */
  ocean: Ocean;
  /** MOD 用の拡張メタデータ */
  metadata: Record<string, unknown>;
}

interface Ocean {
  openness?: number;
  conscientiousness?: number;
  extraversion?: number;
  agreeableness?: number;
  neuroticism?: number;
}
```

## 次のステップ

- **[イベント](./events)** -- `persona-change` やその他のキャラクターイベントを購読します。
- **[スポーンと検索](./spawn-and-find)** -- スポーン時にペルソナを設定します。
- **[VRM 概要](./)** -- 完全な API リファレンス表。
