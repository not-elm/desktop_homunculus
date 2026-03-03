# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

`@hmcs/ui` — the shared React component library for Desktop Homunculus. Published to `dist/`. Consumed by the mod UI apps (`mods/settings/ui/`, `mods/menu/ui/`).

## Commands

```bash
pnpm build           # Vite library build → dist/ (ES + UMD + rolled .d.ts)
pnpm dev             # Vite build --watch
pnpm check-types     # tsc --noEmit
pnpm lint            # ESLint
```

After changes here, rebuild consumers: `pnpm turbo run build` from the repo root, or rebuild individual mod UIs.

## Architecture

### Component System

Built on **shadcn/ui (new-york style)** with Radix UI primitives. Add new components: `npx shadcn@latest add <component>` — `components.json` routes them to `src/components/ui/`. Icons: **lucide-react**.

Two layers of components:
- `src/components/ui/` — shadcn/ui base components (Button, Card, Dialog, etc.), customized with glassmorphism styling
- `src/components/sliders/`, etc. — higher-level composed components (e.g., `NumericSlider`, `SettingsCard`)

Not all `ui/` components are exported. Check `src/index.ts` — when adding or using a component, ensure its export is present.

### Glassmorphism Design Language

All components are customized with a consistent glassmorphism aesthetic for rendering over the transparent Bevy game window. When modifying or creating components, maintain this pattern:
- Semi-transparent backgrounds with opacity (`bg-primary/30`, `bg-card` where `--card` is `oklch(... / 0.7)`)
- `backdrop-blur-sm` or `backdrop-filter: blur(...)` for frosted glass effect
- Subtle borders with opacity (`border-white/20`, `border-primary/50`)
- White text (`text-white`) as the default foreground in dark contexts

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
