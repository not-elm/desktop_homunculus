#!/usr/bin/env tsx

/// <reference types="node" />

import { z } from "zod";
import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const { linkedPersona: personaId } = await input.parse(z.object({ linkedPersona: z.string() }));
  await Webview.open({
    source: webviewSource.local("persona:ui"),
    size: [1.4, 0.85],
    viewportSize: [1200, 700],
    offset: [1.3, 0],
    linkedPersona: personaId,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_UI_FAILED", (e as Error).message);
}
