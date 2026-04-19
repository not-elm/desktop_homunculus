import { describe, expect, it } from 'vitest';
import { deriveSlugFromModName, validateSlug } from './mcp';

describe('deriveSlugFromModName', () => {
  it('extracts the last segment after /', () => {
    expect(deriveSlugFromModName('@hmcs/voicevox')).toBe('voicevox');
  });

  it('replaces dashes with underscores', () => {
    expect(deriveSlugFromModName('my-custom-mod')).toBe('my_custom_mod');
  });

  it('lowercases the result', () => {
    expect(deriveSlugFromModName('@org/MyMod')).toBe('mymod');
  });

  it('handles names without a slash', () => {
    expect(deriveSlugFromModName('standalone')).toBe('standalone');
  });

  it('handles scoped packages', () => {
    expect(deriveSlugFromModName('@hmcs/my-mod')).toBe('my_mod');
  });

  it('converts multiple dashes', () => {
    expect(deriveSlugFromModName('multi---dash')).toBe('multi___dash');
  });

  it('handles uppercase in non-scoped names', () => {
    expect(deriveSlugFromModName('MyStandaloneMod')).toBe('mystandalonemod');
  });
});

describe('validateSlug', () => {
  it('accepts valid slugs starting with a lowercase letter', () => {
    expect(() => validateSlug('voicevox')).not.toThrow();
  });

  it('accepts slugs with numbers', () => {
    expect(() => validateSlug('mod1')).not.toThrow();
    expect(() => validateSlug('my_mod_2')).not.toThrow();
  });

  it('accepts slugs with underscores', () => {
    expect(() => validateSlug('my_mod')).not.toThrow();
    expect(() => validateSlug('mod_1_2')).not.toThrow();
  });

  it('rejects empty string', () => {
    expect(() => validateSlug('')).toThrow('Invalid mod slug');
  });

  it('rejects slugs starting with underscore', () => {
    expect(() => validateSlug('_foo')).toThrow('Invalid mod slug');
  });

  it('rejects slugs starting with a digit', () => {
    expect(() => validateSlug('1foo')).toThrow('Invalid mod slug');
  });

  it('rejects uppercase characters', () => {
    expect(() => validateSlug('Foo')).toThrow('Invalid mod slug');
    expect(() => validateSlug('MyMod')).toThrow('Invalid mod slug');
  });

  it('rejects hyphens', () => {
    expect(() => validateSlug('foo-bar')).toThrow('Invalid mod slug');
  });

  it('rejects special characters', () => {
    expect(() => validateSlug('foo!bar')).toThrow('Invalid mod slug');
    expect(() => validateSlug('foo.bar')).toThrow('Invalid mod slug');
    expect(() => validateSlug('foo@bar')).toThrow('Invalid mod slug');
  });

  it('rejects spaces', () => {
    expect(() => validateSlug('foo bar')).toThrow('Invalid mod slug');
  });

  it('error message includes the invalid slug', () => {
    expect(() => validateSlug('Invalid')).toThrow("Invalid mod slug 'Invalid'");
  });
});
