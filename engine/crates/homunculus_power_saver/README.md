# homunculus_power_saver

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_power_saver` provides power-saving functionality for the Homunculus application. It optimizes resource usage by adjusting frame rates and update modes based on the application's activity state.

## Features

- **Frame Rate Limiting**: Caps the frame rate to 60 FPS to reduce GPU and CPU usage
- **Reactive Update Mode**: Switches to low-power mode when the application is idle
- **Loading State Tracking**: Monitors loading components to determine application activity
- **Automatic Mode Switching**: Transitions between active and sleep modes based on content loading status
- **Window Cursor Management**: Disables hit testing in sleep mode to prevent blocking operations
- **VRM and VRMA Loading Detection**: Automatically tracks loading of VRM models and animations
- **Component Hook Registration**: Uses Bevy's component hooks to detect loading states