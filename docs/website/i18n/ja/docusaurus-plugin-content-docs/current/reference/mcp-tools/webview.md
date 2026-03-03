---
title: "WebView"
sidebar_position: 5
---

# WebView

WebView ツールはキャラクター付近にアタッチされた CEF パネルの開閉と更新を行います。

WebView はキャラクター付近にアタッチされた CEF ベースのブラウザパネルです。`open_webview` はエンティティ ID を返し、`close_webview` と `navigate_webview` で使用します。

#### `open_webview`

アクティブキャラクター付近に HTML コンテンツまたは URL を表示する WebView パネルを開きます。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `html` | `string` | -- | インライン HTML コンテンツ。`url` と排他的。 |
| `url` | `string` | -- | URL または MOD アセットパス。`html` と排他的。 |
| `size_x` | `number` | `0.7` | パネルの幅（ワールド単位） |
| `size_y` | `number` | `0.5` | パネルの高さ（ワールド単位） |
| `viewport_width` | `number` | `800` | ブラウザビューポートの幅（ピクセル） |
| `viewport_height` | `number` | `600` | ブラウザビューポートの高さ（ピクセル） |
| `offset_x` | `number` | `0` | キャラクター中心からの水平オフセット |
| `offset_y` | `number` | `0.5` | キャラクター中心からの垂直オフセット（正の値 = 上） |

`html` または `url` のどちらかが必須です。

**例 -- キャラクターの上にスタイル付きカードを表示：**

```json
{
  "html": "<html><body style='background:#1e1e2e;color:#cdd6f4;font-family:sans-serif;padding:16px'><h2>Build succeeded</h2></body></html>",
  "size_x": 0.8,
  "size_y": 0.3
}
```

---

#### `close_webview`

1つまたはすべての WebView パネルを閉じます。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `entity` | `number` | -- | 閉じる WebView のエンティティ ID。省略した場合、最後に開いた WebView を閉じます。 |
| `all` | `boolean` | `false` | すべての開いている WebView を閉じる |

---

#### `navigate_webview`

既存の WebView の HTML コンテンツを閉じて開き直すことなく更新します。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `entity` | `number` | -- | WebView エンティティ ID。省略した場合、最後に開いた WebView を対象にします。 |
| `html` | `string` | **必須** | 表示する新しいインライン HTML コンテンツ |
