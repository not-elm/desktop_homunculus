---
name: sync-api-docs
description: >
  Sync API documentation with implementation. Use when files in
  crates/homunculus_http_server/src/** or sdk/typescript/src/** have changed,
  or when user requests documentation updates.
---

# Sync API Documentation

Synchronize API documentation with the actual HTTP server and SDK implementation.

## Phase 1: Analysis

Gather information and present findings before making changes.

### Steps

1. **Read HTTP endpoints:** Scan `crates/homunculus_http_server/src/**` to extract all endpoint definitions (routes, methods, request/response types)

2. **Read SDK interfaces:** Scan `sdk/typescript/src/**` to extract client interfaces and type definitions

3. **Read current documentation:**
   - `docs/api/open-api.yml` for OpenAPI spec
   - `docs/mod-manual/src/sdk/**` for SDK manual

4. **Compare and classify:** For each endpoint and SDK interface, determine status:
   - `new` — exists in code but not in docs
   - `changed` — exists in both but differs
   - `unchanged` — matches between code and docs
   - `removed` — exists in docs but not in code

5. **Present summary table:**

```
| Type     | Name           | Status    | Notes                   |
|----------|----------------|-----------|-------------------------|
| Endpoint | POST /api/xxx  | new       | Not in OpenAPI spec     |
| Endpoint | GET /api/yyy   | changed   | Response schema differs |
| SDK      | SomeClient     | unchanged |                         |
```

6. **Ask for confirmation:** "Proceed with documentation updates?"

Do NOT proceed to Phase 2 without explicit user confirmation.

## Phase 2: Update

After user confirms, update the documentation.

### Steps

1. **Update OpenAPI spec:** Modify `docs/api/open-api.yml` to reflect current endpoints
   - Add new endpoints with appropriate tags
   - Update changed endpoint schemas
   - For `removed` items: confirm with user before deleting

2. **Update SDK manual:** Modify `docs/mod-manual/src/sdk/**` to match SDK interfaces

3. **Regenerate:** Run `make build-openapi`

4. **Report:** Summarize what was updated

## Constraints

- Documentation must be in English
- Documentation must match the actual implementation (not aspirational)
- Documentation must be clear and concise
- Tag APIs appropriately by category (e.g., `vrm`, `webview`, `chat`, `system`)
- Never remove documentation for "removed" endpoints without explicit confirmation
