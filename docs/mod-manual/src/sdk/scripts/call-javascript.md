# JavaScript Execution

Calls a Javascript(or Typescript) file located in assets/mods.

The script will be called on the built-in Deno runtime.
When the Javascript file being executed is changed, a hot reload will occur, and it will be re-executed.

## Parameters

- source (string): The path to the script file relative to `assets/mods`.

## Returns

`Promise<void>` - A promise that resolves when the script has been executed.

## Examples

```typescript
import {scripts} from '@homunculus/sdk';

await scripts.callJavascript("my-mod/my-script.js");
```