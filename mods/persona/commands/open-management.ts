#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource, WebviewLayer } from "@hmcs/sdk";
import { output } from "@hmcs/sdk/commands";

try {
    // Singleton check — don't open if already open
    const webviews = await Webview.list();
    const existing = webviews.find(
        (w) => w.source?.type === "local" && w.source?.id === "persona:management",
    );
    if (existing) {
        output.succeed();
    } else {
        await Webview.open({
            source: webviewSource.local("persona:management"),
            size: [1.4, 0.9],
            viewportSize: [1200, 700],
            offset: [1.3, 0, WebviewLayer.FOREGROUND],
        });
        await audio.se.play("se:open");
        output.succeed();
    }
} catch (e) {
    output.fail("OPEN_MANAGEMENT_FAILED", (e as Error).message);
}
