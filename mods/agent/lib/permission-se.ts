/** Default SE asset ID for permission requests. */
export const DEFAULT_PERMISSION_SE = 'agent:request-permissions';

/**
 * Resolves the SE asset ID to play for a permission request.
 *
 * @param metadata - Persona metadata (may be undefined or empty)
 * @returns Asset ID string to play, or `null` if SE is disabled
 */
export function resolvePermissionSeAsset(
  metadata: Record<string, unknown> | undefined,
): string | null {
  if (!metadata || metadata.permissionSe === undefined) {
    return DEFAULT_PERMISSION_SE;
  }
  const value = metadata.permissionSe;
  if (value === null) return null;
  return String(value);
}
