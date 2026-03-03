---
title: "キャラクター"
sidebar_position: 1
---

# キャラクター

キャラクターツールは VRM のライフサイクル、アクティブ選択、ペルソナメタデータを管理します。

#### `get_character_snapshot`

すべてのデスクトップキャラクターの現在の状態を取得します。各キャラクターのエンティティ ID、名前、位置、アクティブな表情、再生中のアニメーション、ペルソナ、lookAt 状態を返します。

このツールにはパラメータはありません。最初に見つかったキャラクターが、以降のツール呼び出しのアクティブキャラクターとして自動的に設定されます。

**レスポンス例：**

```json
[
  {
    "entity": 42,
    "name": "Elmer",
    "state": "idle",
    "position": [800, 600],
    "activeExpressions": [{ "name": "happy", "weight": 1.0 }],
    "playingAnimations": ["idle-maid"],
    "persona": { "profile": "A cheerful assistant", "personality": null },
    "lookAt": { "type": "cursor" }
  }
]
```

`position` はグローバルビューポート座標での `[x, y]` で、利用できない場合は `null` です。`lookAt` はオブジェクト（例：`{ "type": "cursor" }` や `{ "type": "target", "entity": 123 }`）、または `null` です。

---

#### `spawn_character`

デスクトップに新しい VRM キャラクターをスポーンします。スポーンされたキャラクターがアクティブキャラクターになります。利用可能な VRM アセット ID を確認するには `homunculus://assets` リソースを使用してください。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `asset` | `string` | **必須** | VRM モデルアセット ID（例：`vrm:elmer`） |
| `name` | `string` | -- | キャラクターの表示名 |
| `persona_profile` | `string` | -- | キャラクターの性格／背景説明 |
| `x` | `number` | -- | 初期ビューポート X 位置（ピクセル） |
| `y` | `number` | -- | 初期ビューポート Y 位置（ピクセル） |

**例：**

```json
{
  "asset": "vrm:elmer",
  "name": "Elmer",
  "persona_profile": "A cheerful coding companion who loves Rust",
  "x": 900,
  "y": 700
}
```

---

#### `remove_character`

デスクトップから VRM キャラクターを削除します。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `name` | `string` | -- | 削除するキャラクターの名前。省略した場合、アクティブキャラクターを削除します。 |

---

#### `select_character`

名前でアクティブキャラクターを切り替えます。「アクティブキャラクター」を対象とする以降のすべてのツールは、このキャラクターを対象にします。

利用可能な名前を一覧表示するには `get_character_snapshot` を使用してください。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `name` | `string` | **必須** | 選択するキャラクター名 |

---

#### `set_persona`

アクティブキャラクターの性格プロフィールを設定します。これは AI の会話でキャラクターがどのように表現されるかに影響します。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `profile` | `string` | **必須** | キャラクターの背景説明 |
| `personality` | `string` | -- | 自然言語での性格特性 |

**例：**

```json
{
  "profile": "A serious researcher who specializes in distributed systems",
  "personality": "Precise, methodical, occasionally dry humor"
}
```
