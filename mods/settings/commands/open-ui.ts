#!/usr/bin/env -S node --experimental-strip-types

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";

try {
  await Webview.open({
    source: webviewSource.local("settings:ui"),
    size: [1, 0.9],
    viewportSize: [900, 700],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e);
}
