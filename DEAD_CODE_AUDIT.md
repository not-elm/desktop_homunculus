# Rust Engine Dead Code Audit - Final Report

## Executive Summary

After the persona-first migration (Issue #107), a thorough audit of the Rust engine crates identified several candidates for removal. The VRM API layer is largely intact but contains some methods that are now obsolete with the new persona-centric architecture.

## Critical Findings

### 1. **DEAD: VrmApi::persona() and VrmApi::set_persona()**
**Location:** `engine/crates/homunculus_api/src/vrm/persona.rs` (lines 10-30)

These methods are defined in the VrmApi but are **not called anywhere in the codebase**:
- `VrmApi::persona()` - Retrieves persona of a VRM entity
- `VrmApi::set_persona()` - Sets persona of a VRM entity

**Status:** DEAD CODE - Never referenced outside their definition file

**Why it's dead:**
- The persona-first migration moved persona management to `PersonaApi`
- Personas are now accessed via `PersonaApi::get()`, `PersonaApi::set_name()`, etc.
- The old pattern of attaching persona to VRM entities is deprecated
- No HTTP route calls these methods
- `PrefsKeys::persona()` usage in these methods is vestigial

**Action Required:** Remove both methods and their implementation in `vrm/persona.rs`

---

### 2. **DEAD: VrmApi::find_by_name()**
**Location:** `engine/crates/homunculus_api/src/vrm/find_by_name.rs` (lines 7-17)

**Status:** DEAD CODE - Only used within the VrmApi module itself (observer.rs indirectly via fetch_all)

**Why it's dead:**
- The HTTP layer and all clients use `EntitiesApi::find_by_name()` instead
- No HTTP route calls `VrmApi::find_by_name()`
- The only internal usage is within the `observer()` method (which itself is unused)
- This is duplicative with the general entity finding API

**Search Results:**
```
grep -r "\.find_by_name" engine/crates/homunculus_http_server
→ Only matches to EntitiesApi and ModsApi, never VrmApi
```

**Action Required:** Remove this method and corresponding test infrastructure

---

### 3. **DEAD: VrmApi::wait_load_by_name()**
**Location:** `engine/crates/homunculus_api/src/vrm/wait_load_by_name.rs` (lines 8-15)

**Status:** DEAD CODE - Never called from any HTTP route or other API

**Why it's dead:**
- No HTTP route exposes this functionality
- Not used in any other API methods
- Appears to be legacy from pre-persona migration
- The pattern of "finding a VRM by waiting for load" is superseded by PersonaApi workflows

**Action Required:** Remove this method entirely

---

### 4. **DEAD: VrmApi::observer()**
**Location:** `engine/crates/homunculus_api/src/vrm/observer.rs` (lines 9-40)

**Status:** DEAD CODE - Not exposed via HTTP API; only internal usage

**Why it's dead:**
- No HTTP route calls this method
- The method exists but provides no externally-accessible functionality
- It calls `fetch_all()`, which is only used by this method
- Creates an event observer channel that never receives subscribers through HTTP

**Action Required:** Remove this method and the `observe_load()` helper

---

### 5. **DEAD: VrmApi::fetch_all()**
**Location:** `engine/crates/homunculus_api/src/vrm/fetch_all.rs` (lines 7-13)

**Status:** DEAD CODE - Only called by the unused observer() method

**Why it's dead:**
- Only reference: `observer.rs:12` - `self.fetch_all().await?`
- Observer itself is not exposed via HTTP
- This breaks the dependency chain: `fetch_all()` → used only by `observer()` → used nowhere

**Related Code:**
```rust
// In observer.rs:12
let vrms = self.fetch_all().await?;
```

**Action Required:** Remove this method after removing observer()

---

### 6. **OBSOLETE: VrmApi::spawn() and VrmApi::despawn()**
**Location:** `engine/crates/homunculus_api/src/vrm/spawn.rs`, `engine/crates/homunculus_api/src/vrm/despawn.rs`

**Status:** OBSOLETE - Used internally but through deprecated patterns

**Details:**
- `spawn()` is called internally by the Bevy system during entity creation
- `despawn()` exists but persona deletion is now handled via `PersonaApi::delete()`
- These are lower-level VRM-centric methods that have been superseded by PersonaApi

**Current Usage Pattern:**
```rust
// Old pattern (in spawn.rs):
// Directly uses AssetIdComponent and PrefsKeys::persona()
// Calls PrefsKeys::persona() to load/save preferences (lines 66-78)
```

**Modern Pattern (PersonaApi):**
```rust
// New pattern - personas are created via PersonaApi::create()
// which handles VRM attachment separately via attach_vrm()
```

**Recommendation:** Keep for now (used in VRM entity lifecycle), but refactor to remove PrefsKeys::persona() usage

---

### 7. **OBSOLETE: AssetIdComponent**
**Location:** `engine/crates/homunculus_core/src/schema/asset.rs`

**Usage Analysis:**
- Still used in: `spawn.rs:82`, `persona.rs:40-41`, `snapshot.rs`, `vrma.rs`, `vrm_transform.rs`
- Purpose: Track which asset is attached to a VRM entity
- Status: Still needed for VRM rendering but accessible only through internal systems

**Note:** This component is still functional and necessary for the rendering layer. The question is whether it should be kept as-is or refactored. Current usage shows it's still load-bearing.

---

### 8. **UNUSED: VrmApi::snapshot()**
**Location:** `engine/crates/homunculus_api/src/vrm/snapshot.rs` (lines 46-53)

**Status:** Used only by MCP handler, not exposed via HTTP REST API

**Usage:**
- `engine/crates/homunculus_mcp/src/handler/tools/vrm.rs` - MCP tool uses this
- HTTP endpoint `/snapshot` calls `PersonaApi::full_snapshot()` instead (which is modern)

**Note:** This method is still used by the MCP server, so it should NOT be removed. However, the HTTP layer correctly uses the newer PersonaFullSnapshot instead.

---

### 9. **OBSOLETE: PrefsKeys::persona() usage pattern**
**Location:** Multiple files using old key naming

**Files using the old pattern:**
- `engine/crates/homunculus_api/src/vrm/spawn.rs:66,72` - PrefsKeys::persona()
- `engine/crates/homunculus_api/src/vrm/persona.rs:48` - PrefsKeys::persona()
- `engine/crates/homunculus_prefs/src/lib.rs` - Documentation example shows the pattern

**Status:** These are vestiges of pre-persona migration architecture

**The Pattern:**
```rust
let key = PrefsKeys::persona(asset_id.0.as_ref());
prefs.save_as(&key, &persona)?;
```

**Modern Pattern:** Personas are persisted via PersonaApi with their own PersonaId keys, not by asset ID

**Action Required:** Remove or refactor these usages to align with persona storage

---

### 10. **Type still in use: VrmSnapshot**
**Location:** `engine/crates/homunculus_api/src/vrm/snapshot.rs:13-33`

**Status:** Still used by MCP handler, correctly exported

**Note:** This is NOT dead code - it's properly used by the MCP system which needs to snapshot VRM state. The HTTP layer correctly uses the newer PersonaFullSnapshot instead.

---

## Summary of Dead Code to Remove

| Item | File | Lines | Priority | Reason |
|------|------|-------|----------|--------|
| VrmApi::persona() | vrm/persona.rs | 12-19 | HIGH | Never called; obsolete |
| VrmApi::set_persona() | vrm/persona.rs | 23-30 | HIGH | Never called; obsolete |
| get_persona() fn | vrm/persona.rs | 33-35 | HIGH | Only used by persona() |
| set_persona() fn | vrm/persona.rs | 37-64 | HIGH | Only used by set_persona() |
| VrmApi::find_by_name() | vrm/find_by_name.rs | 8-17 | HIGH | Never called from HTTP |
| fetch_entity() fn | vrm/find_by_name.rs | 20-23 | HIGH | Only used by find_by_name() |
| VrmApi::wait_load_by_name() | vrm/wait_load_by_name.rs | 9-15 | HIGH | Never called anywhere |
| loaded() fn | vrm/wait_load_by_name.rs | 18-24 | HIGH | Only used by wait_load_by_name() |
| VrmApi::observer() | vrm/observer.rs | 10-40 | HIGH | Not exposed via HTTP |
| observe_load() fn | vrm/observer.rs | 26-40 | HIGH | Only used by observer() |
| VrmApi::fetch_all() | vrm/fetch_all.rs | 8-12 | HIGH | Only used by unused observer() |
| all_vrms() fn | vrm/fetch_all.rs | 15-19 | HIGH | Only used by fetch_all() |

## Items to Refactor (not delete)

| Item | File | Action |
|------|------|--------|
| PrefsKeys::persona() calls | spawn.rs:66,72; persona.rs:48 | Refactor to use PersonaId-based storage |
| VrmApi::spawn() | spawn.rs | Keep but refactor PrefsKeys usage |
| VrmApi::despawn() | despawn.rs | Keep but evaluate if still needed |
| AssetIdComponent | core/schema/asset.rs | Keep; still needed for rendering layer |

## Code Dependencies

### Dependency Graph of Dead Code:
```
fetch_all()
    └─> observer()
        └─> (not used by HTTP or other APIs)

wait_load_by_name()
    └─> (not used by anyone)

find_by_name()
    └─> (not used by HTTP; only observer internally)

persona()
    └─> (not used by anyone)

set_persona()
    └─> (not used by anyone)
```

### Code that should NOT be removed:
```
snapshot()
    └─> MCP handler (vrm.rs tool handler)
    └─> Still needed

VrmSnapshot type
    └─> snapshot() method
    └─> MCP uses it
```

## Testing Considerations

After removal:
1. Run `cargo test --workspace` to verify no tests depend on removed methods
2. Rebuild the codebase to ensure no internal dependencies were missed
3. Verify MCP tools still work (snapshot() is used by MCP, not HTTP routes)
4. Check that PersonaApi fully covers the functionality gaps

## Recommendation

**Phase 1 - Remove obvious dead code:**
1. Delete `vrm/persona.rs` entirely (both methods and helpers)
2. Delete `vrm/find_by_name.rs` entirely
3. Delete `vrm/wait_load_by_name.rs` entirely
4. Delete `vrm/observer.rs` entirely
5. Delete `vrm/fetch_all.rs` entirely
6. Update `vrm.rs` to remove these module declarations and exports

**Phase 2 - Refactor and evaluate:**
1. Refactor `spawn.rs` to remove PrefsKeys::persona() usage
2. Evaluate whether `despawn()` is still needed or replaced by PersonaApi::delete()
3. Review AssetIdComponent usage to determine if it can be deprecated

**Phase 3 - Document:**
1. Update CLAUDE.md if any VRM API documentation references removed methods
2. Add migration notes for any external consumers (if any)
