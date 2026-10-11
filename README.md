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
| continue | `.continue/config.json`              | JSON     |

## Usage

1. Create a `CONSTITUTION.md` in your repo root with your project rules.

2. Add to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/danielsteman/constitution-md
    rev: v1.1.0
    hooks:
      - id: sync-constitution
        args: [claude, cursor, copilot]
```

3. List the agents you want to sync in `args`.

4. The hook runs only when `CONSTITUTION.md` is staged, generates the target files, and automatically stages them for the commit.

## How It Works

Every generated file carries an auto-generation notice so it's never hand-edited. How that notice and the constitution content are serialized depends on the file's `Format`:

| Format     | Notice               | Content placement                              |
|------------|----------------------|------------------------------------------------|
| `Markdown` | HTML comment         | Emitted verbatim after the notice              |
| `Yaml`     | `#` comment          | Escaped under a `conventions` key              |
| `Json`     | `_generated` field   | Escaped under a `conventions` key in an object |

## Adding New Agents

The agent mapping in `src/main.rs` is easy to extend. Add a new entry to the `agent_configs()` function with:

- `path`: output file path relative to repo root
- `format`: one of the `Format` variants (`Format::Markdown`, `Format::Yaml`, `Format::Json`)

## Adding New Formats

Need a format that isn't listed? Add a variant to the `Format` enum and a matching arm in `Format::render`. The notice and any escaping live entirely inside that arm, so no other code needs to change.
