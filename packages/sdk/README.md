# @hmcs/sdk

TypeScript SDK for building mods and extensions for [Desktop Homunculus](https://github.com/not-elm/desktop_homunculus).

## Install

```bash
npm install @hmcs/sdk@https://github.com/desktop-homunculus/typescript-sdk
```

## Quick Start

```typescript
import { Vrm } from "@hmcs/sdk";
// Spawn a VRM character
const vrm = await Vrm.spawn("my-mod:avatar");
```

## Function Style

For SDK source code under `src/`, use the following style:

- Use `function` / `async function` declarations for top-level public API functions and reusable top-level helpers.
- Use arrow functions for inline callbacks (for example, array methods and event listeners).

This keeps API boundaries explicit while preserving readability in callback-heavy code.

## License

MIT
