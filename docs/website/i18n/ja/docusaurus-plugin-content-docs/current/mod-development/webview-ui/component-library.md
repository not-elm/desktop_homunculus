---
title: "コンポーネントライブラリ (@hmcs/ui)"
sidebar_position: 3
---

# コンポーネントライブラリ (@hmcs/ui)

`@hmcs/ui` は Desktop Homunculus の共有 React コンポーネントライブラリです。グラスモーフィズム（glassmorphism）デザイン言語でプリスタイルされたコンポーネントを提供し、shadcn/ui + Radix UI + Tailwind CSS v4 をベースに構築されています。

## インストール

```bash
pnpm add @hmcs/ui
```

次に `index.css` に CSS インポートを追加します：

```css
@import "tailwindcss";
@import "@hmcs/ui/dist/index.css";
```

## デザインシステム

ビジュアル言語はいくつかの中核的な原則に基づいて構築されています：

- **グラスモーフィズム** -- 半透明な背景（`bg-card`、`bg-primary/30`）とバックドロップブラーによるすりガラス効果。コンポーネントは透過 Bevy ゲームウィンドウの上にレンダリングされます。
- **ダークモード** -- WebView では常にアクティブです。`<html>` 要素に `class="dark"` を設定してください。
- **oklch カラースペース** -- CSS カスタムプロパティは知覚的に均一な oklch カラーを使用し、一貫したテーマを実現します。
- **透過ボディ** -- `<body>` に `background: transparent` を設定し、ゲームウィンドウが透けて見えるようにします。
- **スクロールバーなし** -- `no-scrollbar` ユーティリティクラスでクロスブラウザのスクロールバーを非表示にします。

:::note
デザインシステムは 3D ゲームウィンドウの上にレンダリングするために最適化されています。色、不透明度、ブラー値は透過背景向けに調整されており、標準的な Web ページ向けではありません。
:::

## コンポーネント

### レイアウト

#### Card

```tsx
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "@hmcs/ui";

<Card>
  <CardHeader>
    <CardTitle>Title</CardTitle>
    <CardDescription>Description text</CardDescription>
  </CardHeader>
  <CardContent>Content goes here</CardContent>
  <CardFooter>Footer actions</CardFooter>
</Card>
```

#### Separator

```tsx
import { Separator } from "@hmcs/ui";

<Separator />
```

#### Accordion

```tsx
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@hmcs/ui";

<Accordion type="single" collapsible>
  <AccordionItem value="item-1">
    <AccordionTrigger>Section Title</AccordionTrigger>
    <AccordionContent>Section content</AccordionContent>
  </AccordionItem>
</Accordion>
```

#### Tabs

```tsx
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@hmcs/ui";

<Tabs defaultValue="tab1">
  <TabsList>
    <TabsTrigger value="tab1">Tab 1</TabsTrigger>
    <TabsTrigger value="tab2">Tab 2</TabsTrigger>
  </TabsList>
  <TabsContent value="tab1">First tab content</TabsContent>
  <TabsContent value="tab2">Second tab content</TabsContent>
</Tabs>
```

### フォーム

#### Button

複数のバリアントとサイズをサポートします：

```tsx
import { Button } from "@hmcs/ui";

<Button>Default</Button>
<Button variant="destructive">Delete</Button>
<Button variant="outline">Cancel</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="ghost">Ghost</Button>
<Button variant="link">Link</Button>
<Button size="sm">Small</Button>
<Button size="lg">Large</Button>
<Button size="icon"><IconComponent /></Button>
```

#### Input

```tsx
import { Input } from "@hmcs/ui";

<Input type="text" placeholder="Enter text..." />
```

#### Textarea

```tsx
import { Textarea } from "@hmcs/ui";

<Textarea placeholder="Write something..." />
```

#### Label

アクセシビリティのためにフォームコントロールとペアで使用します：

```tsx
import { Label, Input } from "@hmcs/ui";

<Label htmlFor="name">Name</Label>
<Input id="name" placeholder="Character name" />
```

#### Select

```tsx
import { Select, SelectTrigger, SelectContent, SelectItem, SelectValue } from "@hmcs/ui";

<Select>
  <SelectTrigger>
    <SelectValue placeholder="Choose..." />
  </SelectTrigger>
  <SelectContent>
    <SelectItem value="a">Option A</SelectItem>
    <SelectItem value="b">Option B</SelectItem>
  </SelectContent>
</Select>
```

#### Checkbox

```tsx
import { Checkbox, Label } from "@hmcs/ui";

<Checkbox id="agree" />
<Label htmlFor="agree">I agree</Label>
```

#### Switch

```tsx
import { Switch, Label } from "@hmcs/ui";

<Switch id="enabled" />
<Label htmlFor="enabled">Enable feature</Label>
```

#### Slider

```tsx
import { Slider } from "@hmcs/ui";

<Slider defaultValue={[50]} max={100} step={1} />
```

### フィードバック

#### Badge

```tsx
import { Badge } from "@hmcs/ui";

<Badge>Default</Badge>
<Badge variant="secondary">Secondary</Badge>
<Badge variant="destructive">Error</Badge>
<Badge variant="outline">Outline</Badge>
```

#### Tooltip

