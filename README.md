# sync-constitution

A [pre-commit](https://pre-commit.com) hook that syncs a single `CONSTITUTION.md` file to agent-specific config files, keeping all your AI coding assistants aligned with one source of truth.

## Concept

Instead of maintaining separate rule files for each AI agent (Claude, Cursor, Copilot, Windsurf, Aider), you write your project rules once in `CONSTITUTION.md`. This hook automatically generates the agent-specific files whenever `CONSTITUTION.md` changes.

## Supported Agents

| Agent    | Output File                          | Format   |
|----------|--------------------------------------|----------|
| claude   | `CLAUDE.md`                          | Markdown |
| cursor   | `.cursorrules`                       | Markdown |
| copilot  | `.github/copilot-instructions.md`    | Markdown |
| windsurf | `.windsurfrules`                     | Markdown |
| aider    | `.aider.conf.yml`                    | YAML     |

## Usage

1. Create a `CONSTITUTION.md` in your repo root with your project rules.

2. Add to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/danielsteman/constitution-md
    rev: v1.0.0
    hooks:
      - id: sync-constitution
        args: [claude, cursor, copilot]
```

3. List the agents you want to sync in `args`.

4. The hook runs only when `CONSTITUTION.md` is staged, generates the target files, and automatically stages them for the commit.

## How It Works

- Each generated file includes a header comment indicating it's auto-generated
- Markdown-based files get an HTML comment header
- YAML files get a `#` comment header
- The content of `CONSTITUTION.md` is placed directly after the header (or wrapped in a `conventions` key for YAML)

## Adding New Agents

The agent mapping in `src/main.rs` is easy to extend. Add a new entry to the `agent_configs()` function with:

- `path`: output file path relative to repo root
- `header`: auto-generation notice in appropriate comment format
- `wrap`: optional, set to `Some("yaml")` to wrap content in a YAML key
