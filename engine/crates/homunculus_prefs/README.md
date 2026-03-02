# homunculus_prefs

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_prefs` provides a preferences management system for the Homunculus application. It enables storing and retrieving application settings and VRM model states using an SQLite database.

## Features

- **SQLite Database**: Stores preferences in a persistent SQLite database
- **JSON Serialization**: Supports storing complex data structures as JSON
- **Key-Value Storage**: Simple key-value interface for storing preferences
- **VRM Transform Persistence**: Automatically saves VRM model transforms on application exit
- **In-Memory Fallback**: Falls back to an in-memory database if file access fails
- **Error Handling**: Comprehensive error handling for database operations
- **Application Data Directory**: Stores preferences in the appropriate application data directory