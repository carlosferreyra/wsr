# sync

Keeps git hook shims in sync with the current branch's workflow files. Runs automatically on branch switches, pulls, and rebases.

## hooks installed

| git hook | trigger |
| --- | --- |
| `post-checkout` | branch switch |
| `post-merge` | `git pull` / `git merge` |
| `post-rewrite` | `git rebase` / `git commit --amend` |

## reconcile model

Sync is stateless. On each invocation:

1. Scan `.github/workflows/` for YAML files
2. Map triggers to hook names via the provider adapter
3. Diff desired state against installed shims (read from embedded manifest comment)
4. Atomically write changed shims: `write tmpfile → rename()`

No lock file. The desired state is embedded as a comment inside each shim script itself.

## shim format

Each installed shim contains a machine-readable manifest comment at the top:

```sh
#!/bin/sh
# wsr-manifest: {"workflows":["ci.yml"],"provider":"github"}
exec wsr run --event push
```

On the next sync, `wsr` reads this comment to determine what was previously installed, diffs it against the current workflow set, and rewrites only what changed.

## warnings

If a workflow file is deleted on the current branch, `wsr` emits a named warning and removes the corresponding hook shim:

```
[wsr] removed pre-push hook (lint.yml deleted on this branch)
```

## source

- [`src/sync/mod.rs`](../../src/sync/mod.rs)
