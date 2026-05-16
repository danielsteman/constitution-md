# User Guide

## What is sync-constitution?

sync-constitution is a [pre-commit](https://pre-commit.com) hook that lets you maintain a single `CONSTITUTION.md` file as the source of truth for all your AI coding assistant configurations. When you commit changes to `CONSTITUTION.md`, the hook automatically generates the correct config file for each agent you use.

## Quick Start

### 1. Install pre-commit

```bash
pip install pre-commit
```

### 2. Create your CONSTITUTION.md

Create a `CONSTITUTION.md` file in the root of your repository. Write your project rules and conventions in Markdown:

```markdown
# Project Rules

- Use TypeScript with strict mode enabled
- Write unit tests for all new functions
- Follow the repository's existing naming conventions
- Keep functions small and focused
- Prefer composition over inheritance
```

### 3. Configure the hook

Add the following to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/danielsteman/constitution-md
    rev: v1.0.0
    hooks:
      - id: sync-constitution
        args: [claude, cursor, copilot]
```

Replace the `args` list with the agents you want to sync to.

### 4. Install the hook

```bash
pre-commit install
```

### 5. Commit your CONSTITUTION.md

```bash
git add CONSTITUTION.md .pre-commit-config.yaml
git commit -m "Add constitution"
```

The hook will run automatically, generate the agent config files, and include them in your commit.

## Supported Agents

| Agent      | Config File                        | Format   |
|------------|------------------------------------|----------|
| `claude`   | `CLAUDE.md`                        | Markdown |
| `cursor`   | `.cursorrules`                     | Markdown |
| `copilot`  | `.github/copilot-instructions.md`  | Markdown |
| `windsurf` | `.windsurfrules`                   | Markdown |
| `aider`    | `.aider.conf.yml`                  | YAML     |

## How It Works

1. The hook triggers only when `CONSTITUTION.md` is staged for commit.
2. For each agent listed in `args`, it generates the corresponding config file.
3. Markdown-based files receive an HTML comment header: `<!-- AUTO-GENERATED FROM CONSTITUTION.md — DO NOT EDIT DIRECTLY -->`
4. YAML-based files (aider) receive a `#` comment header and the content is wrapped in a `conventions` key.
5. All generated files are automatically staged with `git add`.

## Example

Given this `CONSTITUTION.md`:

```markdown
# Conventions

- Use Rust for systems code
- Use Python for scripts
- All public functions must have doc comments
```

The hook generates the following `CLAUDE.md`:

```markdown
<!-- AUTO-GENERATED FROM CONSTITUTION.md — DO NOT EDIT DIRECTLY -->

# Conventions

- Use Rust for systems code
- Use Python for scripts
- All public functions must have doc comments
```

And the following `.aider.conf.yml`:

```yaml
# AUTO-GENERATED FROM CONSTITUTION.md — DO NOT EDIT DIRECTLY

conventions: "# Conventions\n\n- Use Rust for systems code\n- Use Python for scripts\n- All public functions must have doc comments\n"
```

## Tips

- **Don't edit generated files directly.** They will be overwritten on the next commit that touches `CONSTITUTION.md`. The auto-generated header serves as a reminder.
- **Add generated files to version control.** This ensures your teammates' AI assistants pick up the rules without needing to run the hook themselves.
- **Run the hook manually** if you want to regenerate files without committing:
  ```bash
  pre-commit run sync-constitution --all-files
  ```
- **Choose only the agents your team uses.** There's no need to generate config files for agents nobody on your team runs.

## Troubleshooting

### "CONSTITUTION.md not found in repo root"

The hook expects `CONSTITUTION.md` to exist at the root of your repository. Make sure the file exists and is named exactly `CONSTITUTION.md`.

### "Unknown agent 'X'"

You passed an agent name that isn't supported. Check the [Supported Agents](#supported-agents) table for valid names.

### Hook doesn't run on commit

The hook only triggers when `CONSTITUTION.md` is part of the staged changes. If you only modified other files, the hook won't execute. Use `--all-files` to force it:

```bash
pre-commit run sync-constitution --all-files
```

### Generated files aren't in the commit

The hook automatically runs `git add` on generated files. If you're seeing issues, ensure you're using a recent version of pre-commit that supports hooks modifying the staging area.
