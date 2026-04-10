# cli

Entry point for all user-facing commands. Parses arguments and dispatches to the appropriate
subsystem.

Implemented with [`clap`](https://docs.rs/clap) using the derive API.

## commands

### `wsr init`

Scans `.github/workflows/`, generates `wsr.json`, and installs git hook shims into `.git/hooks/`.

### `wsr run [file]`

Executes workflows locally. Without `[file]`, runs all workflows matching the active trigger.

| flag          | description                                                  |
| ------------- | ------------------------------------------------------------ |
| `--event <n>` | Force a specific event type (e.g. `push`, `pull_request`)    |
| `--dry-run`   | Print the execution plan without running anything            |
| `--verbose`   | Show expression evaluation, sandbox grants, and context dump |
| `--yes`       | Skip interactive prompts (for scripted use)                  |

## source

- [`src/cli/mod.rs`](../../src/cli/mod.rs)
