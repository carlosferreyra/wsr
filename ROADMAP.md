# wsr roadmap

Status legend: `done` · `in progress` · `planned` · `future`

---

## v0.1 — foundations `in progress`

The core loop: init, run a workflow, sandbox it, block the push on failure. GitHub Actions as the
reference provider.

**CLI**

- [ ] `wsr init` — scan workflows, generate `wsr.json`, install git hook shims
- [ ] `wsr run` — execute workflows matching `workflow_dispatch`
- [ ] `wsr run <file>` — run specific workflow, prompt if no dispatch trigger
- [ ] `wsr run --event <n>` — force a specific event
- [ ] `wsr run --dry-run` — print execution plan, run nothing
- [ ] `wsr run --verbose` — expose expression eval, sandbox grants, context dump
- [ ] `wsr run --yes` — skip interactive prompts (for scripted use)

**configuration**

- [ ] `wsr.json` generation on `wsr init` with `$schema`, provider, sandbox defaults
- [ ] `serde_json` parsing — no extra crates beyond what the engine already uses
- [ ] JSON Schema published at `https://wsr.dev/schema/wsr.json`
- [ ] VS Code IntelliSense via `$schema` field

**provider adapter — github actions (reference impl)**

- [ ] `WorkflowProvider` trait — `parse()`, `context()`, `trigger_map()`
- [ ] GitHub Actions YAML parser — `serde-yaml` + `schemars`
- [ ] `on:` trigger → git hook mapping
- [ ] `github.*` context builder — event_name, ref, sha, actor, repository
- [ ] `env.*`, `runner.*`, `secrets.*` contexts

**GH Actions compat — run: steps**

- [ ] `run:` with `bash` / `sh`
- [ ] `run:` with `pwsh`
- [ ] `env:` at step and job level
- [ ] `if:` conditions on steps and jobs
- [ ] `continue-on-error:`
- [ ] `working-directory:`

**GH Actions compat — expression engine**

- [ ] `${{ }}` evaluation
- [ ] `fromJSON` / `toJSON`
- [ ] string functions — `contains`, `startsWith`, `endsWith`, `format`
- [ ] status functions — `success()`, `failure()`, `always()`, `cancelled()`

**GH Actions compat — jobs**

- [ ] single job execution
- [ ] `needs:` DAG — sequential and parallel
- [ ] matrix expansion — basic
- [ ] `outputs:` between jobs

**Wasm sandbox**

- [ ] Wasmtime engine with Cranelift JIT
- [ ] WASI 3 — native async layer to Wasm
- [ ] one instance per step, killed after completion
- [ ] WASI capability grants — preopened dirs, env vars
- [ ] allowed_hosts network enforcement via hyper proxy
- [ ] secret injection via env — zeroize on drop, never written to disk
- [ ] AOT module cache — `wasmtime::Module::serialize` + SHA-256 pin

**action resolver — JS/TS**

- [ ] fetch action from GitHub (tags, SHAs)
- [ ] compile JS/TS → Wasm via Javy
- [ ] cache compiled `.wasm` with content hash
- [ ] `actions/checkout@v4`
- [ ] `actions/setup-node@v4`
- [ ] `actions/cache@v4`

**sync**

- [ ] `post-checkout` hook — resync on branch switch
- [ ] `post-merge` hook — resync after pull
- [ ] `post-rewrite` hook — resync after rebase/amend
- [ ] atomic shim writes — `write tmpfile → rename()`
- [ ] manifest comment in shim — embedded desired state, no lock file
- [ ] named warning on workflow deletion

**observability**

- [ ] structured step logs via `tracing` crate
- [ ] per-step timing
- [ ] sandbox violation events
- [ ] exit code propagation to git
- [ ] GitHub Annotations format output (`--format=gha`)

---

## v0.2 — compatibility depth `planned`

Close the remaining GitHub Actions surface area.

**GH Actions compat**

- [ ] composite actions — inline step expansion
- [ ] reusable workflows — `uses: ./.github/workflows/shared.yml`
- [ ] `workflow_call` trigger
- [ ] matrix `include` / `exclude`
- [ ] `strategy.fail-fast`
- [ ] `concurrency` groups — cancel-in-progress
- [ ] `services:` containers — Docker shim with warning
- [ ] Docker-based actions — capability-wrapped shim
- [ ] `secrets: inherit`
- [ ] `permissions:` blocks — respected as capability hints
- [ ] `timeout-minutes:` per step and job

