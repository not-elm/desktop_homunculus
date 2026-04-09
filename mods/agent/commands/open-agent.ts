#!/usr/bin/env tsx
/// <reference types="node" />

import { z } from "zod";
import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const { linkedPersona: personaId } = await input.parse(z.object({ linkedPersona: z.string() }));

  await Webview.open({
    source: webviewSource.local("agent:session-ui"),
    size: [1.07, 0.8],
    viewportSize: [800, 500],
    transform: { translation: [1.3, 1.0, 10.0] },
    linkedPersona: personaId,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_AGENT_FAILED", (e as Error).message);
}
