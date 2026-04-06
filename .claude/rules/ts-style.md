# TypeScript Coding Style

## UI Components

- Always use `@hmcs/ui` components (Select, Button, Label, etc.) instead of native HTML elements (`<select>`, `<button>`, etc.) in MOD WebView UIs. The `@hmcs/ui` library provides glassmorphism-styled components that render correctly in CEF WebViews. Native elements may not display properly in the transparent window context.

## Function Granularity

- Extract functions at a granularity where the calling code reads naturally as prose.
- 関数本体が自然言語のように読めるよう、意図を名前で表現したヘルパー関数に処理を切り出す。呼び出す側は「何をするか」を述べ、ヘルパーは「どうするか」を担当する。
- Aim for function bodies under 20 lines. If a function exceeds this, look for a named sub-operation to extract.
- Inline callbacks (request handlers, Promise executors, etc.) that exceed 5 lines should be extracted as named functions.
