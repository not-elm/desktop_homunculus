/**
 * HMCS `GET /personas` response element.
 * Corresponds to `engine/crates/homunculus_api/src/persona.rs` `PersonaSnapshot`.
 */
export interface HmcsPersonaSnapshot {
  id: string;
  name: string;
  age?: number | null;
  gender?: string | null;
  firstPersonPronoun?: string | null;
  profile?: string | null;
  personality?: string | null;
  vrmAssetId?: string | null;
  metadata: Record<string, unknown>;
  spawned: boolean;
}

/**
 * `openclaw agents list --json` output element.
 */
export interface OpenClawAgentListEntry {
  id: string;
  workspace: string;
}
