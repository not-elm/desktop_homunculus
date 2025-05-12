# homunculus_sitting

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_sitting` provides functionality for VRM models to sit on window edges in the Homunculus application. It enables the desktop mascot to interact with application windows by perching on their borders.

## Features

- **Window Edge Detection**: Identify suitable edges for the mascot to sit on
- **Sitting Animation**: Play VRMA animations when the mascot sits on a window
- **Window Tracking**: Update mascot position when windows move
- **State Management**: Handle transitions between sitting and other states
- **Automatic Animation**: Automatically load and play sitting animations
- **Window Metadata**: Track window properties for proper sitting behavior
- **Position Calculation**: Determine the correct position for sitting on window edges
- **Event Handling**: Respond to window and state change events