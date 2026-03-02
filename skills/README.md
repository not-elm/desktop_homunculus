# Homunculus Skills

Claude Code Skills for controlling Desktop Homunculus characters. Each skill chains [MCP tools](https://not-elm.github.io/desktop-homunculus/docs/ai-integration/) into workflows — characters can lecture, present, react, and more.

## Available Skills

| Skill | Description |
|-------|-------------|
| | |

## Installation

1. Clone this repository (or download the skill you want):

   ```bash
   git clone https://github.com/not-elm/desktop-homunculus.git
   ```

2. Copy the skill to your Claude Code skills directory:

   ```bash
   cp -r desktop-homunculus/skills/<skill-name> ~/.claude/skills/
   ```

3. Verify the skill is loaded — it should appear in Claude Code's skill list.

### Prerequisites

- [Desktop Homunculus](https://not-elm.github.io/desktop-homunculus/docs/getting-started/installation) installed and running
- Claude Code with the [Homunculus MCP server configured](https://not-elm.github.io/desktop-homunculus/docs/ai-integration/setup/claude-code)

## Contributing

We welcome new skills! To contribute:

1. Create a directory: `skills/<your-skill-name>/`
2. Add a `SKILL.md` with YAML frontmatter:

   ```yaml
   ---
   name: your-skill-name
   description: >
     What this skill does (1-3 sentences).
   ---
   ```

3. Write the skill body in Markdown — describe the workflow, which MCP tools to use, and any assets
4. If your skill uses assets (HTML templates, images, etc.), put them in `skills/<your-skill-name>/assets/`
5. Add an entry to the **Available Skills** table above
6. Open a Pull Request

### Guidelines

- Skills should use Homunculus MCP tools (`speak_message`, `open_webview`, `play_animation`, `set_expression`, etc.)
- Keep skill descriptions clear — users should understand what the skill does before installing
- Include example invocations in the SKILL.md workflow
- Assets (HTML, images) should be self-contained and under 50KB each
