---
title: Contributing
sidebar_position: 1
---

# Contributing to Desktop Homunculus

Thank you for your interest in contributing! Desktop Homunculus is in active development and welcomes contributions of all kinds — from asset creation to mod development to bug reports.

## Development Setup

See [Development Setup](./development-setup) for prerequisites and environment setup instructions.

## How to Contribute

- **Report bugs**: Open an issue on [GitHub Issues](https://github.com/not-elm/desktop-homunculus/issues)
- **Suggest features or improvements**: Open a [Proposal issue](https://github.com/not-elm/desktop-homunculus/issues/new?template=proposal.md) or start a thread in [GitHub Discussions](https://github.com/not-elm/desktop-homunculus/discussions)
- **Submit code**: Fork the repo, create a branch, and open a Pull Request
- **Ask questions**: Use [GitHub Discussions](https://github.com/not-elm/desktop-homunculus/discussions)

### Pull Request Guidelines

1. Fork the repository and create a feature branch
2. Make your changes with clear, descriptive commits
   Use [Conventional Commits](https://www.conventionalcommits.org/) style: `feat:`, `fix:`, `docs:`, `refactor:`, etc.
3. Ensure CI checks pass
4. Before opening a PR, run `make test` (tests) and `make fix-lint` (lint) locally.
5. Open a PR with a description of what you changed and why
6. Respond to review feedback

## Wanted Contributions

Desktop Homunculus is currently missing many features and actively seeking contributions in the following areas.

### Providing Assets

The `@hmcs/assets` mod provides official assets, but the collection is still small. We welcome contributions of:

- **Sound effects** — UI sounds, reactions, ambient effects
- **BGM** — Background music tracks
- **VRM models** — 3D character models in VRM format
- **VRMA animations** — Motion animations for VRM characters

:::warning License Requirements
All contributed assets must have a compatible license (e.g., CC0, CC-BY). Please include license information with your submission.
:::

### MOD Development

If you have ideas for official mods, we welcome Pull Requests. You can also propose ideas via Issues or Discussions before writing code.

See the [MOD Development Guide](/mod-development/quick-start) for how to create mods.

### User Skills

We welcome contributions of Claude Code Skills that enhance the end-user experience with Desktop Homunculus characters. Skills chain MCP tool calls into workflows — for example, a skill might have a character deliver a lecture with slides and narration.

See the [Skills catalog and contribution guide](https://github.com/not-elm/desktop-homunculus/tree/main/skills) for available skills, installation instructions, and how to create your own.

### `@hmcs/ui` Improvements

We welcome contributions that improve the shared `@hmcs/ui` component library used by MOD WebView UIs.

- **New reusable components** — Add components that are broadly useful for MOD settings and in-game tools
- **Accessibility improvements** — Improve keyboard navigation, focus states, ARIA semantics, and screen reader support
- **Design and interaction polish** — Refine visual consistency, spacing, states, and motion to improve usability
- **Documentation and examples** — Improve component docs, usage guidance, and practical examples for MOD authors

See the [Component Library guide](/mod-development/webview-ui/component-library) for current usage and API examples.

## Developer Certificate of Origin (DCO)

By contributing, you certify that the contribution is your original work and that you have the right to submit it under the project license (the [Developer Certificate of Origin](https://developercertificate.org/)).

## License

By contributing, you agree that your contributions will be licensed under the same license as the component you're contributing to (MIT/Apache-2.0 for Rust code, MIT for TypeScript code, CC-BY-4.0 for documentation and creative assets).

## Questions?

Open a [GitHub Discussion](https://github.com/not-elm/desktop-homunculus/discussions) — we're happy to help you get started.