```tsx
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@hmcs/ui";

<TooltipProvider>
  <Tooltip>
    <TooltipTrigger>Hover me</TooltipTrigger>
    <TooltipContent>Tooltip text</TooltipContent>
  </Tooltip>
</TooltipProvider>
```

#### HoverCard

```tsx
import { HoverCard, HoverCardTrigger, HoverCardContent } from "@hmcs/ui";

<HoverCard>
  <HoverCardTrigger>Hover target</HoverCardTrigger>
  <HoverCardContent>Detailed info here</HoverCardContent>
</HoverCard>
```

### オーバーレイ

#### Dialog

```tsx
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@hmcs/ui";
import { Button } from "@hmcs/ui";

<Dialog>
  <DialogTrigger asChild>
    <Button>Open Dialog</Button>
  </DialogTrigger>
  <DialogContent>
    <DialogHeader>
      <DialogTitle>Confirm Action</DialogTitle>
      <DialogDescription>Are you sure?</DialogDescription>
    </DialogHeader>
    <DialogFooter>
      <Button variant="outline">Cancel</Button>
      <Button>Confirm</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
```

#### Drawer

```tsx
import { Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle } from "@hmcs/ui";

<Drawer>
  <DrawerTrigger>Open Drawer</DrawerTrigger>
  <DrawerContent>
    <DrawerHeader>
      <DrawerTitle>Drawer Title</DrawerTitle>
    </DrawerHeader>
    {/* コンテンツ */}
  </DrawerContent>
</Drawer>
```

#### DropdownMenu

```tsx
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem } from "@hmcs/ui";
import { Button } from "@hmcs/ui";

<DropdownMenu>
  <DropdownMenuTrigger asChild>
    <Button variant="outline">Menu</Button>
  </DropdownMenuTrigger>
  <DropdownMenuContent>
    <DropdownMenuItem>Action 1</DropdownMenuItem>
    <DropdownMenuItem>Action 2</DropdownMenuItem>
  </DropdownMenuContent>
</DropdownMenu>
```

#### ContextMenu

```tsx
import { ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem } from "@hmcs/ui";

<ContextMenu>
  <ContextMenuTrigger>Right-click here</ContextMenuTrigger>
  <ContextMenuContent>
    <ContextMenuItem>Option 1</ContextMenuItem>
    <ContextMenuItem>Option 2</ContextMenuItem>
  </ContextMenuContent>
</ContextMenu>
```

### カスタムコンポーネント

これらは基本的な shadcn/ui プリミティブの上に構築された高レベルコンポーネントで、Desktop Homunculus の一般的なパターン向けに設計されています。

#### SettingsCard

設定パネル用のプリコンポーズドカード。タイトル、オプションの説明、コントロール用のコンテンツエリアを持つ `Card` をラップします。

```tsx
import { SettingsCard, Slider } from "@hmcs/ui";

<SettingsCard title="Volume" description="Adjust the audio volume">
  <Slider defaultValue={[75]} max={100} />
</SettingsCard>
```

Props: `title`（string、必須）、`description`（string、オプション）、`children`（ReactNode）。

#### NumericSlider

現在の数値を表示するラベル付きスライダー。制御された `value` と `onValueChange` props が必要です。

```tsx
import { NumericSlider } from "@hmcs/ui";
import { useState } from "react";

const [value, setValue] = useState([1.0]);

<NumericSlider
  label="Scale"
  min={0.1}
  max={3.0}
  step={0.1}
  value={value}
  onValueChange={setValue}
/>
```

Props: `label`（string、必須）、`value`（number[]、必須）、`onValueChange`（function、必須）、および標準の Radix `Slider` props すべて。

## Storybook

インタラクティブなコンポーネントエクスプローラー：

```bash
cd packages/ui
pnpm storybook
```

`http://localhost:6006` で開きます。インタラクティブなコントロールとライブプレビューですべてのコンポーネントを閲覧できます。

## カスタムスタイリング

MOD 固有のカスタマイズパターン：

- **Tailwind ユーティリティ** -- スペーシング、カラー、タイポグラフィには標準の Tailwind クラスを使用します。すべての `@hmcs/ui` コンポーネントはオーバーライド用の `className` prop を受け付けます。

- **CSS カスタムプロパティ** -- MOD 固有のテーマ変数（例：`--menu-accent-hue`）を `index.css` で定義します。デザインシステムは oklch ベースのカスタムプロパティを使用しており、オーバーライドや拡張が可能です。

- **cn() ユーティリティ** -- `cn()` 関数（`clsx` + `tailwind-merge`）は `@hmcs/ui/src/lib/utils` で利用できますが、メインパッケージエントリポイントからは再エクスポートされていません。MOD で必要な場合は、依存関係を直接インストールしてください：

  ```bash
  pnpm add clsx tailwind-merge
  ```

  ```tsx
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }
  ```

- **カスタムアニメーション** -- `index.css` にキーフレームアニメーションを追加します。設定 MOD（`mods/settings/ui/src/index.css`）にはホログラフィック HUD エフェクトの例があります。

## 次のステップ

- [コンテキストメニュー](../menus) -- UI を開くための右クリックメニューエントリの追加
- [概要](./overview) -- WebView アーキテクチャと SDK API リファレンスの確認
