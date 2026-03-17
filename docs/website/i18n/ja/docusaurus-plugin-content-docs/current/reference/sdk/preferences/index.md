---
sidebar_position: 1
---

# preferences

JSON シリアライゼーションによる永続的なキーバリューストレージです。`~/.homunculus/preferences.db` の SQLite によりバックアップされています。プリファレンスを使用して、MOD の設定、キャラクターの状態、再起動後も残す必要のあるデータを保存できます。

## インポート

```typescript
import { preferences } from "@hmcs/sdk";
```

## 関数

| 関数 | 説明 |
|----------|-------------|
| [list](./list) | 保存されているすべてのキー名を返します |
| [load](./load) | キーの値を取得します |
| [save](./save) | JSON シリアライズ可能な任意の値を指定のキーに保存します |
