import { describe, expect, it } from 'vitest';
import { webviewSource } from './webviews';

describe('webviewSource', () => {
  it('local() returns WebviewSourceLocal', () => {
    expect(webviewSource.local('menu:ui')).toEqual({
      type: 'local',
      id: 'menu:ui',
    });
  });

  it('url() returns WebviewSourceUrl', () => {
    expect(webviewSource.url('https://example.com')).toEqual({
      type: 'url',
      url: 'https://example.com',
    });
  });

  it('html() returns WebviewSourceHtml', () => {
    expect(webviewSource.html('<h1>Hi</h1>')).toEqual({
      type: 'html',
      content: '<h1>Hi</h1>',
    });
  });
});
