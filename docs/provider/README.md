# provider

Workflow provider adapters. Each provider knows how to parse a CI workflow format, build execution
context, and map triggers to git hooks.

## adapter pattern

All providers implement the `WorkflowProvider` trait:

```rust
trait WorkflowProvider {
    fn parse(&self, path: &Path) -> Result<Workflow>;
    fn context(&self, event: &str) -> Result<Context>;
    fn trigger_map(&self) -> TriggerMap;
}
```

Switching providers in `wsr.json` changes the parser and context builder — the sandbox and execution
engine are unchanged.

## github actions (reference implementation)

The reference provider. Targets 100% GitHub Actions syntax compatibility.

**parser** — deserializes `.github/workflows/*.yml` using `serde-yaml`.

**context builder** — populates `github.*`, `env.*`, `runner.*`, `secrets.*`.

**trigger map**

| workflow trigger            | git hook           |
| --------------------------- | ------------------ |
| `on: push`                  | `pre-push`         |
| `on: pull_request`          | `pre-push`         |
| `on: workflow_dispatch`     | manual (`wsr run`) |
| `on: push` with tag pattern | `pre-push`         |

## planned providers

| provider            | version |
| ------------------- | ------- |
| GitLab CI           | v0.4    |
| Bitbucket Pipelines | v0.5    |

## source

- [`src/provider/mod.rs`](../../src/provider/mod.rs)
