---
sidebar_position: 5
---

# createUrl

オプションのクエリパラメータ付きの完全な API URL を構築します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `path` | `string` | API エンドポイントのパス（ベース URL からの相対パス） |
| `params` | `object`（オプション） | URL に追加するクエリパラメータ |

## 戻り値

`URL`

## 使用例

```typescript
const url = host.createUrl("vrm");
// http://localhost:3100/vrm

const url = host.createUrl("entities", { name: "MyCharacter", root: 42 });
// http://localhost:3100/entities?name=MyCharacter&root=42
```
