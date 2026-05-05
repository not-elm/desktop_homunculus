# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

`@hmcs/ui` — the shared React component library for Desktop Homunculus. Published to `dist/`. Consumed by the mod UI apps (`mods/settings/ui/`, `mods/menu/ui/`).

## Commands

```bash
pnpm build           # Vite library build → dist/ (ES + UMD + rolled .d.ts)
pnpm dev             # Vite build --watch
pnpm check-types     # tsc --noEmit
pnpm lint            # Biome (runs from repo root)
```

After changes here, rebuild consumers: `pnpm turbo run build` from the repo root, or rebuild individual mod UIs.

## Architecture

### Component System

Built on **shadcn/ui (new-york style)** with Radix UI primitives. Add new components: `npx shadcn@latest add <component>` — `components.json` routes them to `src/components/ui/`. Icons: **lucide-react**.

Two layers of components:
- `src/components/ui/` — shadcn/ui base components (Button, Card, Dialog, etc.), customized to the Holographic HUD palette
- `src/components/sliders/`, etc. — higher-level composed components (e.g., `NumericSlider`, `SettingsCard`)

Not all `ui/` components are exported. Check `src/index.ts` — when adding or using a component, ensure its export is present.

### Design Tokens — the Holographic HUD palette

`src/index.css` is the single source of truth for the design system. It exposes:

- shadcn semantic tokens (`--primary`, `--card`, `--muted-foreground`, ...) wired into `@theme inline` so utilities like `bg-primary/N`, `text-foreground`, `border-border` resolve to the canonical color
- Holographic accents (`--holo-cyan`, `--holo-violet`, `--holo-rose`, `--holo-teal`, `--holo-indigo`, `--holo-amber`)
- Project-specific tokens: `--panel` (mod overlay surface, used as `bg-panel/92`), `--success` (saved/confirmed green)
- HUD scale tokens (`--hud-text-*`, `--hud-surface-*`, `--hud-border-*`, `--hud-space-*`, `--hud-font-size-*`, `--hud-shadow-glow-*`, `--hud-focus-*`) for finer-grained control
- Persona-aligned auxiliary tokens: `--hud-surface-toggle-off` (`bg-hud-surface-toggle-off`, `oklch(0.25 0.01 250)` dark) for toggle-off / inset surfaces, `--hud-text-subdued` (`text-hud-text-subdued`, `oklch(0.45 0.02 250)` dark) for sub-labels (one step weaker than `text-muted-foreground`)
- Shadow system (`--shadow-holo-{xs,sm,lg,intense,multi}`) — use `shadow-holo-*` utilities

**`--input` is a deep blue-black surface** (`oklch(0.12 0.01 250 / 0.8)` dark) used by all shadcn form components (`Input`, `Textarea`, `SelectTrigger`, `Checkbox`, `Switch (off)`, `Slider track`, `Progress`, `Tabs`, `Alert`, `Button outline`). This is the persona-aligned form recess color. Prefer `bg-input` over hand-rolled dark backgrounds for form-like surfaces.

`*-foreground` tokens (`--foreground`, `--card-foreground`, `--popover-foreground`, `--accent-foreground`) are all `oklch(0.92 0.01 250)` in dark mode (hue 250, blue-leaning white) for consistent body-text tone. `--secondary-foreground` is `oklch(0.95 0.01 250)`.

Backdrop blur is applied once on `body` (`backdrop-filter: blur(12px)`) — do not re-apply per element.

**Light mode**: Tokens are defined for both `:root` and `.dark`, and Storybook can render either. Mod WebView UIs in production always run with `<html class="dark">`, so light-mode values exist for parity but are not tested in real apps. Verify changes in Storybook (`pnpm storybook`) for both modes.

When adding or styling components: reach for these tokens first. Only fall back to bracket-syntax `oklch(...)` literals for genuinely one-off colors that have no token equivalent. If you find yourself writing the same arbitrary `oklch(...)` value 3+ times, that is a missing token — promote it to `:root` / `.dark` and `@theme inline` instead of duplicating it.

### Styling

- **Tailwind CSS v4** with `@tailwindcss/vite` plugin (not PostCSS)
- CSS custom properties for theming in `src/index.css` — uses **oklch** color space for light mode, mixed oklch/rgba/hex for dark mode
- Dark mode via `.dark` class on ancestor (configured as `@custom-variant dark (&:is(.dark *))`)
- The `cn()` utility (`src/lib/utils.ts`) merges Tailwind classes via `clsx` + `tailwind-merge`
- `src/animation.css` — custom drawer open/close animations using Radix collapsible width variables
- Body is `bg-transparent` with `backdrop-filter: blur(10px)` — renders inside CEF WebViews over a transparent Bevy window
- Custom utility class `no-scrollbar` hides scrollbars cross-browser

### Build Output

Vite builds a library (`src/index.ts` entry) outputting ES + UMD formats. `vite-plugin-dts` with `rollupTypes: true` produces a single `dist/index.d.ts`. A custom plugin copies `package.json`, `LICENSE`, and `*.md` into `dist/` for flat package distribution. React/ReactDOM are externalized as peer dependencies.

### Path Alias

`@/` resolves to `src/` — configured in both `tsconfig.json` and `vite.config.ts`. Use `@/components/ui/...`, `@/lib/utils`, etc.

## Conventions

- Components use `class-variance-authority` (cva) for variant styling — follow existing patterns (see `button.tsx` for reference)
- Components accept `className` prop and merge it with `cn()`
- Use `data-slot` attributes on component root elements for external styling hooks
- The `SomeRequired<T, K>` utility type in `src/lib/utils.ts` makes specific props required from an otherwise-optional interface
