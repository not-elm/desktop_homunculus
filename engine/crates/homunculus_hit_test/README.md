# homunculus_hit_test

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_hit_test` provides hit testing functionality for VRM models in the Homunculus application.
It enables the application to detect when the cursor is over a model, which is essential for user interaction with the
desktop mascot.

## Features

- **Ray Casting**: Perform hit testing using ray casting from the cursor position
- **Window Integration**: Update window hit test properties based on cursor position
- **VRM Model Detection**: Detect when the cursor is over a VRM model
- **Event-Based Updates**: Update hit testing when pointer events occur
- **Development Mode**: Special handling for development mode with egui integration
- **Performance Optimization**: Only update hit testing when necessary in production mode
- **MToon Material Support**: Specifically designed to work with MToon materials used by VRM models