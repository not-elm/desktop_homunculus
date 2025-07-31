# homunculus_deno

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_deno` integrates the Deno JavaScript/TypeScript runtime with the Homunculus project.
It provides functionality for executing JavaScript and TypeScript code within the application, enabling scripting and
extensibility.

## Features

- **JavaScript Execution**: Run JavaScript files from the application's assets
- **Deno Runtime Integration**: Leverage the secure Deno runtime for executing code
- **Event-Based API**: Request script execution through Bevy's event system
- **Asynchronous Execution**: Scripts run asynchronously without blocking the main thread
- **Permission Management**: Control what scripts can access through Deno's permission system
- **Error Handling**: Comprehensive error reporting for script execution failures