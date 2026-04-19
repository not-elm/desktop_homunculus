# TypeScript Coding Style

## UI Components

- Always use `@hmcs/ui` components (Select, Button, Label, etc.) instead of native HTML elements (`<select>`, `<button>`, etc.) in MOD WebView UIs. The `@hmcs/ui` library provides glassmorphism-styled components that render correctly in CEF WebViews. Native elements may not display properly in the transparent window context.

## Comments

### Region-divider comments are strictly forbidden

Never write comments whose sole purpose is to visually separate sections of a file, module, or function. This is a hard rule with no exceptions.

Forbidden forms include (but are not limited to):

```ts
// ---------------------
// --- Section Name ---
// ===== Handlers =====
// #################### Helpers ####################
// ----------------------------------------
// region: public api
// endregion
// #region Hooks
// #endregion
/* ============ Types ============ */
```

Any line or block comment consisting primarily of repeated punctuation (`-`, `=`, `*`, `#`, `/`, `~`), or explicit `region` / `endregion` / `#region` / `#endregion` / `MARK:` markers, is banned — with or without a label between the punctuation runs.

If a file feels like it needs section dividers, that is a signal to restructure, not to annotate. Use the file's natural structure instead:

- Split the file into separate modules.
- Extract cohesive logic into named functions so the function name does the labeling.
- Rely on JSDoc (`/** */`) on exported APIs and blank lines between declarations for visual grouping.

## Function Granularity

- Extract functions at a granularity where the calling code reads naturally as prose. The caller states "what" to do; the helper handles "how".
- Aim for function bodies under 20 lines. If a function exceeds this, look for a named sub-operation to extract.
- Inline callbacks (request handlers, Promise executors, etc.) that exceed 5 lines should be extracted as named functions.

## Item Ordering

Arrange items top-down within a file. High-level components go at the top; lower-level details go toward the bottom.

### File-level ordering

1. Imports (`import`)
2. Constants, type definitions, and module-level state
3. Function definitions (callers above, callees below)
4. Top-level execution code (entry points, side-effect registrations)

### Principles

- **Callers above, callees below.**
- Helper functions must be placed below the functions that call them.
- Use function declarations for module-level helpers (they are hoisted, so they can be called before their textual definition). Use arrow functions only when lexical `this` or callback semantics are needed.

## Custom Hooks

- When a React component mixes data fetching, side effects, and rendering, extract state and logic into a `useXxx` custom hook. The component should focus on rendering and wiring callbacks.
- Aim for hooks that return a single coherent interface (state + actions). Avoid splitting into multiple hooks when the state is tightly coupled.
- Generic utility hooks (e.g., `useClickOutside`, `useDebounce`) should stay local until a second consumer appears, then promote to `@hmcs/ui`.
