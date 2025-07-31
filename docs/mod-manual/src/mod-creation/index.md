# MOD Creation Guide

This comprehensive guide will walk you through creating your first MOD for Desktop Homunculus. By the end of this
section, you'll understand how to structure, configure, and develop fully functional MODs.

## Overview

Creating a MOD involves several key steps:

1. **[Setting up the directory structure](./directory-structure.md)** - Organizing your files properly
2. **[Writing the configuration file](./mod-json.md)** - Defining your MOD's metadata and behavior
3. **[Configuring menus](./menus.md)** - Adding entries to character context menus
4. **[Setting up system menus](./system-menus.md)** - Integrating with the system tray
5. **[Creating startup scripts](./startup-scripts.md)** - Running code when the application starts

## Quick Start Example

Let's create a simple "Hello World" MOD to demonstrate the basic concepts:

### 1. Create the Directory

```
assets/mods/hello-world/
├── mod.json
├── index.html
└── icon.png
```

### 2. Basic Configuration (`mod.json`)

```json
{
  "name": "hello-world",
  "version": "1.0.0",
  "description": "A simple Hello World MOD",
  "author": "Your Name",
  "menus": [
    {
      "text": "Hello World",
      "thumbnail": "hello-world/icon.png",
      "webview": {
        "source": "hello-world/index.html",
        "resolution": [
          300,
          200
        ],
        "transparent": true,
        "position": {
          "offset": [
            -350,
            -200
          ]
        }
      }
    }
  ]
}
```

### 3. Simple Interface (`index.html`)

```html
<!DOCTYPE html>
<html>
<head>
    <title>Hello World</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            padding: 20px;
            background: rgba(255, 255, 255, 0.9);
            border-radius: 10px;
            text-align: center;
        }

        button {
            padding: 10px 20px;
            background: #007acc;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }

        button:hover {
            background: #005999;
        }
    </style>
</head>
<body>
<h2>Hello World!</h2>
<p>This is your first Desktop Homunculus MOD.</p>
<button onclick="sayHello()">Click Me!</button>

<script>
    function sayHello() {
        alert("Hello from your MOD!");
    }
</script>
</body>
</html>
```

### 4. Testing Your MOD

1. Place your MOD directory in `assets/mods/hello-world/`
2. Restart Desktop Homunculus
3. Right-click on any VRM character
4. Select "Hello World" from the context menu
5. Your MOD interface should appear!

## MOD Loading Process

Understanding how MODs are loaded helps you debug issues:

1. **Discovery**: Desktop Homunculus scans the `assets/mods/` directory on startup
2. **Configuration Parsing**: Each `mod.json` file is read and validated
3. **Asset Registration**: All files in MOD directories are registered as available assets
4. **Menu Integration**: Menu items defined in configuration are added to the UI
5. **Script Execution**: Any startup scripts are executed in the builtin Deno runtime

## Common MOD Patterns

### UI-Only MODs

Perfect for simple interfaces and tools:

- Configuration panels
- Information displays
- Simple games or utilities

### Script-Only MODs

Great for automation and background functionality:

- Automated character behaviors
- External API integrations
- Scheduled tasks and reminders

### Hybrid MODs

Combining UI and scripting for complex functionality:

- Interactive applications
- Real-time data displays
- Multi-component systems

## Development Tips

### Hot Reload Support

Desktop Homunculus supports hot reloading during development:

- Changes to HTML/CSS/JS files are automatically detected
- Webviews refresh automatically when files change
- No need to restart the application during development

### Asset Referencing

Use the `mod-name/file-path` format for all asset references within your MOD:

- `"hello-world/icon.png"` refers to `/assets/mods/hello-world/icon.png`
- `"my-mod/sounds/click.mp3"` refers to `/assets/mods/my-mod/sounds/click.mp3`
- Paths are always relative to the `assets/mods/` directory

### Testing Strategy

- Start with simple functionality and gradually add complexity
- Test your MOD on different operating systems if possible
- Verify that your MOD works with multiple VRM characters
- Check that menu items appear correctly and webviews open properly

## Next Steps

Now that you understand the basics, dive deeper into each aspect:

- **[Directory Structure](./directory-structure.md)** - Learn about optimal file organization
- **[Configuration File](./mod-json.md)** - Master the `mod.json` format
- **[Menus](./menus.md)** - Create compelling menu experiences
- **[System Menus](./system-menus.md)** - Integrate with the system tray
- **[Startup Scripts](./startup-scripts.md)** - Automate functionality on app launch

For more advanced functionality, explore:

- **[Webview UI Development](../webview-ui/index.md)** - Build sophisticated interfaces
- **[TypeScript SDK](../sdk/index.md)** - Access powerful programming capabilities