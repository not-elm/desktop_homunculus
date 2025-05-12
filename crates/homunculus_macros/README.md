# homunculus_macros

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_macros` provides procedural macros for the Homunculus project. These macros simplify common patterns and reduce boilerplate code, particularly for scripting functionality.

## Features

- **ScriptArgs Derive Macro**: Automatically generates code for converting script arguments to Rust structs
- **Type Conversion**: Handles conversion of various types (String, numeric types, boolean, etc.) from script values
- **Optional Field Support**: Properly handles optional fields in argument structs
- **Error Handling**: Provides clear error messages for missing required arguments
- **Reflection Support**: Supports complex types through Bevy's reflection system
- **Integration with bevy_mod_scripting**: Designed to work seamlessly with the scripting system