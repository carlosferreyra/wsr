# wsr — workflow sandboxed runner

A local, Wasm-sandboxed CI runner. Run your workflows on every git hook — before you push, not
after.

```
cargo install wsr
cd my-repo
wsr init
```

---

## the problem

GitHub Actions is the standard. But the feedback loop is broken: you push, wait for CI, read a
failure, fix it, push again. `act` helped but shells out to Docker — no real sandboxing, root
privileges, and a container daemon you have to babysit.

`wsr` runs your workflows locally, on every git hook, in a Wasm sandbox. Every step gets its own
isolated Wasmtime instance with explicit capability grants. No Docker. No surprises on remote.

---

## how it works

`wsr` maps your workflow triggers to git hooks automatically:

| workflow trigger            | git hook           |
| --------------------------- | ------------------ |
| `on: push`                  | `pre-push`         |
| `on: pull_request`          | `pre-push`         |
| `on: workflow_dispatch`     | `wsr run` (manual) |
| `on: push` with tag pattern | `pre-push`         |

When you run `git push`, your `ci.yml` runs locally first. If it fails, the push is blocked. If it
passes, you already know remote CI will pass too — same workflow syntax, same expressions, same
action versions.

---

## install

```bash
cargo install wsr
```

**requirements**: Rust 1.78+. No Docker. No daemon required to get started.

---

## usage

### init a repo

```bash
wsr init
```

Scans your workflow files, maps triggers to git hooks, and writes shims into `.git/hooks/`. A
`wsr.json` is generated at the repo root for any overrides you need. No manual config required.

### run a workflow manually

```bash
# runs all workflows with workflow_dispatch trigger
wsr run

# run a specific workflow, ignoring its triggers
wsr run .github/workflows/ci.yml

# force a specific event
wsr run --event push

# dry run — print the execution plan, run nothing
wsr run --dry-run

# verbose — show expression evaluation, sandbox capability grants
wsr run --verbose
```

### debug a workflow you're authoring

Add `workflow_dispatch` to your workflow while writing it:

```yaml
on:
  push:
  workflow_dispatch: # enables wsr run locally
```

Then iterate locally without pushing:

```bash
wsr run   # runs immediately, no push needed
```

Remove or keep `workflow_dispatch` when done — it works identically on remote CI.

### keep hooks in sync

```bash
wsr daemon
```

Watches your workflow directory for changes and keeps git hooks up to date. Useful when actively
authoring workflows or working across branches with different workflow sets.

---

## configuration

`wsr init` generates a `wsr.json` at the repo root. It is optional — sensible defaults apply without
it.

```json
{
 "$schema": "https://wsr.dev/schema/wsr.json",
 "provider": "github",
 "sandbox": {
  "allowed_hosts": [],
  "secrets_from": ".env.wsr"
 }
}
```

`wsr.json` is readable by any language and toolchain without extra dependencies. It also serves as
the interchange format between `wsr` internals, the daemon, and provider adapters. The `$schema`
field enables IntelliSense and inline validation in VS Code and any JSON Schema-aware editor.

---

## sandbox model

Every step runs in its own Wasmtime instance, compiled with Cranelift JIT and running on WASI 3 —
which provides a native async layer to Wasm. Steps get exactly the capabilities they need and
nothing more.

A step that tries to reach a host outside `allowed_hosts` gets a capability denied error — locally,
immediately, with a hint — not a silent failure on remote CI 10 minutes later:

```
capability denied: net → registry.example.com
hint: add to sandbox.allowed_hosts in wsr.json
```

---

## provider support

`wsr` is built around a provider adapter pattern. GitHub Actions is the reference implementation.
GitLab CI and Bitbucket Pipelines are planned as first-class adapters — same sandbox, same execution
engine, different workflow syntax and context normalization.

| provider            | status                |
| ------------------- | --------------------- |
| GitHub Actions      | v0.1 — reference impl |
| GitLab CI           | v0.4 — planned        |
| Bitbucket Pipelines | v0.5 — planned        |

The active provider is declared in `wsr.json`. Switching providers changes the parser and context
builder — the sandbox and execution engine are unchanged.

---

## github actions compatibility

`wsr` targets 100% GitHub Actions syntax compatibility. MVP coverage:

- [x] `run:` steps with `bash`, `sh`, `pwsh`
- [x] `uses:` — marketplace actions via Javy (JS/TS → Wasm)
- [x] `${{ }}` expression engine — `github.*`, `env.*`, `needs.*`, `fromJSON`, `toJSON`
- [x] matrix expansion
- [x] `needs:` DAG
- [x] composite actions (inlined natively)
- [ ] Docker-based actions — compat shim with warning in v0.1, native in v0.2

---

## branch sync

Switching branches or pulling from remote automatically resyncs your hooks:

```
git checkout feat/dev
# [wsr] sync · pre-commit → lint.yml, test.yml (was: ci.yml)

git pull
# [wsr] sync · pre-push → ci.yml (updated)
# [wsr] sync · pre-commit hook removed (lint.yml deleted on this branch)
```

`wsr` trusts git to own the filesystem. Sync is a stateless reconcile — scan, diff, atomic write.

---

## vs act

|                     | act                  | wsr                                     |
| ------------------- | -------------------- | --------------------------------------- |
| sandbox             | Docker namespaces    | Wasmtime per step                       |
| requires daemon     | Docker daemon        | nothing (optional wsr daemon for watch) |
| CI provider support | GitHub Actions       | GitHub · GitLab · Bitbucket (planned)   |
| local trigger       | manual               | git hooks                               |
| secret safety       | env vars             | capability grants + zeroize             |
| async runtime       | —                    | WASI 3 native async                     |
| performance         | container cold start | Cranelift JIT + AOT cache               |

---

## license

MIT
