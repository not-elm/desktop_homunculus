---
title: "Component Library (@hmcs/ui)"
sidebar_position: 3
---

# Component Library (@hmcs/ui)

`@hmcs/ui` is the shared React component library for Desktop Homunculus. It provides pre-styled components with a glassmorphism design language, built on shadcn/ui + Radix UI + Tailwind CSS v4.

## Installation

```bash
pnpm add @hmcs/ui
```

Then add CSS imports to your `index.css`:

```css
@import "tailwindcss";
@import "@hmcs/ui/dist/index.css";
```

## Design System

The visual language is built around a few core principles:

- **Glassmorphism** -- Semi-transparent backgrounds (`bg-card`, `bg-primary/30`) with backdrop blur for a frosted glass effect. Components render over the transparent Bevy game window.
- **Dark mode** -- Always active in webviews. Set `class="dark"` on the `<html>` element.
- **oklch color space** -- CSS custom properties use perceptually uniform oklch colors for consistent theming.
- **Transparent body** -- Set `background: transparent` on `<body>` so the game window shows through.
- **No scrollbars** -- The `no-scrollbar` utility class hides scrollbars cross-browser.

:::note
The design system is optimized for rendering over the 3D game window. Colors, opacity, and blur values are tuned for transparent backgrounds, not for standard web pages.
:::

## Components

### Layout

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

### Forms

#### Button

Supports multiple variants and sizes:

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

Pair with form controls for accessibility:

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

### Feedback

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

### Overlays

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
    {/* content */}
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

### Custom Components

These are higher-level components built on top of the base shadcn/ui primitives, designed for common Desktop Homunculus patterns.

#### SettingsCard

A pre-composed card for settings panels. Wraps a `Card` with a title, optional description, and a content area for controls.

```tsx
import { SettingsCard, Slider } from "@hmcs/ui";

<SettingsCard title="Volume" description="Adjust the audio volume">
  <Slider defaultValue={[75]} max={100} />
</SettingsCard>
```

Props: `title` (string, required), `description` (string, optional), `children` (ReactNode).

#### NumericSlider

A labeled slider that displays its current numeric value. Requires controlled `value` and `onValueChange` props.

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

Props: `label` (string, required), `value` (number[], required), `onValueChange` (function, required), plus all standard Radix `Slider` props.

## Storybook

Interactive component explorer:

```bash
cd packages/ui
pnpm storybook
```

Opens at `http://localhost:6006`. Browse all components with interactive controls and live previews.

## Custom Styling

Patterns for mod-specific customization:

- **Tailwind utilities** -- Use standard Tailwind classes for spacing, colors, and typography. All `@hmcs/ui` components accept a `className` prop for overrides.

- **CSS custom properties** -- Define mod-specific theme variables (e.g., `--menu-accent-hue`) in your `index.css`. The design system uses oklch-based custom properties that you can override or extend.

- **cn() utility** -- The `cn()` function (from `clsx` + `tailwind-merge`) is available at `@hmcs/ui/src/lib/utils` but is not re-exported from the main package entry point. If you need it in your mod, install the dependencies directly:

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

- **Custom animations** -- Add keyframe animations in your `index.css`. The settings mod (`mods/settings/ui/src/index.css`) has examples of holographic HUD effects.

## Next Steps

- [Menus](../menus) -- Add right-click menu entries to open your UI
- [Overview](./overview) -- Review the WebView architecture and SDK API reference
