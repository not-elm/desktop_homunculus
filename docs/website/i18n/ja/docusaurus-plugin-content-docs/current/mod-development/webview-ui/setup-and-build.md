---
title: "セットアップ＆ビルド"
sidebar_position: 2
---

# セットアップ＆ビルド

Desktop Homunculus MOD 用の最初の WebView UI を構築します。このガイドの最後には、スタイル付きの「Hello from WebView」カードがエンジン内でレンダリングされます。

## 前提条件

- 既存の MOD（または先に[クイックスタート](../quick-start.md)を完了してください）
- Node.js 22 以降
- pnpm
- Desktop Homunculus が動作中であること

## Step 1 -- UI プロジェクトの作成

MOD ディレクトリ内に `ui/` フォルダと `src/` サブディレクトリを作成し、パッケージとして初期化します：

```bash
mkdir -p ui/src
cd ui
pnpm init
```

## Step 2 -- 依存関係のインストール

React、共有コンポーネントライブラリ、ビルドツールチェーンをインストールします：

```bash
pnpm add react react-dom @hmcs/ui
pnpm add -D @vitejs/plugin-react-swc @tailwindcss/vite tailwindcss typescript vite vite-plugin-singlefile @types/react @types/react-dom
```

:::note[なぜ `vite-plugin-singlefile` なのか？]
Desktop Homunculus は単一ファイルアセットから WebView HTML を読み込むため、すべての CSS と JavaScript を 1 つの HTML ファイルにインライン化する必要があります。`vite-plugin-singlefile` プラグインがビルド時にこれを自動的に処理します。
:::

## Step 3 -- TypeScript の設定

`ui/tsconfig.json` を作成します：

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true
  },
  "include": ["src"]
}
```

## Step 4 -- Vite の設定

`ui/vite.config.ts` を作成します：

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tailwindcss from "@tailwindcss/vite";
import { viteSingleFile } from "vite-plugin-singlefile";

export default defineConfig({
  plugins: [react(), tailwindcss(), viteSingleFile()],
  resolve: {
    dedupe: ["react", "react-dom", "react/jsx-runtime"],
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    assetsInlineLimit: 100000,
    cssCodeSplit: false,
  },
});
```

主要な設定：

- **`viteSingleFile()`** -- エンジンが単一アセットとして読み込めるよう、すべてを 1 つの HTML ファイルにバンドルします。
- **`dedupe`** -- `@hmcs/ui` も React に依存する場合の React インスタンスの重複を防ぎます。
- **`assetsInlineLimit: 100000`** -- 100 KB までのアセットを別ファイルとして出力する代わりにデータ URL としてインライン化します。
- **`cssCodeSplit: false`** -- すべての CSS を 1 つのチャンクに保ち、シングルファイルインライン化時に失われないようにします。

## Step 5 -- エントリファイルの作成

### `ui/index.html`

```html
<!DOCTYPE html>
<html lang="en" class="dark">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>My WebView</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

`class="dark"` 属性はダークモードを有効にします。WebView はエンジンの透過オーバーレイの外観に合わせて常にダークモードを使用します。

### `ui/src/index.css`

```css
@import "tailwindcss";
@import "@hmcs/ui/dist/index.css";

body {
  background: transparent;
}

#root {
  width: 100%;
  height: 100%;
}
```

`background: transparent` により、Bevy ウィンドウが透けて見えます。`@hmcs/ui` のインポートで、グラスモーフィズム（glassmorphism）デザインシステムとコンポーネントスタイルが取り込まれます。

### `ui/src/main.tsx`

```tsx
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import "./index.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
```

## Step 6 -- アプリの構築

`ui/src/App.tsx` を作成します：

```tsx
import { Card, CardHeader, CardTitle, CardContent } from "@hmcs/ui";

export function App() {
  return (
    <div className="p-4">
      <Card>
        <CardHeader>
          <CardTitle>Hello from WebView</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">
            This UI is rendered inside Desktop Homunculus.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
```

## Step 7 -- MOD アセットとして登録

MOD のルート `package.json` を開き、ビルドスクリプトとアセット宣言を追加します：

```json
{
  "name": "my-mod",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "build": "vite build ui"
  },
  "homunculus": {
    "assets": {
      "my-mod:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "My WebView UI"
      }
    }
  },
  "dependencies": {
    "@hmcs/sdk": "...",
    "@hmcs/ui": "...",
    "react": "...",
    "react-dom": "..."
  }
}
```

`"build"` スクリプトは Vite に `ui/` ディレクトリをビルドするよう指示します。`homunculus.assets` エントリはビルドされた HTML ファイルを登録し、エンジンがアセット ID で読み込めるようにします。

## Step 8 -- ビルドとテスト

```bash
pnpm build
hmcs mod install /path/to/my-mod
```

Desktop Homunculus を再起動します。WebView は以下のようにプログラムで開けます：

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";

Webview.open({ source: webviewSource.local("my-mod:ui") });
```

## さらに進む

### WebView 内での SDK へのアクセス

`Webview.current()` を使って React アプリ内から WebView のハンドルを取得し、`linkedVrm()` で関連付けられたキャラクターにアクセスします：

```typescript
import { Webview } from "@hmcs/sdk";

const webview = Webview.current();
const vrm = await webview?.linkedVrm();
```

`Webview.current()` は CEF がすべての WebView コンテキストに注入する `window.WEBVIEW_ENTITY` 値を読み取ります。

### MOD コマンドによるオープン

WebView をオンデマンドで開く MOD コマンドを作成します。`commands/open-ui.ts` を追加します：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { z } from "zod";
import { Webview, webviewSource } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(
    z.object({ linkedVrm: z.number() })
  );
  await Webview.open({
    source: webviewSource.local("my-mod:ui"),
    size: [1, 0.9],
    viewportSize: [900, 700],
    offset: [1.1, 0],
    linkedVrm: data.linkedVrm,
  });
} catch (e) {
  console.error(e);
}
```

`package.json` に登録します：

```json
"bin": {
  "open-ui": "commands/open-ui.ts"
}
```

### 右クリックメニューエントリの追加

ユーザーが右クリックメニューから UI を開けるようにするには、`package.json` の `homunculus` フィールドに `menus` エントリを追加します：

```json
"menus": [
  {
    "id": "open-my-ui",
    "text": "Open My UI",
    "command": "open-ui"
  }
]
```

`command` の値は `"bin"` のキーに対応するため、メニューエントリをクリックすると `open-ui` MOD コマンドが実行されます。

### 開発ワークフロー

`ui/` ディレクトリ内で `pnpm dev` を使って Vite 開発サーバーを起動し、ブラウザで反復開発できます。SDK の呼び出しはエンジン外では失敗しますが、レイアウト、スタイリング、コンポーネント構成の作業はリビルドなしで行えます。

エンジン内でテストする準備ができたら：

```bash
pnpm build
hmcs mod install /path/to/my-mod
```

Desktop Homunculus を再起動して結果を確認してください。

:::tip[開発のヒント]
- 実行中の WebView で `F1`/`F2` を押すと DevTools の開閉ができます
- `Cmd+[` / `Cmd+]` で前後にナビゲーション
- `viewportSize` は HTML ピクセルサイズ（例：`[800, 600]`）を制御し、`size` は 3D ワールド空間のサイズ（例：`[0.7, 0.7]`）を制御します
:::

## 次のステップ

- **[コンポーネントライブラリ](./component-library)** -- WebView UI で利用できる `@hmcs/ui` コンポーネントについて学ぶ
