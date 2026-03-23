#!/usr/bin/env tsx
/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const character = await input.parseMenuCharacter();
  await Webview.open({
    source: webviewSource.local("agent:settings-ui"),
    size: [0.7, 0.9],
    viewportSize: [400, 700],
    offset: [1.1, 0],
    linkedCharacter: character.characterId,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_UI_FAILED", (e as Error).message);
}
