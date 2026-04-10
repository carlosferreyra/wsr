# sandbox

Wasm-based execution sandbox. Every step runs in its own isolated [Wasmtime](https://wasmtime.dev) instance with explicit capability grants.

## design

- One `Store` + `Instance` per step, dropped immediately after completion
- Cranelift JIT compilation with optional AOT cache
- WASI 3 — native async layer to Wasm
- No Docker. No daemon.

## capability model

Steps get exactly the capabilities they need and nothing more. Capabilities are granted at step instantiation time:

| capability | mechanism |
| --- | --- |
| Filesystem access | WASI preopened directories |
| Environment variables | explicit env grants |
| Network access | `allowed_hosts` enforced via hyper proxy |
| Secrets | injected as env vars, `zeroize`d on drop, never written to disk |

A step that reaches a host outside `allowed_hosts` gets an immediate local error:

```
capability denied: net → registry.example.com
hint: add to sandbox.allowed_hosts in wsr.json
```

## aot module cache

Compiled Wasm modules are cached using `wasmtime::Module::serialize` and pinned with a SHA-256 hash of the source. Subsequent runs skip recompilation.

## source

- [`src/sandbox/mod.rs`](../../src/sandbox/mod.rs)
