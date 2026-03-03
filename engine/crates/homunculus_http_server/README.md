# homunculus_http_server

This crate is part of the [Homunculus project](https://github.com/not-elm/desktop_homunculus)

## Overview

`homunculus_http_server` provides an HTTP server implementation for the Homunculus application. It exposes a RESTFUL API
that allows external applications to interact with and control the desktop mascot through HTTP requests.

## Features

- **RESTFUL API**: Exposes a comprehensive HTTP API for controlling the desktop mascot
- **Axum Integration**: Built on the Axum web framework for efficient request handling
- **VRM Model Control**: Endpoints for managing VRM models and their properties
- **Animation Control**: API for controlling VRMA animations
- **Camera Management**: Endpoints for adjusting camera settings and perspective
- **Shadow Panel Control**: API for managing shadow rendering
- **WebView Integration**: Endpoints for controlling embedded web content
- **Preferences Management**: API for accessing and modifying user preferences
- **Display Configuration**: Endpoints for managing display settings
- **Effects Control**: API for triggering visual and audio effects
- **Script Execution**: Endpoints for running scripts and extensions
- **Asynchronous Processing**: Non-blocking request handling with Tokio runtime