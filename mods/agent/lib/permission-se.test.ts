import { describe, expect, it } from 'vitest';
import { DEFAULT_PERMISSION_SE, resolvePermissionSeAsset } from './permission-se';

describe('resolvePermissionSeAsset', () => {
  it('returns default SE when metadata has no permissionSe key', () => {
    const result = resolvePermissionSeAsset({});
    expect(result).toBe(DEFAULT_PERMISSION_SE);
  });

  it('returns default SE when metadata is undefined', () => {
    const result = resolvePermissionSeAsset(undefined);
    expect(result).toBe(DEFAULT_PERMISSION_SE);
  });

  it('returns null when permissionSe is null (disabled)', () => {
    const result = resolvePermissionSeAsset({ permissionSe: null });
    expect(result).toBeNull();
  });

  it('returns custom asset ID when permissionSe is a string', () => {
    const result = resolvePermissionSeAsset({ permissionSe: 'se:local:abc:xyz' });
    expect(result).toBe('se:local:abc:xyz');
  });

  it('returns default SE when permissionSe is undefined explicitly', () => {
    const result = resolvePermissionSeAsset({ permissionSe: undefined });
    expect(result).toBe(DEFAULT_PERMISSION_SE);
  });
});
