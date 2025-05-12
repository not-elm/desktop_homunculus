# homunculus_mod

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_mod` provides a modding system for the Homunculus application. It enables loading, managing, and executing user-created mods that can extend the functionality of the desktop mascot.

## Features

- **Mod Loading**: Load mods from the mod directory with their assets and configuration
- **JSON Configuration**: Parse mod.json files for mod metadata and settings
- **Asset Management**: Register and manage mod assets for use in the application
- **Autorun Scripts**: Automatically execute scripts when mods are loaded
- **Hot Reloading**: Detect and reload scripts when they are modified
- **Mod Metadata**: Support for mod name, description, version, and author information
- **JavaScript/TypeScript Support**: Run mod scripts using the Deno runtime