**action resolver**

- [ ] `actions/upload-artifact@v4` / `download-artifact`
- [ ] `actions/setup-python`, `setup-go`, `setup-java`
- [ ] local actions — `uses: ./actions/my-action`
- [ ] private repo actions (with token)

**Wasm sandbox**

- [ ] per-step memory limits
- [ ] per-step CPU time limits (Cranelift fuel)
- [ ] network proxy — DNS allowlist + TLS inspection
- [ ] async step execution via WASI 3 — concurrent I/O within a step

**CLI**

- [ ] `wsr list` — show all workflows and their hook mappings
- [ ] `wsr inspect <file>` — parse and pretty-print a workflow, validate expressions
- [ ] `wsr cache` — list, verify, and purge compiled Wasm module cache
- [ ] `wsr hook` — manually install/remove individual hook shims

---

## v0.3 — daemon and dx `planned`

The authoring experience. Fast feedback while writing workflows.

- [ ] `wsr daemon` — persistent file watcher via `notify` crate
- [ ] watches workflow directory for changes between git events
- [ ] auto-resyncs hooks on workflow file edits
- [ ] debounce — 300ms, coalesce rapid saves
- [ ] IPC socket for `wsr status` to query daemon state
- [ ] `wsr daemon install` — register as launchd / systemd service
- [ ] `wsr status` — show active hook map, daemon state, last sync
- [ ] hot reload — recompile changed Wasm modules without restarting

---

## v0.4 — gitlab ci adapter `planned`

First non-GitHub provider. Validates the adapter pattern against a real alternative syntax.

- [ ] `WorkflowProvider` impl for GitLab CI
- [ ] `.gitlab-ci.yml` parser
- [ ] GitLab CI trigger → git hook mapping (`push`, `merge_request`)
- [ ] `CI_*` variable context normalization → internal IR
- [ ] `stage:` and `needs:` DAG execution
- [ ] GitLab-specific expressions and `rules:` evaluation
- [ ] `wsr.json` — `"provider": "gitlab"`

---

## v0.5 — bitbucket pipelines adapter `planned`

- [ ] `WorkflowProvider` impl for Bitbucket Pipelines
- [ ] `bitbucket-pipelines.yml` parser
- [ ] Bitbucket trigger → git hook mapping
- [ ] `BITBUCKET_*` variable context normalization → internal IR
- [ ] `step:` execution model
- [ ] `wsr.json` — `"provider": "bitbucket"`

---

## v0.6 — performance `future`

Once compat is proven across providers, optimize the hot path.

- [ ] AOT compilation on `wsr init` — pre-compile all action Wasm at install time
- [ ] shared Wasmtime `Engine` across steps — amortize JIT cost
- [ ] parallel step execution where DAG allows
- [ ] incremental expression caching — memoize pure `${{ }}` eval
- [ ] Cranelift optimization flags for release builds
- [ ] benchmark suite — compare step startup time vs `act` + Docker

---

## future / unscheduled

- **`wsr run --watch`** — re-run on file save, for TDD-style workflow authoring
- **remote cache** — share compiled `.wasm` artifacts across a team via S3/R2
- **VS Code extension** — inline step results, expression hover evaluation
- **native actions** — compile Rust-based actions directly, skip Javy entirely
- **signed action verification** — verify action SHAs against a trust policy before compilation
- **wsr.json as IR** — use the config file as the normalized interchange format between providers,
  enabling cross-provider workflow translation

---

## non-goals

Things `wsr` will deliberately not do:

- replace remote CI — `wsr` is a local pre-flight, not a CI server
- run Docker-based actions natively — the security boundary is the point
- support act's `--platform` flag — no container runtime dependency, ever
- auto-update workflows — `wsr` reads workflows, never writes them
- lock developers to a single CI provider — the adapter pattern is a first-class design goal

- **signed action verification** — verify action SHAs against a trust policy before compilation
- **wsr.json as IR** — use the config file as the normalized interchange format between providers,
  enabling cross-provider workflow translation

---

## non-goals

Things `wsr` will deliberately not do:

- replace remote CI — `wsr` is a local pre-flight, not a CI server
- run Docker-based actions natively — the security boundary is the point
- support act's `--platform` flag — no container runtime dependency, ever
- auto-update workflows — `wsr` reads workflows, never writes them
- lock developers to a single CI provider — the adapter pattern is a first-class design goal
