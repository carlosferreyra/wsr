# engine

Workflow execution engine. Orchestrates job scheduling, expression evaluation, step dispatch, and result propagation.

## responsibilities

- Build the job DAG from `needs:` declarations
- Expand matrix strategies into concrete job instances
- Evaluate `${{ }}` expressions via the expression engine
- Dispatch each step to the sandbox
- Propagate `outputs:` between jobs
- Collect exit codes and surface failures to git

## expression engine

Evaluates GitHub Actions expression syntax: `${{ <expr> }}`.

**supported contexts**

| context | fields |
| --- | --- |
| `github.*` | `event_name`, `ref`, `sha`, `actor`, `repository` |
| `env.*` | job and step level env vars |
| `runner.*` | `os`, `arch`, `temp`, `tool_cache` |
| `secrets.*` | injected from `secrets_from` file |

**supported functions**

| type | functions |
| --- | --- |
| string | `contains`, `startsWith`, `endsWith`, `format` |
| JSON | `fromJSON`, `toJSON` |
| status | `success()`, `failure()`, `always()`, `cancelled()` |

## job dag

Jobs are sorted topologically by their `needs:` graph. Independent jobs run in parallel; dependent jobs wait for their predecessors to complete and expose their `outputs:`.

## source

- [`src/engine/mod.rs`](../../src/engine/mod.rs)
