import { describe, expect, test } from 'vitest';
import { setTtsModName } from './metadata';

describe('setTtsModName', () => {
  test('sets a new ttsModName string on empty metadata', () => {
    const out = setTtsModName({}, '@hmcs/voicevox');
    expect(out).toEqual({ ttsModName: '@hmcs/voicevox' });
  });

  test('preserves unrelated fields when setting', () => {
    const out = setTtsModName({ favoriteColor: 'blue', level: 5 }, '@hmcs/voicevox');
    expect(out).toEqual({
      favoriteColor: 'blue',
      level: 5,
      ttsModName: '@hmcs/voicevox',
    });
  });

  test('overwrites existing ttsModName', () => {
    const out = setTtsModName({ ttsModName: '@hmcs/old' }, '@hmcs/new');
    expect(out).toEqual({ ttsModName: '@hmcs/new' });
  });

  test('sets ttsModName to null for "None"', () => {
    const out = setTtsModName({ favoriteColor: 'blue' }, null);
    expect(out).toEqual({ favoriteColor: 'blue', ttsModName: null });
  });

  test('does not mutate the input', () => {
    const input = { favoriteColor: 'blue' };
    setTtsModName(input, '@hmcs/voicevox');
    expect(input).toEqual({ favoriteColor: 'blue' });
  });

  test('treats null existing metadata as empty', () => {
    const out = setTtsModName(null, '@hmcs/voicevox');
    expect(out).toEqual({ ttsModName: '@hmcs/voicevox' });
  });

  test('treats undefined existing metadata as empty', () => {
    const out = setTtsModName(undefined, '@hmcs/voicevox');
    expect(out).toEqual({ ttsModName: '@hmcs/voicevox' });
  });
});
