import { describe, expect, test } from 'vitest';
import { buildCorrelationKey, parseSessionKey } from './session-key.js';

describe('parseSessionKey', () => {
  test('parses a valid 5-part sessionKey', () => {
    expect(parseSessionKey('agent:elmer:slack:direct:U123')).toEqual({
      agentId: 'elmer',
      channelId: 'slack',
      kind: 'direct',
      conversationId: 'U123',
    });
  });

  test('accepts discord-style numeric conversationId', () => {
    expect(parseSessionKey('agent:elmer:discord:direct:1226364616356139105')).toEqual({
      agentId: 'elmer',
      channelId: 'discord',
      kind: 'direct',
      conversationId: '1226364616356139105',
    });
  });

  test('returns null when prefix is not "agent"', () => {
    expect(parseSessionKey('session:elmer:slack:direct:U123')).toBeNull();
  });

  test('returns null when part count is wrong', () => {
    expect(parseSessionKey('agent:elmer:slack:direct')).toBeNull();
    expect(parseSessionKey('agent:elmer:slack:direct:U123:extra')).toBeNull();
  });

  test('returns null when any part is empty', () => {
    expect(parseSessionKey('agent::slack:direct:U123')).toBeNull();
    expect(parseSessionKey('agent:elmer:slack:direct:')).toBeNull();
  });
});

describe('buildCorrelationKey', () => {
  test('lowercases channelId and concatenates with conversationId', () => {
    expect(buildCorrelationKey('Slack', 'U123')).toBe('slack:U123');
    expect(buildCorrelationKey('DISCORD', 'U123')).toBe('discord:U123');
  });

  test('preserves conversationId case (user ids can be mixed case)', () => {
    expect(buildCorrelationKey('slack', 'UAbC')).toBe('slack:UAbC');
  });
});
