---
title: "@hmcs/app-exit"
sidebar_position: 7
---

# @hmcs/app-exit

The App Exit MOD (`@hmcs/app-exit`) adds an **Exit** entry to the system tray menu, allowing you to cleanly shut down Desktop Homunculus.

## Overview

To exit the application:

1. Click the Desktop Homunculus tray icon in your OS menu bar / system tray
2. Select **"Exit"**
3. The application shuts down gracefully

Under the hood, the MOD calls the engine's `AppApi::exit()` endpoint to trigger a clean shutdown.

## Notes

- On **Windows**, this MOD is particularly important because there is no standard window close button on the transparent overlay window. The tray exit menu provides the primary way to quit the application.
- On **macOS**, you can also quit via `Cmd+Q` or the Dock menu, but the tray entry offers a consistent cross-platform option.
