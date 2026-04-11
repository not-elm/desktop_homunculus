#!/usr/bin/env tsx
/// <reference types="node" />

import { z } from "zod";
import { audio, Webview, WebviewLayer, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const { linkedPersona: personaId } = await input.parse(
    z.object({ linkedPersona: z.string() }),
  );

  await Webview.open({
    source: webviewSource.local("agent:session-ui"),
    size: [1.28, 0.8],
    viewportSize: [800, 500],
    transform: { translation: [1.7, 0.8, WebviewLayer.UI] },
    linkedPersona: personaId,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_AGENT_FAILED", (e as Error).message);
}
