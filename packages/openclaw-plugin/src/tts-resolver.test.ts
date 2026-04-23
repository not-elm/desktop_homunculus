import { Persona } from '@hmcs/sdk';
import { afterEach, describe, expect, test, vi } from 'vitest';
import { resolveTtsModName } from './tts-resolver.js';

afterEach(() => {
  vi.restoreAllMocks();
});

function mockMetadata(meta: Record<string, unknown>): void {
  vi.spyOn(Persona, 'load').mockResolvedValue({
    metadata: vi.fn().mockResolvedValue(meta),
  } as unknown as Persona);
}

describe('resolveTtsModName', () => {
  test('returns the modName string when metadata.ttsModName is a non-empty string', async () => {
    mockMetadata({ ttsModName: '@hmcs/voicevox' });
    await expect(resolveTtsModName('alice')).resolves.toBe('@hmcs/voicevox');
  });

  test('returns null when metadata.ttsModName is an empty string', async () => {
    mockMetadata({ ttsModName: '' });
    await expect(resolveTtsModName('alice')).resolves.toBeNull();
  });

  test('returns null when metadata.ttsModName is null', async () => {
    mockMetadata({ ttsModName: null });
    await expect(resolveTtsModName('alice')).resolves.toBeNull();
  });

  test('returns null when metadata.ttsModName is absent', async () => {
    mockMetadata({});
    await expect(resolveTtsModName('alice')).resolves.toBeNull();
  });

  test('returns null when Persona.load throws', async () => {
    vi.spyOn(Persona, 'load').mockRejectedValue(new Error('404 not found'));
    await expect(resolveTtsModName('ghost')).resolves.toBeNull();
  });

  test('returns null when metadata() throws', async () => {
    vi.spyOn(Persona, 'load').mockResolvedValue({
      metadata: vi.fn().mockRejectedValue(new Error('network')),
    } as unknown as Persona);
    await expect(resolveTtsModName('alice')).resolves.toBeNull();
  });
});
