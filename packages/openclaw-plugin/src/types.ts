/**
 * DH `GET /personas` のレスポンス要素。spec §6.2 Phase A で使う。
 * engine/crates/homunculus_api/src/persona.rs の PersonaSnapshot に対応。
 */
export interface DhPersonaSnapshot {
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
 * `openclaw agents list --json` の出力要素。
 */
export interface OpenClawAgentListEntry {
  id: string;
  workspace: string;
}
