---
title: "Publishing & Distribution"
sidebar_position: 12
---

# Publishing & Distribution

MODs are standard npm packages. To distribute your MOD, publish it to the [npm registry](https://www.npmjs.com/). Users install published MODs with a single command:

```bash
hmcs mod install <package-name>
```

## Package Naming

:::warning
The `@hmcs/` scope on npm is reserved for official MODs. Do not publish packages under this scope.
:::

When publishing your own MOD, use one of these conventions:

- **Scoped** (recommended) — Use your own npm scope: `@yourname/hmcs-my-mod`
- **Unscoped** — Use a `hmcs-` prefix: `hmcs-my-mod`

The package `name` in `package.json` is what users pass to `hmcs mod install`. It is also used to derive the mod name for asset IDs, menu entries, and bin commands. See [Asset IDs](./project-setup/asset-ids.md) for details.

## Before You Publish

:::tip
Test your MOD locally before publishing. Install it from a local path and verify that assets load and scripts run correctly:

```bash
hmcs mod install /path/to/your-mod
```
:::

## Publish to npm

### 1. Log in to npm

If you don't have an npm account, create one at [npmjs.com/signup](https://www.npmjs.com/signup). Then authenticate from the terminal:

```bash
npm login
```

### 2. Verify package.json

Make sure your `package.json` includes all required fields:

```json
{
  "name": "@yourname/hmcs-my-mod",
  "version": "1.0.0",
  "type": "module",
  "description": "A short description of your MOD",
  "homunculus": {
    "service": "index.ts",
    "assets": {}
  }
}
```

Key fields:

| Field         | Required | Notes                         |
| ------------- | -------- | ----------------------------- |
| `name`        | Yes      | Must be unique on npm         |
| `version`     | Yes      | [Semver](https://semver.org/) |
| `type`        | Yes      | Must be `"module"`            |
| `description` | Yes      | Shown in `hmcs mod list`      |
| `homunculus`  | Yes      | What makes it a MOD           |

See [Package Configuration](./project-setup/package-json.md) for full details on the `homunculus` field and `bin`.

### 3. Control what gets published

By default, npm publishes everything in your project directory. Use the `files` field in `package.json` to include only what's needed:

```json
{
  "files": [
    "index.ts",
    "commands/",
    "assets/"
  ]
}
```

Alternatively, create a `.npmignore` file to exclude specific paths. Either way, make sure your published package includes all asset files declared in `homunculus.assets`.

### 4. Publish

For **scoped** packages (e.g., `@yourname/hmcs-my-mod`):

```bash
npm publish --access public
```

For **unscoped** packages:

```bash
npm publish
```

### 5. Verify

Install your published MOD to confirm everything works:

```bash
hmcs mod install @yourname/hmcs-my-mod
```

Restart Desktop Homunculus and verify that your MOD loads correctly.

## Updating a Published MOD

To publish a new version:

1. Update the `version` field in `package.json`
2. Run `npm publish` (or `npm publish --access public` for scoped packages)

Users update by reinstalling:

```bash
hmcs mod install @yourname/hmcs-my-mod
```
