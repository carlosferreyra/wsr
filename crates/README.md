# wsr crates

This directory contains all crates that make up the wsr workspace.

---

## The binary

| Crate | Description |
|---|---|
| [`wsr`](wsr/) | Binary entry point. Contains `src/bin/wsr.rs` (thin `main`), `src/lib.rs`, and `src/commands/` (one file per subcommand). Bridges parsed CLI args from `wsr-cli` to the engine, git, and tracing crates. Published to crates.io as `wsr`. |
| [`wsr-cli`](wsr-cli/) | All clap-specific code: `Cli` struct, every subcommand enum, and all argument types. A library — no `main`, no side effects. Lets shell-completion generators and test harnesses import the CLI schema without the full binary dep graph. |

---

## Execution core

| Crate | Description |
|---|---|
| [`wsr-engine`](wsr-engine/) | Job DAG scheduler, step orchestrator, matrix expansion, and `outputs:` propagation. The runtime core — drives every workflow execution from start to final exit code. |
| [`wsr-sandbox`](wsr-sandbox/) | **Tier 1** — Wasmtime + WASI Preview 3. One isolated Wasm instance per step. ~1–3 ms cold starts via Cranelift JIT and AOT module cache. Default execution environment. |
| [`wsr-wasix`](wsr-wasix/) | **Tier 2** — Wasmer + WASIX. POSIX-compatible sandbox for heavy toolchains (`rustc`, LLVM, Go) and GHA Marketplace actions that require `fork`/`exec` or threading. Transitional: workloads migrate to Tier 1 as WASI 3 coverage grows. |
| [`wsr-shell`](wsr-shell/) | Shell step executor for `run:` directives (`bash`, `sh`, `pwsh`). Handles script preparation, `env:` injection, `working-directory:`, and exit code mapping. |

---

## Workflow parsing

| Crate | Description |
|---|---|
| [`wsr-gha`](wsr-gha/) | **GitHub Actions provider** — the reference implementation of `WorkflowProvider`. Parses `.github/workflows/*.yml`, builds `github.*` / `env.*` / `runner.*` context maps, and maps `on:` triggers to git hooks. |
| [`wsr-expr`](wsr-expr/) | Provider-agnostic `${{ }}` expression evaluator. Implements the full GHA expression surface: string/status functions, `fromJSON`/`toJSON`, all standard contexts. Context map is supplied by the active provider adapter. |
| [`wsr-resolver`](wsr-resolver/) | Action reference resolver. Maps `uses: owner/action@ref` to a concrete Wasm component, pins mutable tags to immutable SHAs, and assigns the execution tier (Tier 1 or Tier 2). Never silently falls back to Docker. |

---

## Infrastructure

| Crate | Description |
|---|---|
| [`wsr-git`](wsr-git/) | Git hook management and stateless reconciliation. Generates hook shims, embeds a manifest comment (no lock file), and reconciles desired vs installed hooks via atomic `rename()` writes. Also handles `post-checkout`, `post-merge`, and `post-rewrite` auto-sync. |
| [`wsr-client`](wsr-client/) | HTTP client for fetching actions and Wasm components. Resolves mutable refs to pinned SHAs, queries the `ectorial/*` component registry, and handles retry with exponential backoff. |
| [`wsr-cache`](wsr-cache/) | Content-addressed Wasm module cache keyed by SHA-256 digest. Enables `Module::deserialize` fast-path in Wasmtime, avoiding recompilation. Backs `wsr cache list/verify/purge`. |
| [`wsr-fs`](wsr-fs/) | Filesystem utilities: atomic writes (`tmpfile → rename()`), VFS abstraction for sandbox capability enforcement, workspace root detection, and `.git/` path helpers. |

---

## Shared

| Crate | Description |
|---|---|
| [`wsr-types`](wsr-types/) | Shared types, traits, and errors. Defines `WorkflowProvider`, `WorkflowIR`, `TriggerEvent`, `GitHook`, `ExecutionTier`, and `WsrError`. No internal dependencies — the foundation every other crate builds on. |
| [`wsr-tracing`](wsr-tracing/) | Structured logging and output formatting. Owns subscriber initialization and format selection: **human** (compact, coloured, git-hook-friendly) and **gha** (GitHub Annotations for `::error::` / `::notice::` consumers). |

---

## Development

| Crate | Description |
|---|---|
| [`wsr-bench`](wsr-bench/) | Benchmark suite. Measures step cold start time by tier, cache hit rates, expression evaluator throughput, hook reconcile latency, and comparative benchmarks against `act` + Docker. |

---

## Dependency graph

```
wsr  (src/bin/wsr.rs → src/commands/)
  ├── wsr-cli  (Cli struct, all clap arg types)
  ├── wsr-engine
  │     ├── wsr-expr
  │     ├── wsr-resolver
  │     │     ├── wsr-client
  │     │     ├── wsr-cache ──── wsr-fs
  │     │     └── wsr-gha ────── wsr-expr
  │     ├── wsr-sandbox ──────── wsr-cache
  │     ├── wsr-wasix ─────────── wsr-cache
  │     ├── wsr-shell
  │     └── wsr-types  (all crates depend on this)
  ├── wsr-git ──────────────────── wsr-fs
  └── wsr-tracing
```

## Adding a new provider

1. Create a new crate `crates/wsr-<provider>/` (e.g. `wsr-gitlab`, `wsr-bitbucket`).
2. Add it to `[workspace.members]` in the root `Cargo.toml`.
3. Implement the `WorkflowProvider` trait from `wsr-types`.
4. Add the provider name to the `"provider"` field in `wsr.json`.
5. Wire it into `wsr-engine`'s provider registry.

The engine, sandbox, and expression evaluator are unchanged — only the parser and
context builder differ per provider.
