# wsr — documentation

Internal architecture docs. Each directory mirrors a module in `src/`.

## modules

| module                         | description                                                       |
| ------------------------------ | ----------------------------------------------------------------- |
| [cli](cli/README.md)           | User-facing commands — `wsr init`, `wsr run`                      |
| [config](config/README.md)     | `wsr.json` parsing and generation                                 |
| [provider](provider/README.md) | Workflow provider adapters (GitHub Actions, GitLab CI, Bitbucket) |
| [engine](engine/README.md)     | Job DAG, expression engine, step orchestration                    |
| [sandbox](sandbox/README.md)   | Wasmtime-based execution sandbox and capability model             |
| [sync](sync/README.md)         | Git hook shim reconciliation                                      |
