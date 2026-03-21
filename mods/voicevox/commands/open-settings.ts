#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const character = await input.parseMenuCharacter();
  await Webview.open({
    source: webviewSource.local("voicevox:ui"),
    size: [0.6, 0.8],
    viewportSize: [460, 520],
    offset: [1.1, 0],
    linkedCharacter: character.characterId,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_UI_FAILED", (e as Error).message);
}
