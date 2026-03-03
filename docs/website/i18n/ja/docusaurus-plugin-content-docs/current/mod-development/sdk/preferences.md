---
title: "Preferences"
sidebar_position: 11
---

# Preferences

JSON シリアライゼーションによる永続的なキーバリューストレージです。`~/.homunculus/preferences.db` の SQLite によりバックアップされています。プリファレンスを使用して、MOD の設定、キャラクターの状態、再起動後も残す必要のあるデータを保存できます。

## インポート

```typescript
import { preferences } from "@hmcs/sdk";
```

## 保存

`preferences.save(key, value)` は、JSON シリアライズ可能な任意の値を指定のキーに保存します。キーが既に存在する場合は、以前の値を上書きします。

```typescript
await preferences.save("my-mod:theme", "dark");

await preferences.save("my-mod:settings", {
  volume: 0.8,
  notifications: true,
});
```

## 読み込み

`preferences.load<V>(key)` はキーの値を取得します。キーが存在しない場合は `undefined` を返します。

```typescript
const theme = await preferences.load<string>("my-mod:theme");
if (theme !== undefined) {
  console.log(`テーマ: ${theme}`);
}

interface Settings {
  volume: number;
  notifications: boolean;
}
const settings = await preferences.load<Settings>("my-mod:settings");
```

## 一覧

`preferences.list()` は保存されているすべてのキー名を返します。

```typescript
const keys = await preferences.list();
console.log(`${keys.length} 個のプリファレンスが保存されています`);

for (const key of keys) {
  const value = await preferences.load(key);
  console.log(`${key}:`, value);
}
```

## 削除

SDK は現在、個別のプリファレンスキーの削除をサポートしていません。キーを削除するには CLI を使用してください：

```shell
hmcs prefs delete <key>
```

## 例: キャラクターの位置を保存

一般的なパターンとして、次回起動時に復元するためにキャラクターのトランスフォームを保存します。

```typescript
import { Vrm, preferences } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// 状態変化時に位置を保存
vrm.events().on("state-change", async () => {
  const transform = await vrm.transform();
  await preferences.save("my-mod:vrm-transform", transform);
});

// 起動時に復元
const saved = await preferences.load("my-mod:vrm-transform");
if (saved) {
  const character = await Vrm.spawn("my-mod:avatar", { transform: saved });
}
```

:::note キーの命名
他の MOD との衝突を避けるため、`"mod-name:key"` プレフィックスを使用してください。例: `"my-mod:theme"`、`"my-mod:settings"`。
:::

## 次のステップ

- **[SDK 概要](./)** -- 完全なモジュールマップとインストール
