# gripe

Submit structured feedback as GitHub issues from the command line.

`gripe` turns a YAML schema into an interactive (or fully automated) issue-creation workflow. Define the fields you care about, and gripe handles prompting, validation, formatting, and submission via `gh`.

## Install

### Homebrew (macOS/Linux)

```sh
brew install agent-clis/gripe/gripe
```

### Shell (macOS/Linux)

```sh
curl -fsSL https://raw.githubusercontent.com/agent-clis/gripe/main/install.sh | sh
```

### PowerShell (Windows)

```powershell
irm https://raw.githubusercontent.com/agent-clis/gripe/main/install.ps1 | iex
```

### npm

```sh
npm install -g @agent-clis/gripe
```

### Cargo (build from source)

```sh
cargo install --path .
```

Requires the [GitHub CLI](https://cli.github.com) (`gh`) to be installed and authenticated.

## Quick start

```sh
# Initialize a gripe.yaml in your project
gripe init

# Submit feedback interactively
gripe submit

# Submit programmatically
gripe submit tool=vim summary="cursor jumps on save"

# Preview without creating an issue
gripe submit --dry-run tool=vim summary="cursor jumps"
```

## Commands

### `gripe submit`

Create a GitHub issue from structured input. Input can come from:

- **Interactive prompts** (default) — walks through each field
- **Key-value args** — `gripe submit field1=value1 field2=value2`
- **JSON flag** — `gripe submit --json '{"field1": "value1"}'`
- **Stdin** — `echo '{"field1": "value1"}' | gripe submit --stdin`

| Flag            | Description                        |
| --------------- | ---------------------------------- |
| `--json <JSON>` | Provide field values as JSON       |
| `--stdin`       | Read JSON from stdin               |
| `--dry-run`     | Preview the issue without creating |
| `--output-json` | Output result as JSON              |
| `--repo <REPO>` | Target repository (`owner/repo`)   |

### `gripe init`

Generate a starter `gripe.yaml` in the current directory.

| Flag      | Description                  |
| --------- | ---------------------------- |
| `--force` | Overwrite existing gripe.yaml |

### `gripe schema`

Display the resolved schema (useful for debugging which config is active).

| Flag     | Description      |
| -------- | ---------------- |
| `--json` | Output as JSON   |

## Configuration

gripe resolves its schema through a fallback chain:

1. **`gripe.yaml`** — walks up from the current directory
2. **`.github/ISSUE_TEMPLATE/*.yml`** — GitHub issue templates in the repo
3. **Built-in default** — a generic feedback form

The target repository is auto-detected from the git remote if not specified.

### `gripe.yaml`

```yaml
repo: owner/repo          # optional, auto-detected from git remote
automated: allow           # allow | deny — controls programmatic submissions
labels:
  - feedback
title_template: "[{tool}] {summary}"
fields:
  - id: tool
    label: Tool Name
    type: input
    required: true
  - id: summary
    label: Summary
    type: input
    required: true
  - id: severity
    label: Severity
    type: select
    options: [low, medium, high, critical]
    default: medium
  - id: context
    label: Additional Context
    type: textarea
```

### Field types

| Type       | Description               |
| ---------- | ------------------------- |
| `input`    | Single-line text          |
| `textarea` | Multi-line text           |
| `select`   | Choose from `options` list |

### Automated policy

Repos can set `automated: deny` in their `gripe.yaml` to reject non-interactive submissions. gripe checks this before creating issues programmatically.

## CI / scripting

```sh
gripe submit \
  --repo owner/repo \
  --json '{"tool": "linter", "summary": "false positive on rule X"}' \
  --output-json
```

The `--output-json` flag returns structured output:

```json
{
  "url": "https://github.com/owner/repo/issues/42",
  "number": 42,
  "repo": "owner/repo",
  "title": "[linter] false positive on rule X"
}
```
