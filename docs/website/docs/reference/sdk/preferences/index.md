---
sidebar_position: 1
---

# preferences

Persistent key-value storage with JSON serialization, backed by SQLite at `~/.homunculus/preferences.db`. Use preferences to save MOD settings, character state, or any data that should survive restarts.

## Import

```typescript
import { preferences } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [list](./list) | Returns all stored key names |
| [load](./load) | Retrieves the value for a key |
| [save](./save) | Stores any JSON-serializable value under the given key |
