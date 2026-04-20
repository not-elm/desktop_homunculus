/**
 * Coerces an unknown thrown value to a human-readable message.
 * Preserves `Error.message` when available; otherwise stringifies.
 */
export function errorMessage(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}
