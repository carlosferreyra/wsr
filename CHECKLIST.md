# wsr v0.1 implementation checklist

## 1. Project structure

- [ ] Create `src/lib.rs` and declare all top-level modules
- [ ] Create `src/cli/mod.rs`
- [ ] Create `src/config/mod.rs`
- [ ] Create `src/provider/mod.rs`
- [ ] Create `src/sandbox/mod.rs`
- [ ] Create `src/sync/mod.rs`
- [ ] Create `src/engine/mod.rs`

## 2. CLI skeleton

- [ ] Add `clap` dependency with `derive` feature
- [ ] `wsr init` subcommand
- [ ] `wsr run` subcommand
  - [ ] `--event <n>` flag
  - [ ] `--dry-run` flag
  - [ ] `--verbose` flag
  - [ ] `--yes` flag
  - [ ] optional `<file>` positional argument
- [ ] Wire subcommands into `main.rs`

## 3. wsr.json config

- [ ] Add `serde`, `serde_json` dependencies
- [ ] `Config` struct with `provider`, `sandbox` fields
- [ ] `SandboxConfig` struct with `allowed_hosts`, `secrets_from` fields
- [ ] `Config::load()` — read and parse `wsr.json` from repo root
- [ ] `Config::generate()` — write default `wsr.json` with `$schema`
- [ ] JSON Schema URL hardcoded as `https://wsr.dev/schema/wsr.json`

## 4. GitHub Actions YAML parser

- [ ] Add `serde-yaml`, `schemars` dependencies
- [ ] `Workflow` struct — `name`, `on`, `jobs`
- [ ] `Trigger` enum — `push`, `pull_request`, `workflow_dispatch`, tag patterns
- [ ] `Job` struct — `runs-on`, `needs`, `steps`, `outputs`, `env`, `if`, `strategy`
- [ ] `Step` struct — `name`, `run`, `uses`, `with`, `env`, `if`, `continue-on-error`,
      `working-directory`
- [ ] `Matrix` struct — basic `include`/`exclude`
- [ ] `WorkflowParser::parse(path)` — deserialize a workflow file

## 5. Expression engine

- [ ] `${{ }}` tokenizer and parser
- [ ] `github.*` context — `event_name`, `ref`, `sha`, `actor`, `repository`
- [ ] `env.*` context
- [ ] `runner.*`, `secrets.*` contexts
- [ ] `fromJSON` / `toJSON`
- [ ] String functions — `contains`, `startsWith`, `endsWith`, `format`
- [ ] Status functions — `success()`, `failure()`, `always()`, `cancelled()`
- [ ] `ExpressionEngine::eval(expr, context)` entry point

## 6. Trigger → git hook mapping

- [ ] `TriggerMap` — maps `on:` triggers to git hook names
- [ ] `push` / `pull_request` → `pre-push`
- [ ] `workflow_dispatch` → manual (`wsr run`)
- [ ] Tag pattern triggers → `pre-push`
- [ ] `HookShim::write(hook_name, workflows)` — write shim script to `.git/hooks/`
- [ ] Shim includes embedded manifest comment for stateless reconcile

## 7. `wsr init`

- [ ] Scan `.github/workflows/` for YAML files
- [ ] Parse each workflow with #4
- [ ] Generate `wsr.json` via #3
- [ ] Map triggers via #6
- [ ] Install hook shims into `.git/hooks/`
- [ ] Make shims executable (`chmod +x`)
- [ ] Print summary of installed hooks

## 8. Wasm sandbox

- [ ] Add `wasmtime`, `wasi-common` dependencies
- [ ] `Sandbox::new()` — create Wasmtime engine with Cranelift JIT
- [ ] One `Store` + `Instance` per step, dropped after completion
- [ ] WASI capability grants — preopened dirs, env vars
- [ ] `allowed_hosts` network enforcement via hyper proxy
- [ ] Secret injection via env — `zeroize` on drop, never written to disk
- [ ] AOT module cache — `wasmtime::Module::serialize` + SHA-256 pin

## 9. `run:` step execution

- [ ] `StepRunner::run(step, context, sandbox)` entry point
- [ ] `bash` / `sh` shell execution inside sandbox
- [ ] `pwsh` shell execution inside sandbox
- [ ] `env:` resolution at step and job level
- [ ] `if:` condition evaluation via expression engine
- [ ] `continue-on-error:` handling
- [ ] `working-directory:` support

## 10. Job DAG + matrix

- [ ] `Dag::build(jobs)` — resolve `needs:` into execution order
- [ ] Sequential job execution
- [ ] Parallel job execution where DAG allows
- [ ] `outputs:` propagation between jobs
- [ ] Basic matrix expansion — cartesian product of matrix values
- [ ] `strategy.fail-fast` handling

## 11. Action resolver (Javy)

- [ ] Fetch action metadata from GitHub (tags, SHAs)
- [ ] Download action source
- [ ] Compile JS/TS → Wasm via Javy CLI
- [ ] Cache compiled `.wasm` with content hash
- [ ] `actions/checkout@v4` support
- [ ] `actions/setup-node@v4` support
- [ ] `actions/cache@v4` support

## 12. Sync hooks

- [ ] `post-checkout` hook — resync on branch switch
- [ ] `post-merge` hook — resync after pull
- [ ] `post-rewrite` hook — resync after rebase/amend
- [ ] Atomic shim writes — write tmpfile → `rename()`
- [ ] Manifest comment in shim — embedded desired state, no lock file
- [ ] Named warning on workflow deletion

## 13. Observability

- [ ] Add `tracing`, `tracing-subscriber` dependencies
- [ ] Structured step logs via `tracing`
- [ ] Per-step timing
- [ ] Sandbox violation events
- [ ] Exit code propagation to git
- [ ] `--format=gha` — GitHub Annotations output format
