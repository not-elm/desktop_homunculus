#!/usr/bin/env tsx

/// <reference types="node" />

import { z } from "zod";
import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const { linkedPersona: personaId } = await input.parse(z.object({ linkedPersona: z.string() }));
  await Webview.open({
    source: webviewSource.local("voicevox:ui"),
    size: [0.6, 0.8],
    viewportSize: [460, 520],
    transform: { translation: [1.1, 1.5, 10.0] },
    linkedPersona: personaId,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_UI_FAILED", (e as Error).message);
}
